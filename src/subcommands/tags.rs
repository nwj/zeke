use crate::note::Note;
use anyhow::Result;
use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::fs;

pub fn run() -> Result<()> {
    let mut tags: BTreeSet<String> = BTreeSet::new();

    for entry in fs::read_dir(".")? {
        let p = &entry?.path();

        if p.is_dir() {
            continue;
        }

        if p.extension().unwrap_or_default() != OsStr::new("md") {
            continue;
        }

        match Note::read_from_file(&p) {
            Ok(n) => {
                for tag in n.front_matter.tags {
                    tags.insert(tag);
                }
            }
            Err(_) => continue,
        };
    }

    for t in tags {
        println!("{}", t);
    }
    Ok(())
}
