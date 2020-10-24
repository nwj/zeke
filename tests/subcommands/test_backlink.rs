use crate::tester::ZekeTester;
use anyhow::Result;
use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;

#[test]
fn links_notes_referenced_by_other_notes() -> Result<()> {
    let t = ZekeTester::new();
    t.setup_fs("subcommands/test_backlink/links_notes_referenced_by_other_notes")?;

    t.zeke_backlink()?.assert().success();

    t.temp_dir
        .child("20201024-b.md")
        .assert(predicate::str::contains("20201024-a.md"));
    t.temp_dir
        .child("20201024-c.md")
        .assert(predicate::str::contains("20201024-a.md"));
    t.temp_dir
        .child("20201024-c.md")
        .assert(predicate::str::contains("20201024-b.md"));

    Ok(())
}

#[test]
fn does_not_panic_on_directories() -> Result<()> {
    let t = ZekeTester::new();
    t.temp_dir.child("subdir").create_dir_all()?;
    t.zeke_backlink()?.assert().success();
    Ok(())
}

#[test]
fn idempotent() -> Result<()> {
    let t = ZekeTester::new();
    t.setup_fs("subcommands/test_backlink/idempotent")?;

    t.zeke_backlink()?.assert().success();
    t.zeke_backlink()?.assert().success();
    t.temp_dir
        .child("20201024-b.md")
        .assert(predicate::str::contains("20201024-a.md").count(1));

    Ok(())
}
