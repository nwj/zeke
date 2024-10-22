#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

use anyhow::Result;

mod cli;
mod config;
mod subcommands;

fn main() {
    match run() {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error: {e:#}");
            std::process::exit(1);
        }
    }
}

#[allow(clippy::unnecessary_wraps)]
fn run() -> Result<()> {
    let args = cli::get_cli_arguments();
    let config = config::get_configuration(&args);

    match args.subcommand() {
        Some(("new", _subcommand_args)) => todo!("new subcommand not yet implemented"),
        Some(("ls", _subcommand_args)) => todo!("ls subcommand not yet implemented"),
        Some(("config", subcommand_args)) => subcommands::config::run(subcommand_args, config),
        _ => unreachable!("clap's subcommand_required option prevents this state"),
    }
}
