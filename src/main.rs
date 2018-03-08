#[macro_use] extern crate quicli;
#[macro_use] extern crate serde_derive;
extern crate csv;

use quicli::prelude::*;
use std::fs::File;
use std::process;

/// Get search term for Pathfinder stuff
// This needs to change to support a search term instead

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
        let file_path = "./data.csv";
        if let Err(err) = parse_spells(file_path) {
            println!("{}", err);
            process::exit(1);
        }
    };
});

fn parse_spells(file_path: &str) -> Result<()> {
    let file = open_file(file_path);
    let mut csv_reader = csv::Reader::from_reader(file);
    // This shit is dope.
    // because we used the derive(Deserialize) with spells we can make a spell
    // from this record implicitly
    for result in csv_reader.deserialize() {
        let spell: Spell = result?;
        println!("{:#?}", spell);
    }
    Ok(())
}

/// Open a File at a given Path
fn open_file(path: &str) -> std::fs::File {
    match File::open(path) {
        Err(_why) => panic!("Couldn't open file: {}", path),
        Ok(file) => file,
    }
}
