use crate::note::Note;
use anyhow::{anyhow, Context, Result};
use ignore::{overrides::OverrideBuilder, types::TypesBuilder, Walk, WalkBuilder};
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

pub fn read_dir<P: AsRef<Path>>(path: P) -> Result<Walk> {
    let p = path.as_ref();

    let markdown_matcher = TypesBuilder::new()
        .add_defaults()
        .select("markdown")
        .build()?;

    // This override, which ignores hidden entries, is necessary because the markdown matcher
    // itself overrides the WalkBuilder's default filtering of hidden entries.
    let hidden_override = OverrideBuilder::new(&p).add("!.*")?.build()?;

    Ok(WalkBuilder::new(&p)
        .types(markdown_matcher)
        .overrides(hidden_override)
        .build())
}

pub fn read_note<P: AsRef<Path>>(path: P) -> Result<Note> {
    let p = path.as_ref();

    let file_content = std::fs::read_to_string(&p)
        .with_context(|| format!("Failed to read note file `{}`", &p.display()))?;

    let (front_matter, content) = Note::from_string(file_content)
        .with_context(|| format!("Failed to deserialize note file `{}`", &p.display()))?;

    Ok(Note {
        front_matter,
        content,
        path: Some(p.to_path_buf()),
    })
}

pub fn write_note(note: &Note, create_new: bool) -> Result<()> {
    let path = note
        .path
        .as_ref()
        .ok_or_else(|| anyhow!("Attempted to write a note that does not have a path"))?;

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(create_new)
        .truncate(true)
        .open(&path)
        .with_context(|| format!("Failed to open note file `{}`", &path.display()))?;

    file.write_all(
        note.to_string()
            .with_context(|| format!("Failed to serialize note file `{}", &path.display()))?
            .as_bytes(),
    )
    .with_context(|| format!("Failed to write note file `{}`", &path.display()))?;
    Ok(())
}
