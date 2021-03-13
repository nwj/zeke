use crate::tester::ZekeTester;
use anyhow::Result;
use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use chrono::Utc;
use predicates::prelude::*;
use regex::Regex;
use std::str;

#[test]
fn creates_note() -> Result<()> {
    let t = ZekeTester::new();

    let output = t.zeke_new("Cats dogs")?.output()?;
    let stderr = str::from_utf8(&output.stderr)?;
    let new_path = Regex::new(r"\d{8}-cats_dogs.md")?
        .find(stderr)
        .unwrap()
        .as_str();
    t.temp_dir
        .child(new_path)
        .assert(predicate::path::is_file())
        .assert(predicate::str::contains("---\ntitle: Cats dogs"));

    Ok(())
}

#[test]
fn does_not_overwrite_existing_files() -> Result<()> {
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

#[test]
fn spawns_zeke_editor_process_if_edit_flag() -> Result<()> {
    let t = ZekeTester::new();
    let echo_test = "test";

    let output = t
        .zeke_new("foo")?
        .arg("-e")
        .env_clear()
        .env("ZEKE_EDITOR", format!("echo {}", echo_test))
        .output()?;
    let stdout = str::from_utf8(&output.stdout)?;
    assert_eq!(stdout.matches(echo_test).count(), 1);

    Ok(())
}

#[test]
fn spawns_editor_process_if_edit_flag() -> Result<()> {
    let t = ZekeTester::new();
    let echo_test = "test";

    let output = t
        .zeke_new("foo")?
        .arg("-e")
        .env_clear()
        .env("EDITOR", format!("echo {}", echo_test))
        .output()?;
    let stdout = str::from_utf8(&output.stdout)?;
    assert_eq!(stdout.matches(echo_test).count(), 1);

    Ok(())
}

#[test]
fn prefers_zeke_editor_to_editor_if_edit_flag() -> Result<()> {
    let t = ZekeTester::new();
    let echo_test1 = "test1";
    let echo_test2 = "test2";

    let output = t
        .zeke_new("foo")?
        .arg("-e")
        .env_clear()
        .env("ZEKE_EDITOR", format!("echo {}", echo_test1))
        .env("EDITOR", format!("echo {}", echo_test2))
        .output()?;
    let stdout = str::from_utf8(&output.stdout)?;
    assert_eq!(stdout.matches(echo_test1).count(), 1);

    Ok(())
}

#[test]
fn fails_gracefully_if_edit_flag_but_no_editor() -> Result<()> {
    let t = ZekeTester::new();
    t.zeke_new("bar")?.arg("-e").env_clear().assert().failure();

    Ok(())
}
