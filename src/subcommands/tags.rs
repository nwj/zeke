use crate::note::Note;
use anyhow::Result;
use ignore::{overrides::OverrideBuilder, types::TypesBuilder, WalkBuilder};
use rayon::prelude::*;
use std::collections::HashSet;

pub fn run() -> Result<()> {
    let markdown_matcher = TypesBuilder::new()
        .add_defaults()
        .select("markdown")
        .build()?;

    // This override, which ignores hidden entries, is necessary because the markdown matcher
    // itself overrides the WalkBuilder's default filtering of hidden entries.
    let hidden_override = OverrideBuilder::new("./").add("!.*")?.build()?;

    let entries: Vec<_> = WalkBuilder::new("./")
        .types(markdown_matcher)
        .overrides(hidden_override)
        .build()
        .into_iter()
        .filter_map(|r| r.ok())
        .collect();

    let mut tags: Vec<_> = entries
        .par_iter()
        .map(|entry| match Note::read_from_file(entry.path()) {
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
