use crate::note::Note;
use chrono::Utc;
use clap::ArgMatches;
use regex::Regex;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let title = match matches.value_of("TITLE") {
        Some(s) => s.to_string(),
        _ => unreachable!(),
    };

    let mut note = Note::new();
    let path = format!(
        "{}-{}.md",
        Utc::now().format("%Y%m%d").to_string(),
        Regex::new(r"\s")?.replace_all(&title, "_")
    )
    .to_lowercase();
    note.front_matter.title = title;

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)?;
    file.write_all(note.to_string()?.as_bytes())?;
    println!("Created `{}` note file", &path);

    Ok(())
}
