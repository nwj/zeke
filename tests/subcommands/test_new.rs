use crate::tester::ZekeTester;
use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use chrono::Utc;
use predicates::prelude::*;
use regex::Regex;
use std::error::Error;
use std::str;

#[test]
fn creates_note() -> Result<(), Box<dyn Error>> {
    let t = ZekeTester::new();

    let output = t.zeke_new("Cats dogs")?.output()?;
    let stdout = str::from_utf8(&output.stdout)?;
    let new_path = Regex::new(r"\d{8}-cats_dogs.md")?
        .find(stdout)
        .unwrap()
        .as_str();
    t.temp_dir
        .child(new_path)
        .assert(predicate::path::is_file())
        .assert(predicate::str::contains("---\ntitle: Cats dogs"));

    Ok(())
}

#[test]
fn does_not_overwrite_existing_files() -> Result<(), Box<dyn Error>> {
    let t = ZekeTester::new();
    let title = "foo";
    // This will be a source of test flakiness if the test is run right as the date changes, but meh
    let existing_file_path = format!("{}-{}.md", Utc::now().format("%Y%m%d").to_string(), title);
    let existing_file = t.temp_dir.child(&existing_file_path);
    existing_file.touch()?;

    t.zeke_new(title)?.assert().failure();
    existing_file.assert("");

    Ok(())
}
