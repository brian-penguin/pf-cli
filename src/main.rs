#[macro_use] extern crate elastic_derive;
#[macro_use] extern crate quicli;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate structopt;
extern crate csv;
extern crate elastic;
extern crate pbr;

use elastic::error::ApiError;
use elastic::prelude::*;
use std::error::Error;
use std::fs::File;
use structopt::StructOpt;
use pbr::ProgressBar;

const SPELL_DATA_FILE: &str = "./data.csv";

/// Get search term for Pathfinder stuff
// TODO: allow for a search term

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "rebuild", short = "r", default_value = "y")]
    rebuild: String,
}

// At some point in the future we will want to switch some of the attrs for Spells into keywords
// (for filtering) - this may require some cleanup of the data to make it filterable (ie Where
// school is X )
//
// Our main model in ES is the Spell
#[derive(Debug, ElasticType, Deserialize, Serialize)]
struct Spell {
    id: i32,
    #[serde(default = "Date::now")]
    timestamp: Date<DefaultDateMapping>,
    name: String,
    area: String,
    casting_time: String,
    components: String,
    costly_components: String,
    description: String,
    descriptor: String,
    dismissable: Option<String>,
    duration: String,
    effect: String,
    material_costs: Option<String>,
    range: String,
    saving_throw: String,
    school: String,
    shapeable: Option<String>,
    short_description: String,
    source: String,
    spell_level: String,
    spell_resistance: Option<String>,
    subschool: String,
    targets: String,
}

fn main() {
    let args = Cli::from_args();
    if &args.rebuild == "y" {
        run_spell_rebuild().unwrap();
    };
}

/// Run the spell rebuild
fn run_spell_rebuild() -> Result<(), Box<Error>> {
    let spell_collection = match parse_spells(SPELL_DATA_FILE) {
        Err(err) => panic!("Can't hang with parsing the spell files{:?}", err),
        Ok(spell_collection) => spell_collection,
    };

    let client = SyncClientBuilder::new().build()?;

    let spell_collection_size = spell_collection.len() as u64;
    let mut pb = ProgressBar::new(spell_collection_size);
    pb.format("╢▌▌░╟");

    for spell in spell_collection.iter() {
        ensure_spell_indexed(&client, spell);
        pb.inc();
    }

    Ok(())
}

fn ensure_spell_indexed(client: &SyncClient, doc: &Spell) -> Result<(), Box<Error>> {
    let get_res = client
        .document_get::<Spell>(spell_index(), id(doc.id))
        .send();

    match get_res.map(|res| res.into_document()) {
        // The doc was found: no need to index
        Ok(Some(doc)) => {
            println!("Document already indexed: {:?}", doc);
        }
        // The index exists, but the doc wasn't found: map and index
        // Mapping ensures the Types are correct in elasticsearch
        Ok(None) => {
            put_spell_doc(client, doc)?;
        }
        // No index: create it, then map and index
        Err(elastic::Error::Api(ApiError::IndexNotFound { .. })) => {
            println!("Creating Index and doc(s)");

            put_spell_index(client)?;
            put_spell_doc(client, doc)?;
        }
        // Something went wrong: panic
        Err(e) => Err(e)?,
    }

    Ok(())
}

fn spell_index() -> Index<'static> {
    Index::from("spells")
}

fn put_spell_index(client: &SyncClient) -> Result<(), Box<Error>> {
    client.index_create(spell_index()).send()?;
    client
        .document_put_mapping::<Spell>(spell_index())
        .send()?;

    Ok(())
}

fn put_spell_doc(client: &SyncClient, doc: &Spell) -> Result<(), Box<Error>> {
    client
        .document_index(spell_index(), id(doc.id), doc)
        .params(|p| p.url_param("refresh", true))
        .send()?;

    Ok(())
}


/// Parse the spells into a Spell Struct
fn parse_spells(file_path: &str) -> Result<Vec<Spell>, Box<Error>> {
    let file = open_file(file_path);
    let mut spell_collection = Vec::new();
    let mut csv_reader = csv::Reader::from_reader(file);
    // This just says read each line in and deserialize it to a spell struct
    for result in csv_reader.deserialize() {
        let spell: Spell = result?;
        spell_collection.push(spell);
    }
    Ok(spell_collection)
}

/// Open a File at a given Path
fn open_file(path: &str) -> std::fs::File {
    match File::open(path) {
        Err(_why) => panic!("Couldn't open file: {}", path),
        Ok(file) => file,
    }
}
