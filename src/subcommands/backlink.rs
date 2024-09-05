use crate::fs::{read_dir, read_note, write_note};
use anyhow::Result;
use path_clean::PathClean;
use rayon::iter::Either;
use rayon::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::PathBuf;

pub fn run() -> Result<i32> {
    let (link_map, mut errs): (Vec<_>, Vec<_>) =
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

    let inverse_link_map: HashMap<&PathBuf, HashSet<&PathBuf>> =
        link_map
            .iter()
            .fold(HashMap::new(), |mut acc, (path, links)| {
                for l in links {
                    acc.entry(l).or_default().insert(path);
                }
                acc
            });

    errs = inverse_link_map
        .par_iter()
        .filter_map(|(path, links)| {
            let mut linked_note = match read_note(path) {
                Ok(n) => n,
                Err(e) => return Some(e),
            };

            let insert_results: Vec<_> = links
                .iter()
                .map(|l| linked_note.front_matter.links.insert(l.clean()))
                .collect();
            if insert_results.iter().any(|&b| b) {
                if let Err(e) = write_note(&linked_note, false) {
                    return Some(e);
                };
            }

            None
        })
        .collect();

    err_count += errs.len();
    for e in errs {
        eprintln!("{:?}", e);
    }

    eprintln!("Backlinking done.");
    Ok(if err_count > 0 { 1 } else { 0 })
}
