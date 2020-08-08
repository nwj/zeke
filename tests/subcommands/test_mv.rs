use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::error::Error;
use std::process::Command;

#[test]
fn not_implemented() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("zeke")?;
    cmd.arg("mv");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Not yet implemented."));
    Ok(())
}
