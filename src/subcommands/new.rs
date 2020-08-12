use crate::note::Note;
use clap::ArgMatches;
use std::error::Error;
use std::process::Command;
use std::env;

pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
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
