use crate::note::Note;
use std::collections::BTreeSet;
use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut tags: BTreeSet<String> = BTreeSet::new();

    for entry in fs::read_dir(".")? {
        let path = entry?.path();

        match Note::read_from_file(&path) {
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
