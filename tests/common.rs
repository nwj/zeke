use crate::tester::ZekeTester;
use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::*;

mod subcommands;
mod tester;

#[test]
fn without_args() -> Result<()> {
    let t = ZekeTester::new();

    t.zeke()?
        .assert()
        .failure()
        .stderr(predicate::str::contains("zeke"))
        .stderr(predicate::str::contains("USAGE:"))
        .stderr(predicate::str::contains("OPTIONS:"))
        .stderr(predicate::str::contains("SUBCOMMANDS:"));

    Ok(())
}
