use crate::fs::write_note;
use crate::note::Note;
use anyhow::{Context, Result};
use clap::ArgMatches;
use std::env;
use std::process::Command;

pub fn run(matches: &ArgMatches) -> Result<i32> {
    let title = match matches.value_of("TITLE") {
        Some(s) => s.to_string(),
        _ => unreachable!(),
    };

    let mut note = Note::new();
    note.front_matter.title = title;
    let path = note.generate_path();
    note.path = Some(path.clone());

    write_note(&note, true)?;
    eprintln!("Created `{}`.", path.to_string_lossy());

    if matches.is_present("edit") {
        let cmd = env::var("ZEKE_EDITOR")
            .with_context(|| "Failed to start editor process. Check that the ZEKE_EDITOR environment variable is set.")?;
        Command::new(&cmd)
            .arg(&path)
            .spawn()
            .with_context(|| format!("Failed to start editor process `{}`.", &cmd))?
            .wait()?;
    }

    Ok(0)
}
