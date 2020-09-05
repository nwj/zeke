use crate::note::Note;
use anyhow::Result;
use clap::ArgMatches;
use std::path::PathBuf;

pub fn run(matches: &ArgMatches) -> Result<()> {
    let tag = match matches.value_of("TAG") {
        Some(s) => s.to_string(),
        _ => unreachable!(),
    };

    let paths: Vec<PathBuf> = match matches.values_of_lossy("FILE") {
        Some(v) => v.into_iter().map(|s| PathBuf::from(s)).collect(),
        _ => unreachable!(),
    };

    for path in paths.iter() {
        let mut note = Note::read_from_file(&path)?;

        if note.front_matter.tags.remove(&tag) {
            note.write_to_file(false)?;
        }
    }

    println!(
        "Untagged `{}` from `{}`",
        paths
            .iter()
            .map(|p| p.to_string_lossy().into_owned())
            .collect::<Vec<String>>()
            .join(", "),
        &tag
    );
    Ok(())
}
