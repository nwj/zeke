use crate::fs::write_note;
use crate::note::Note;
use anyhow::{anyhow, Context, Result};
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
        get_editor()
            .with_context(|| "Failed to start editor process. Check that either the EDITOR or ZEKE_EDITOR environment variables are set.")?
            .arg(&path)
            .spawn()
            .with_context(|| "Failed to start editor process.")?
            .wait()?;
    }

    Ok(0)
}

fn get_editor() -> Result<Command> {
    let cmd_unparsed = env::var("ZEKE_EDITOR").or_else(|_| env::var("EDITOR"))?;

    let cmd_parts = shell_words::split(&cmd_unparsed)?;
    match cmd_parts.split_first() {
        Some((bin, args)) => {
            let mut cmd = Command::new(bin);
            cmd.args(args);
            Ok(cmd)
        }
        None => Err(anyhow!("Failed to parse editor command.")),
    }
}
