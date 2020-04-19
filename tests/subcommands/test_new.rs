use crate::tester::ZekeTester;
use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use regex::Regex;
use std::error::Error;
use std::str;

#[test]
fn creates_note() -> Result<(), Box<dyn Error>> {
    let t = ZekeTester::new();

    let output = t.zeke_new(None)?.output()?;
    let stdout = str::from_utf8(&output.stdout)?;
    let new_path = Regex::new(r"\d{14}.md")?.find(stdout).unwrap().as_str();
    t.temp_dir
        .child(new_path)
        .assert(predicate::path::is_file())
        .assert(predicate::str::contains("---\ntitle:"));

    Ok(())
}

#[test]
fn creates_note_with_given_id() -> Result<(), Box<dyn Error>> {
    let t = ZekeTester::new();
    let new_path = "foo.md";

    t.zeke_new(Some(&new_path))?.assert().success();
    t.temp_dir
        .child(&new_path)
        .assert(predicate::path::is_file())
        .assert(predicate::str::contains("---\ntitle:"));

    Ok(())
}

#[test]
fn does_not_overwrite_existing_files() -> Result<(), Box<dyn Error>> {
    let t = ZekeTester::new();
    let existing_file_path = "foo.md";
    let existing_file = t.temp_dir.child(&existing_file_path);
    existing_file.touch()?;

    t.zeke_new(Some(&existing_file_path))?.assert().failure();
    existing_file.assert("");

    Ok(())
}
