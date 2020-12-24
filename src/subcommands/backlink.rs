use crate::fs::{read_dir, read_note, write_note};
use anyhow::Result;
use path_clean::PathClean;
use rayon::prelude::*;

pub fn run() -> Result<()> {
    let entries: Vec<_> = read_dir("./")?.filter_map(|r| r.ok()).collect();

    let link_map: Vec<(_, _)> = entries
        .par_iter()
        .map(|e| read_note(e.path()))
        .filter_map(|r| r.ok())
        .map(|n| (n.path.clone().unwrap(), n.content.get_note_links()))
        .collect();

    for (path, links) in link_map.iter() {
        for link in links.iter() {
            let mut linked_note = match read_note(link) {
                Ok(n) => n,
                Err(_) => continue,
            };

            if linked_note.front_matter.links.insert(path.clean()) {
                write_note(&linked_note, false)?;
            }
        }
    }

    Ok(())
}
