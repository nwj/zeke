extern crate assert_cmd;
extern crate assert_fs;
extern crate predicates;
extern crate regex;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::error::Error;
use std::process::Command;

mod subcommands;

#[test]
fn without_args() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("zeke")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("zeke"))
        .stderr(predicate::str::contains("USAGE:"))
        .stderr(predicate::str::contains("FLAGS:"))
        .stderr(predicate::str::contains("SUBCOMMANDS:"));
    Ok(())
}

