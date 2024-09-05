use crate::note::Note;
use anyhow::{anyhow, Context, Result};
use ignore::{DirEntry, WalkBuilder};
use std::ffi::OsStr;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

pub fn read_dir<P: AsRef<Path>>(path: P) -> impl Iterator<Item = DirEntry> {
    let p = path.as_ref();
    WalkBuilder::new(p)
        .hidden(true)
        .parents(true)
        .ignore(true)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .build()
        .filter_map(|r| r.ok())
        .filter(|en| !is_dir(en) && is_markdown(en))
}

fn is_dir(entry: &DirEntry) -> bool {
    match entry.file_type() {
        Some(ft) => ft.is_dir(),
        None => false,
    }
}

fn is_markdown(entry: &DirEntry) -> bool {
    // There may be a bug here, as I'm unsure what the rules are around case sensitivity
    // and file extensions on various OSes.
    match entry.path().extension() {
        Some(ext) => {
            ext == OsStr::new("md")
                || ext == OsStr::new("markdown")
                || ext == OsStr::new("mdown")
                || ext == OsStr::new("mkdn")
        }
        None => false,
    }
}

pub fn read_note<P: AsRef<Path>>(path: P) -> Result<Note> {
    let p = path.as_ref();

    let file_content = std::fs::read_to_string(p)
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
        .open(path)
        .with_context(|| format!("Failed to open note file `{}`", &path.display()))?;

    file.write_all(
        note.to_string()
            .with_context(|| format!("Failed to serialize note file `{}", &path.display()))?
            .as_bytes(),
    )
    .with_context(|| format!("Failed to write note file `{}`", &path.display()))?;
    Ok(())
}
