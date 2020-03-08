use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use regex::Regex;
use std::error::Error;
use std::process::Command;
use std::str;

fn setup() -> Result<(Command, assert_fs::TempDir), Box<dyn Error>> {
    let tempdir = assert_fs::TempDir::new()?;
    let mut cmd = Command::cargo_bin("zeke")?;
    cmd.arg("new").current_dir(tempdir.path());

    Ok((cmd, tempdir))
}

#[test]
fn work_in_progress() -> Result<(), Box<dyn Error>> {
    let (mut cmd, _tempdir) = setup()?;

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Not fully implemented yet."));

    Ok(())
}

#[test]
fn creates_note() -> Result<(), Box<dyn Error>> {
    let (mut cmd, tempdir) = setup()?;

    let output = cmd.output()?;
    let stdout = str::from_utf8(&output.stdout)?;
    let new_path = Regex::new(r"\d{14}.md")?.find(stdout).unwrap().as_str();
    tempdir.child(new_path).assert(predicate::path::is_file());
    tempdir
        .child(new_path)
        .assert(predicate::str::contains("---\ntitle:"));

    Ok(())
}

#[test]
fn creates_note_with_given_id() -> Result<(), Box<dyn Error>> {
    let (mut cmd, tempdir) = setup()?;
    let new_path = "foo.md";
    cmd.arg(new_path);

    cmd.assert().success();
    tempdir.child(new_path).assert(predicate::path::is_file());
    tempdir
        .child(new_path)
        .assert(predicate::str::contains("---\ntitle:"));

    Ok(())
}

#[test]
fn does_not_overwrite_existing_files() -> Result<(), Box<dyn Error>> {
    let (mut cmd, tempdir) = setup()?;
    let existing_file_path = "foo.md";
    let existing_file = tempdir.child(existing_file_path);
    existing_file.touch()?;
    cmd.arg(existing_file_path);

    cmd.assert().failure();
    existing_file.assert("");

    Ok(())
}
