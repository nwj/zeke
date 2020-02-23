#[macro_use]
extern crate clap;

use clap::{App, SubCommand};
use std::process;

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .subcommand(SubCommand::with_name("new").about("Create a new note"))
        .get_matches();

    if let Err(e) = zeke::run(matches) {
        eprintln!("[{} error] {}", crate_name!(), e);
        process::exit(1);
    }
}
