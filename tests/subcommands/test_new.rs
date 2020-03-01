use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use regex::Regex;
use std::error::Error;
use std::process::Command;
use std::str;

#[test]
fn work_in_progress() -> Result<(), Box<dyn Error>> {
    let tempdir = assert_fs::TempDir::new()?;
    let mut cmd = Command::cargo_bin("zeke")?;
    cmd.arg("new").current_dir(tempdir.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Not fully implemented yet."));

    Ok(())
}

#[test]
fn creates_a_note() -> Result<(), Box<dyn Error>> {
    let tempdir = assert_fs::TempDir::new()?;
    let mut cmd = Command::cargo_bin("zeke")?;
    cmd.arg("new").current_dir(tempdir.path());

    let output = cmd.output()?;
    let stdout = str::from_utf8(&output.stdout)?;
    let new_file_name = Regex::new(r"\d{14}.md")?.find(stdout).unwrap().as_str();

    tempdir
        .child(new_file_name)
        .assert(predicate::path::is_file());
    tempdir.child(new_file_name).assert("Hello world");

    Ok(())
}
