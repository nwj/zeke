use crate::fs::{read_dir, read_note};
use anyhow::Result;
use rayon::prelude::*;
use std::collections::HashSet;

pub fn run() -> Result<()> {
    let entries: Vec<_> = read_dir("./").collect();

    let mut tags: Vec<_> = entries
        .par_iter()
        .map(|entry| match read_note(entry.path()) {
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
