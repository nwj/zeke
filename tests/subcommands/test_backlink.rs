use crate::tester::ZekeTester;
use anyhow::Result;
use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::fs;

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

#[test]
fn does_not_modify_other_parts_of_linked_notes() -> Result<()> {
    let t = ZekeTester::new();
    t.setup_fs("subcommands/test_backlink/does_not_modify_other_parts_of_linked_notes")?;
    let initial_content = fs::read_to_string(t.temp_dir.child("20201024-b.md").path())?;

    t.zeke_backlink()?.assert().success();
    t.temp_dir
        .child("20201024-b.md")
        .assert(initial_content.replace("links: []", "links:\n- 20201024-a.md"));

    Ok(())
}

#[test]
fn only_finds_links_in_markdown_files() -> Result<()> {
    let t = ZekeTester::new();
    t.setup_fs("subcommands/test_backlink/only_finds_links_in_markdown_files")?;
    let initial_content = fs::read_to_string(t.temp_dir.child("20201024-b.md").path())?;

    t.zeke_backlink()?.assert().success();
    t.temp_dir
        .child("20201024-b.md")
        .assert(predicate::str::contains(initial_content));

    Ok(())
}

#[test]
fn does_not_find_links_in_hidden_files() -> Result<()> {
    let t = ZekeTester::new();
    t.setup_fs("subcommands/test_backlink/does_not_find_links_in_hidden_files")?;
    let initial_content = fs::read_to_string(t.temp_dir.child("20201223-b.md").path())?;

    t.zeke_backlink()?.assert().success();
    t.temp_dir
        .child("20201223-b.md")
        .assert(predicate::str::contains(initial_content));

    Ok(())
}

#[test]
fn does_not_find_links_in_ignored_files() -> Result<()> {
    let t = ZekeTester::new();
    t.setup_fs("subcommands/test_backlink/does_not_find_links_in_ignored_files")?;
    let initial_content = fs::read_to_string(t.temp_dir.child("20201223-b.md").path())?;

    t.zeke_backlink()?.assert().success();
    t.temp_dir
        .child("20201223-b.md")
        .assert(predicate::str::contains(initial_content));

    Ok(())
}

#[test]
fn does_not_find_links_in_gitignored_files_if_git() -> Result<()> {
    let t = ZekeTester::new();
    t.setup_fs("subcommands/test_backlink/does_not_find_links_in_gitignored_files_if_git")?;
    let initial_content = fs::read_to_string(t.temp_dir.child("20201223-b.md").path())?;
    // The fake .gitignore and .git dir here are created programmatically, rather than via path
    // copy because they may not play nice with zeke's real git repo.
    t.temp_dir.child(".gitignore").write_str("20201223-a.md")?;
    t.temp_dir.child(".git").create_dir_all()?;

    t.zeke_backlink()?.assert().success();
    t.temp_dir.child("20201223-b.md").assert(initial_content);

    Ok(())
}

#[test]
fn finds_links_in_gitignored_files_if_no_git() -> Result<()> {
    let t = ZekeTester::new();
    t.setup_fs("subcommands/test_backlink/finds_links_in_gitignored_files_if_no_git")?;
    let initial_content = fs::read_to_string(t.temp_dir.child("20201223-b.md").path())?;
    // The fake .gitignore and .git dir here are created programmatically, rather than via path
    // copy because they may not play nice with zeke's real git repo.
    t.temp_dir.child(".gitignore").write_str("20201223-a.md")?;

    t.zeke_backlink()?.assert().success();
    t.temp_dir
        .child("20201223-b.md")
        .assert(initial_content.replace("links: []", "links:\n- 20201223-a.md"));

    Ok(())
}

#[test]
fn does_not_panic_on_directories() -> Result<()> {
    let t = ZekeTester::new();
    t.temp_dir.child("subdir").create_dir_all()?;
    t.zeke_backlink()?.assert().success();
    Ok(())
}
