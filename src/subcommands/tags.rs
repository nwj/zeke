use crate::fs;
use anyhow::Result;
use rayon::prelude::*;
use std::collections::HashSet;

pub fn run() -> Result<()> {
    let entries: Vec<_> = fs::read_dir("./")?
        .filter_map(|r| r.ok())
        .collect();

    let mut tags: Vec<_> = entries
        .par_iter()
        .map(|entry| match fs::read_note(entry.path()) {
            Ok(n) => n.front_matter.tags,
            Err(_) => HashSet::new(),
        })
        .flatten()
        .collect();

    tags.par_sort();
    tags.dedup();

    for t in tags {
        println!("{}", t);
    }
    Ok(())
}
