use crate::tester::ZekeTester;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::error::Error;

mod subcommands;
mod tester;

#[test]
fn without_args() -> Result<(), Box<dyn Error>> {
    let t = ZekeTester::new();

    t.zeke()?
        .assert()
        .failure()
        .stderr(predicate::str::contains("zeke"))
        .stderr(predicate::str::contains("USAGE:"))
        .stderr(predicate::str::contains("FLAGS:"))
        .stderr(predicate::str::contains("SUBCOMMANDS:"));

    Ok(())
}
