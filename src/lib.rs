use anyhow::Result;
use clap::ArgMatches;

mod content;
mod front_matter;
mod fs;
mod note;
mod subcommands;

pub fn run(matches: &ArgMatches) -> Result<i32> {
    match matches.subcommand() {
        ("new", Some(m)) => subcommands::new::run(m),
        ("link", Some(m)) => subcommands::link::run(m),
        ("unlink", Some(m)) => subcommands::unlink::run(m),
        ("tags", Some(_)) => subcommands::tags::run(),
        ("tag", Some(m)) => subcommands::tag::run(m),
        ("untag", Some(m)) => subcommands::untag::run(m),
        ("mv", Some(m)) => subcommands::mv::run(m),
        ("backlink", Some(_)) => subcommands::backlink::run(),
        ("graph", Some(_)) => subcommands::graph::run(),
        _ => Ok(0),
    }
}
