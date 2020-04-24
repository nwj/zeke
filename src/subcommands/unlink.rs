use crate::note::Note;
use clap::ArgMatches;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let from = match matches.value_of("FROM") {
        Some(s) => s.to_string(),
        _ => unreachable!(),
    };

    let to = match matches.value_of("TO") {
        Some(s) => s.to_string(),
        _ => unreachable!(),
    };

    let mut from_file_in = OpenOptions::new()
        .read(true)
        .create_new(false)
        .open(&from)?;

    let mut to_file_in = OpenOptions::new().read(true).create_new(false).open(&to)?;

    let mut from_contents = String::new();
    from_file_in.read_to_string(&mut from_contents)?;

    let mut to_contents = String::new();
    to_file_in.read_to_string(&mut to_contents)?;

    let mut from_note = Note::from_string(from_contents)?;
    let mut to_note = Note::from_string(to_contents)?;

    if from_note.front_matter.links_out.contains(&to) {
        from_note.front_matter.links_out.remove(&to);
        let mut from_file_out = OpenOptions::new()
            .write(true)
            .create_new(false)
            .truncate(true)
            .open(&from)?;
        from_file_out.write_all(from_note.to_string()?.as_bytes())?;
    }

    if to_note.front_matter.links_in.contains(&from) {
        to_note.front_matter.links_in.remove(&from);
        let mut to_file_out = OpenOptions::new()
            .write(true)
            .create_new(false)
            .truncate(true)
            .open(&to)?;
        to_file_out.write_all(to_note.to_string()?.as_bytes())?;
    }

    println!("Linked `{}` to `{}`", &from, &to);
    Ok(())
}
