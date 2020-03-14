use crate::frontmatter::FrontMatter;
use chrono::Local;
use clap::ArgMatches;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let path = match matches.value_of("PATH") {
        Some(s) => s.to_string(),
        None => format!("{}.md", Local::now().format("%Y%m%d%H%M%S").to_string()),
    };
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)?;
    file.write_all(FrontMatter::default().to_yaml()?.as_bytes())?;
    println!("Created `{}` note file", &path);

    Ok(())
}
