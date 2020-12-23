use crate::fs::{read_dir, read_note, write_note};
use anyhow::Result;
use path_clean::PathClean;

pub fn run() -> Result<()> {
    for entry in read_dir("./")?.filter_map(|r| r.ok()) {
        let p = &entry.path();

        let note = match read_note(&p) {
            Ok(n) => n,
            Err(_) => continue,
        };

        for link in note.content.get_note_links().iter() {
            let mut linked_note = match read_note(link) {
                Ok(n) => n,
                Err(_) => continue,
            };

            if linked_note
                .front_matter
                .links
                .insert(note.path.clone().unwrap().clean())
            {
                write_note(&linked_note, false)?;
            }
        }
    }

    Ok(())
}
