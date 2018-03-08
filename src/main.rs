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
    reload: String,
}

#[derive(Debug)]
struct Spell {
    name: String,


}

main!(|args: Cli| {
    if &args.reload == "y" {
        run_reload;
    };
});

/// Reload the Elastic Search Index
fn run_reload() {
    let file_name = "./data.csv";
    let spells = read_file(file_name);
    println!("{:?}", spells);

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
