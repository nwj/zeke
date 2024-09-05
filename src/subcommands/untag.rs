use crate::fs::{read_note, write_note};
use anyhow::Result;
use std::path::PathBuf;

pub fn run(tag: &str, paths: &[PathBuf]) -> Result<i32> {
    for path in paths.iter() {
        let mut note = read_note(path)?;

        if note.front_matter.tags.remove(tag) {
            write_note(&note, false)?;
        }
    }

    eprintln!(
        "Untagged `{}` from `{}`.",
        paths
            .iter()
            .map(|p| p.to_string_lossy().into_owned())
            .collect::<Vec<String>>()
            .join(", "),
        &tag
    );
    Ok(0)
}
