use crate::fs::{read_note, write_note};
use anyhow::Result;
use path_clean::PathClean;
use std::path::PathBuf;

pub fn run(path_a: &PathBuf, path_b: &PathBuf) -> Result<i32> {
    let cleaned_path_a = path_a.clean();
    let cleaned_path_b = path_b.clean();

    let mut note_a = read_note(&cleaned_path_a)?;
    let mut note_b = read_note(&cleaned_path_b)?;

    if note_a.front_matter.links.remove(&cleaned_path_b) {
        write_note(&note_a, false)?;
    }

    if note_b.front_matter.links.remove(&cleaned_path_a) {
        write_note(&note_b, false)?;
    }

    eprintln!(
        "Unlinked `{}` from `{}`.",
        cleaned_path_a.display(),
        cleaned_path_b.display()
    );
    Ok(0)
}
