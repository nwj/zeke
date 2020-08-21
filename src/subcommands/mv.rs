use crate::note::Note;
use clap::ArgMatches;
use path_clean::PathClean;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let old_path = match matches.value_of("FILE") {
        Some(s) => PathBuf::from(s).clean(),
        _ => unreachable!(),
    };

    let new_title = match matches.value_of("TITLE") {
        Some(s) => s.to_string(),
        _ => unreachable!(),
    };

    let mut note = Note::read_from_file(&old_path)?;
    note.front_matter.title = new_title;
    let new_path = note.generate_path()?;
    note.path = Some(new_path.clone());
    note.write_to_file(true)?;

    for entry in fs::read_dir(".")? {
        let p = entry?.path();
        let mut n = Note::read_from_file(&p)?;
        if n.front_matter.links.remove(&old_path) {
            n.front_matter.links.insert(new_path.clone());
            n.write_to_file(false)?;
        }
    }

    fs::remove_file(&old_path)?;

    println!("Moved `{}` to `{}`", old_path.display(), new_path.display());
    Ok(())
}
