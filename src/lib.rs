use clap::ArgMatches;
use std::error::Error;

mod front_matter;
mod note;
mod subcommands;

pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    match matches.subcommand() {
        ("new", Some(m)) => subcommands::new::run(m),
        ("link", Some(m)) => subcommands::link::run(m),
        ("unlink", Some(m)) => subcommands::unlink::run(m),
        ("tags", Some(_)) => subcommands::tags::run(),
        ("tag", Some(m)) => subcommands::tag::run(m),
        ("untag", Some(m)) => subcommands::untag::run(m),
        ("list", Some(m)) => subcommands::list::run(m),
        ("backlink", Some(m)) => subcommands::backlink::run(m),
        _ => Ok(()),
    }
}
