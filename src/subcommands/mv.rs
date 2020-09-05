use crate::note::Note;
use anyhow::{Context, Result};
use clap::ArgMatches;
use path_clean::PathClean;
use std::fs;
use std::path::PathBuf;

pub fn run(matches: &ArgMatches) -> Result<()> {
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
    let new_path = note.generate_path();
    note.path = Some(new_path.clone());
    note.write_to_file(true)?;

    for entry in fs::read_dir(".")? {
        let p = entry?.path();
        let mut n = Note::read_from_file(&p)?;
        let mut should_write = false;

        if n.front_matter.links.remove(&old_path) {
            n.front_matter.links.insert(new_path.clone());
            should_write = true;
        }

        let new_content = n
            .content
            .replace_note_links(&old_path, &new_path)
            .with_context(|| {
                format!(
                    "Failed to update reference to target note in note file `{}`",
                    &p.display()
                )
            })?;
        if new_content != n.content {
            n.content = new_content;
            should_write = true;
        }

        if should_write {
            n.write_to_file(false)?;
        }
    }

    fs::remove_file(&old_path)
        .with_context(|| format!("Failed to remove old note file `{}`", &old_path.display()))?;

    println!("Moved `{}` to `{}`", old_path.display(), new_path.display());
    Ok(())
}
