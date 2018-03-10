#[macro_use] extern crate quicli;
#[macro_use] extern crate serde_derive;
extern crate csv;

use quicli::prelude::*;
use std::fs::File;

const SPELL_DATA_FILE: &str = "./data.csv";

/// Get search term for Pathfinder stuff
// TODO: allow for a search term
#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "rebuild", short = "r", default_value = "y")]
    rebuild: String,
}

#[derive(Debug, Deserialize)]
struct Spell {
    name: String,
    description: String,
    school: String,
    subschool: String,
    descriptor: String,
    spell_level: String,
    casting_time: String,
    components: String,
    costly_components: String,
    range: String,
    area: String,
    effect: String,
    targets: String,
    duration: String,
    dismissable: Option<String>,
    shapeable: Option<String>,
    saving_throw: String,
    spell_resistance: Option<String>,
    source: String,
    short_description: String,
    id: String,
    material_costs: Option<String>,
}

main!(|args: Cli| {
    if &args.rebuild == "y" {
        let collection = match parse_spells(SPELL_DATA_FILE) {
            Err(err) => panic!("Can't hang with parsing the spell files{:?}", err),
            Ok(spell_collection) => spell_collection,
        };

        println!("{:?}", collection)
    };
});


/// Parse the spells into a Spell Struct
fn parse_spells(file_path: &str) -> Result<Vec<Spell>> {
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
