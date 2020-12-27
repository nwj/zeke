use crate::fs::{read_dir, read_note, write_note};
use anyhow::Result;
use path_clean::PathClean;
use rayon::iter::Either;
use rayon::prelude::*;

pub fn run() -> Result<i32> {
    let (link_map, errs): (Vec<_>, Vec<_>) =
        read_dir("./")
            .par_bridge()
            .partition_map(|en| match read_note(en.path()) {
                Ok(n) => Either::Left((n.path.clone().unwrap(), n.content.get_note_links())),
                Err(e) => Either::Right(e),
            });

    let mut err_count = errs.len();
    for e in errs {
        eprintln!("{:?}", e);
    }

    for (path, links) in link_map {
        for link in links {
            let mut linked_note = match read_note(link) {
                Ok(n) => n,
                Err(e) => {
                    eprintln!("{:?}", e);
                    err_count += 1;
                    continue;
                }
            };

            if linked_note.front_matter.links.insert(path.clean()) {
                if let Err(e) = write_note(&linked_note, false) {
                    eprintln!("{:?}", e);
                    err_count += 1;
                };
            }
        }
    }

    eprintln!("Backlinking done.");
    Ok(if err_count > 0 {1} else {0})
}
