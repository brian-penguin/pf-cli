#[macro_use] extern crate quicli;
#[macro_use] extern crate serde_derive;
extern crate csv;

use quicli::prelude::*;
use std::fs::File;
use std::io::Read;

/// Get search term for Pathfinder stuff
// This needs to change to support a search term instead
#[derive(Debug, StructOpt)]
struct Cli {
    // Add a CLI argument `--spell`/-s` that defaults to none, and has this help text:
    /// How many lines to get
    #[structopt(long = "spell", short = "s", default_value = "")]
    spell: String,
    #[structopt(long = "rebuild", short = "r", default_value = "y")]
    rebuild: String,
}

#[derive(Debug)]
struct Spell {
    name: String,
    description: String,
}

main!(|args: Cli| {
    if &args.rebuild == "y" {
        // run_reload;
        let file_path = "./data.csv";
        parse_spells(file_path);
    };
});

/// Reload the Elastic Search Index
fn run_reload(file_path: &str) {
    let spells = read_file(file_path);
    println!("{:?}", spells);
    parse_spells(file_path);
}

fn parse_spells(file_path: &str) -> Result<()> {
    let file = open_file("./data.csv");
    let mut csv_reader = csv::Reader::from_reader(file);
    for result in csv_reader.records() {
        let record = result?;

        // Todo: Add more of these guys and fill out the struct
        let name = &record[0];
        let description = &record[16];

        println!("{:?} -> {:?}", name, description)
    }
    Ok(())
}

/// Open a File at a given Path
fn open_file(path: &str) -> std::fs::File {
    match File::open(path) {
        Err(_why) => panic!("Couldn't open file"),
        Ok(file) => file,
    }
}

/// Read a file at a given path
fn read_file(path: &str) -> Result<String> {
    let mut result = String::new();
    let mut file = open_file(path);
    file.read_to_string(&mut result)?;
    Ok(result)
}
