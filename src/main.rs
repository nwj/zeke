#![deny(clippy::all)]
#![deny(clippy::pedantic)]

use anyhow::Result;

mod cli;

fn main() {
    match run() {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error: {e:#}");
            std::process::exit(1);
        }
    }
}

fn run() -> Result<()> {
    let cli_arguments = cli::get_cli_arguments();

    match cli_arguments.subcommand() {
        Some(("new", _subcommand_args)) => todo!("new subcommand not yet implemented"),
        Some(("ls", _subcommand_args)) => todo!("ls subcommand not yet implemented"),
        Some(("config", _subcommand_args)) => todo!("config subcommand not yet implemented"),
        _ => unreachable!("clap's subcommand_required option prevents this state"),
    }
}
