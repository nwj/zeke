use crate::note::Note;
use anyhow::Result;
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
    let path = note.generate_path()?;
    note.path = Some(path.clone());

    note.write_to_file(true)?;
    println!("Created `{}` note file", path.to_string_lossy());

    if matches.is_present("edit") {
        let cmd = env::var("ZEKE_EDITOR")?;
        Command::new(cmd).arg(&path).spawn()?.wait()?;
    }

    Ok(())
}
