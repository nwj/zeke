use crate::fs::{read_dir, read_note, write_note};
use anyhow::{Context, Result};
use path_clean::PathClean;
use rayon::prelude::*;
use std::fs::remove_file;
use std::path::PathBuf;

pub fn run(path: &PathBuf, title: &str) -> Result<i32> {
    let old_path = path.clean();

    let mut note = read_note(&old_path)?;
    note.front_matter.title = title.to_string();
    let new_path = note.generate_path();
    note.path = Some(new_path.clone());
    write_note(&note, true)?;

    let errs: Vec<_> = read_dir("./")
        .par_bridge()
        .filter_map(|en| {
            let mut n = match read_note(en.path()) {
                Ok(n) => n,
                Err(e) => return Some(e),
            };

            let mut should_write = false;
            if n.front_matter.links.remove(&old_path) {
                n.front_matter.links.insert(new_path.clone());
                should_write = true;
            }
            let new_content = match n
                .content
                .replace_note_links(&old_path, &new_path)
                .with_context(|| {
                    format!(
                        "Failed to update possible references to target note in note file `{}`.",
                        n.path.clone().unwrap().display()
                    )
                }) {
                Ok(c) => c,
                Err(e) => return Some(e),
            };

            if new_content != n.content {
                n.content = new_content;
                should_write = true;
            }

            if should_write {
                if let Err(e) = write_note(&n, false).with_context(|| {
                    format!(
                        "Failed to update possible references to target note in note file `{}`.",
                        n.path.clone().unwrap().display()
                    )
                }) {
                    return Some(e);
                };
            }

            None
        })
        .collect();

    let err_count = errs.len();
    for e in errs {
        eprintln!("{e:?}");
    }

    remove_file(&old_path)
        .with_context(|| format!("Failed to remove old note file `{}`.", &old_path.display()))?;

    eprintln!(
        "Moved `{}` to `{}`.",
        old_path.display(),
        new_path.display()
    );
    Ok(i32::from(err_count > 0))
}
