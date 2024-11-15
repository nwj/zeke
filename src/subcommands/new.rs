use crate::config;
use anyhow::{Context, Result};
use clap::ArgMatches;
use std::fs;

#[allow(clippy::unnecessary_wraps)]
pub fn run(_args: &ArgMatches, _config: &config::Config) -> Result<()> {
    log::debug!("Running the 'new' subcommand");

    fs::File::create_new("note.md").context("Failed to create new note")?;

    Ok(())
}
