use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::error::Error;
use std::process::Command;

fn setup(from_path: &str, to_path: &str) -> Result<(Command, assert_fs::TempDir), Box<dyn Error>> {
    let tempdir = assert_fs::TempDir::new()?;
    Command::cargo_bin("zeke")?
        .current_dir(tempdir.path())
        .arg("new")
        .arg(from_path)
        .assert()
        .success();
    Command::cargo_bin("zeke")?
        .current_dir(tempdir.path())
        .arg("new")
        .arg(to_path)
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("zeke")?;
    cmd.arg("link").current_dir(tempdir.path());

    Ok((cmd, tempdir))
}

#[test]
fn links_both_notes() -> Result<(), Box<dyn Error>> {
    let from_path = "a.md";
    let to_path = "b.md";
    let (mut cmd, tempdir) = setup(&from_path, &to_path)?;

    cmd.arg(&from_path).arg(&to_path).assert().success();

    tempdir
        .child(from_path)
        .assert(predicate::str::contains(format!(
            "links_out:\n  - {}\n",
            to_path
        )));
    tempdir
        .child(to_path)
        .assert(predicate::str::contains(format!(
            "links_in:\n  - {}\n",
            from_path
        )));

    Ok(())
}
