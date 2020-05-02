use crate::note::Note;
use clap::ArgMatches;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let path = match matches.value_of("FILE") {
        Some(s) => s.to_string(),
        _ => unreachable!(),
    };

    let tag = match matches.value_of("TAG") {
        Some(s) => s.to_string(),
        _ => unreachable!(),
    };

    let mut file_in = OpenOptions::new()
        .read(true)
        .create_new(false)
        .open(&path)?;

    let mut file_contents = String::new();
    file_in.read_to_string(&mut file_contents)?;

    let mut note = Note::from_string(file_contents)?;

    if note.front_matter.tags.remove(&tag) {
        let mut file_out = OpenOptions::new()
            .write(true)
            .create_new(false)
            .truncate(true)
            .open(&path)?;

        file_out.write_all(note.to_string()?.as_bytes())?;
    }

    println!("Untagged `{}` from `{}`", &path, &tag);
    Ok(())
}
