use crate::note::Note;
use anyhow::Result;
use clap::ArgMatches;
use path_clean::PathClean;
use std::path::PathBuf;

pub fn run(matches: &ArgMatches) -> Result<()> {
    let path_a = match matches.value_of("FILE_A") {
        Some(s) => PathBuf::from(s).clean(),
        _ => unreachable!(),
    };

    let path_b = match matches.value_of("FILE_B") {
        Some(s) => PathBuf::from(s).clean(),
        _ => unreachable!(),
    };

    let mut note_a = Note::read_from_file(&path_a)?;
    let mut note_b = Note::read_from_file(&path_b)?;

    if note_a.front_matter.links.insert(path_b.clone()) {
        note_a.write_to_file(false)?;
    }

    if note_b.front_matter.links.insert(path_a.clone()) {
        note_b.write_to_file(false)?;
    }

    println!("Linked `{}` to `{}`", path_a.display(), path_b.display());
    Ok(())
}
