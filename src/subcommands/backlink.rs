use crate::note::Note;
use anyhow::Result;
use path_clean::PathClean;
use std::ffi::OsStr;
use std::fs;

pub fn run() -> Result<()> {
    for entry in fs::read_dir(".")? {
        let p = &entry?.path();

        if p.is_dir() {
            continue;
        }

        if p.extension().unwrap_or_default() != OsStr::new("md") {
            continue;
        }

        let note = match Note::read_from_file(&p) {
            Ok(n) => n,
            Err(_) => continue,
        };

        for link in note.content.get_note_links().iter() {
            let mut linked_note = match Note::read_from_file(link) {
                Ok(n) => n,
                Err(_) => continue,
            };

            if linked_note
                .front_matter
                .links
                .insert(note.path.clone().unwrap().clean())
            {
                linked_note.write_to_file(false)?;
            }
        }
    }

    Ok(())
}
