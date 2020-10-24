use crate::fs::{read_dir, read_note, write_note};
use anyhow::{Context, Result};
use clap::ArgMatches;
use path_clean::PathClean;
use rayon::prelude::*;
use std::fs::remove_file;
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

    let mut note = read_note(&old_path)?;
    note.front_matter.title = new_title;
    let new_path = note.generate_path();
    note.path = Some(new_path.clone());
    write_note(&note, true)?;

    let entries: Vec<_> = read_dir("./")?.filter_map(|r| r.ok()).collect();

    let errs: Vec<_> = entries
        .par_iter()
        .map(|e| read_note(e.path()))
        .filter_map(|r| r.ok())
        .map(|mut n| -> Result<()> {
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
                        "Failed to update possible references to target note in note file `{}`",
                        n.path.clone().unwrap().display()
                    )
                })?;
            if new_content != n.content {
                n.content = new_content;
                should_write = true;
            }

            if should_write {
                write_note(&n, false).with_context(|| {
                    format!(
                        "Failed to update possible references to target note in note file `{}`",
                        n.path.clone().unwrap().display()
                    )
                })?;
            }

            Ok(())
        })
        .filter_map(|r| r.err())
        .collect();

    for e in errs {
        eprintln!("{:?}", e);
    }

    remove_file(&old_path)
        .with_context(|| format!("Failed to remove old note file `{}`", &old_path.display()))?;

    println!("Moved `{}` to `{}`", old_path.display(), new_path.display());
    Ok(())
}
