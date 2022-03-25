use crate::fs::write_note;
use crate::note::Note;
use anyhow::{anyhow, Context, Result};
use std::env;
use std::process::Command;

pub fn run(title: &str, edit_flag: &bool) -> Result<i32> {
    let mut note = Note::new();
    note.front_matter.title = title.to_string();
    let path = note.generate_path();
    note.path = Some(path.clone());

    write_note(&note, true)?;
    eprintln!("Created `{}`.", path.to_string_lossy());

    if *edit_flag {
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
