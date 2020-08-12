use crate::note::Note;
use clap::ArgMatches;
use std::error::Error;
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

    let mut note = Note::read_from_file(&path)?;

    if note.front_matter.tags.remove(&tag) {
        note.write_to_file(false)?;
    }

    println!("Untagged `{}` from `{}`", path.display(), &tag);
    Ok(())
}
