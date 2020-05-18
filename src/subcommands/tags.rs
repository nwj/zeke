use crate::note::Note;
use std::collections::BTreeSet;
use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut tags: BTreeSet<String> = BTreeSet::new();

    for entry in fs::read_dir(".")? {
        let path = entry?.path();

        if path.is_dir() {
            continue;
        }

        let mut file = OpenOptions::new()
            .read(true)
            .create_new(false)
            .open(&path)?;
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)?;

        match Note::from_string(file_contents) {
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
