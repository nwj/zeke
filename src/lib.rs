use clap::ArgMatches;
use std::error::Error;

pub fn run(matches: ArgMatches) -> Result<(), Box<dyn Error>> {
    match matches.subcommand() {
        ("new", Some(m)) => run_new(m),
        _ => Ok(()),
    }
}

fn run_new(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    println!("To be implemented.");
    Ok(())
}
