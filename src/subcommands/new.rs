use crate::fs::write_note;
use crate::note::Note;
use anyhow::{Context, Result};
use clap::ArgMatches;
use std::env;
use std::process::Command;

pub fn run(matches: &ArgMatches) -> Result<()> {
    let title = match matches.value_of("TITLE") {
        Some(s) => s.to_string(),
        _ => unreachable!(),
    };

    let mut note = Note::new();
    note.front_matter.title = title;
    let path = note.generate_path();
    note.path = Some(path.clone());

    write_note(&note, true)?;
    println!("Created `{}` note file", path.to_string_lossy());

    if matches.is_present("edit") {
        let cmd = env::var("ZEKE_EDITOR")
            .with_context(|| "Failed attempting to get the ZEKE_EDITOR env variable")?;
        Command::new(&cmd)
            .arg(&path)
            .spawn()
            .with_context(|| format!("Failed to spawn editor process `{}`", &cmd))?
            .wait()?;
    }

    Ok(())
}
