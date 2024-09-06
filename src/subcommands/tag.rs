use crate::fs::{read_note, write_note};
use anyhow::Result;
use std::path::PathBuf;

pub fn run(tag: &str, paths: &[PathBuf]) -> Result<i32> {
    for path in paths {
        let mut note = read_note(path)?;

        if note.front_matter.tags.insert(tag.to_string()) {
            write_note(&note, false)?;
        }
    }

    eprintln!(
        "Tagged `{}` with `{}`.",
        paths
            .iter()
            .map(|p| p.to_string_lossy().into_owned())
            .collect::<Vec<String>>()
            .join(", "),
        &tag
    );
    Ok(0)
}
