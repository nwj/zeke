use crate::note::Note;
use clap::ArgMatches;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::PathBuf;

pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let path_a = match matches.value_of("FILE_A") {
        Some(s) => PathBuf::from(s),
        _ => unreachable!(),
    };

    let path_b = match matches.value_of("FILE_B") {
        Some(s) => PathBuf::from(s),
        _ => unreachable!(),
    };

    let mut note_a = Note::from_file(&path_a)?;
    let mut note_b = Note::from_file(&path_b)?;

    if note_a.front_matter.links.remove(&path_b) {
        let mut file_a_out = OpenOptions::new()
            .write(true)
            .create_new(false)
            .truncate(true)
            .open(&path_a)?;
        file_a_out.write_all(note_a.to_string()?.as_bytes())?;
    }

    if note_b.front_matter.links.remove(&path_a) {
        let mut file_b_out = OpenOptions::new()
            .write(true)
            .create_new(false)
            .truncate(true)
            .open(&path_b)?;
        file_b_out.write_all(note_b.to_string()?.as_bytes())?;
    }

    println!(
        "Unlinked `{}` from `{}`",
        path_a.display(),
        path_b.display()
    );
    Ok(())
}
