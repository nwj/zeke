#[macro_use]
extern crate clap;

use clap::{App, SubCommand};

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .subcommand(SubCommand::with_name("new").about("Create a new note"))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("new") {
        println!("To be implemented");
    }
}
