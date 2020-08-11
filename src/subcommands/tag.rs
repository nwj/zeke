use crate::note::Note;
use clap::ArgMatches;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::PathBuf;

pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let path = match matches.value_of("FILE") {
        Some(s) => PathBuf::from(s),
        _ => unreachable!(),
    };

    let tag = match matches.value_of("TAG") {
        Some(s) => s.to_string(),
        _ => unreachable!(),
    };

    let mut note = Note::from_file(&path)?;

    if note.front_matter.tags.insert(tag.clone()) {
        let mut file_out = OpenOptions::new()
            .write(true)
            .create_new(false)
            .truncate(true)
            .open(&path)?;

        file_out.write_all(note.to_string()?.as_bytes())?;
    }

    println!("Tagged `{}` with `{}`", path.display(), &tag);
    Ok(())
}
