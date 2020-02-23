#[macro_use]
extern crate clap;

use clap::{App, SubCommand};

fn main() {
    let matches = App::new("zeke")
        .version(crate_version!())
        .about("A tool for managing notes according to the 'Zettelkasten' system")
        .subcommand(SubCommand::with_name("new").about("Create a new note"))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("new") {
        println!("To be implemented");
    }
}
