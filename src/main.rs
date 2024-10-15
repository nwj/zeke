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
    let _cli_arguments = cli::get_cli_arguments();

    Ok(())
}
