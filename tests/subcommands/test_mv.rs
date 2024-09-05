use crate::tester::ZekeTester;
use anyhow::Result;
use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::fs;

#[test]
fn renames_note() -> Result<()> {
    let t = ZekeTester::new();
    t.setup_fs("subcommands/test_mv/renames_note/")?;

    t.zeke_mv("20201010-a.md", "B")?.assert().success();
    t.temp_dir
        .child("20201010-b.md")
        .assert(predicate::path::is_file());
    t.temp_dir
        .child("20201010-a.md")
        .assert(predicate::path::missing());

    Ok(())
}

#[test]
fn updates_note_title() -> Result<()> {
    let t = ZekeTester::new();
    t.setup_fs("subcommands/test_mv/renames_note/")?;

    t.zeke_mv("20201010-a.md", "B")?.assert().success();
    t.temp_dir
        .child("20201010-b.md")
        .assert(predicate::str::contains("---\ntitle: B"));

    Ok(())
}

#[test]
fn does_not_modify_other_parts_of_moved_note() -> Result<()> {
    let t = ZekeTester::new();
    t.setup_fs("subcommands/test_mv/does_not_modify_other_parts_of_moved_note/")?;
    let initial_content = fs::read_to_string(t.temp_dir.child("20201010-a.md").path())?;

    t.zeke_mv("20201010-a.md", "B")?.assert().success();
    t.temp_dir
        .child("20201010-b.md")
        .assert(initial_content.replace("title: A", "title: B"));

    Ok(())
}

#[test]
fn updates_front_matter_linked_notes() -> Result<()> {
    let t = ZekeTester::new();
    t.setup_fs("subcommands/test_mv/updates_front_matter_linked_notes/")?;

    t.zeke_mv("20201010-a.md", "C")?.assert().success();
    t.temp_dir
        .child("20201010-b.md")
        .assert(predicate::str::contains("links:\n- 20201010-a.md\n").not())
        .assert(predicate::str::contains("links:\n- 20201010-c.md\n"));

    Ok(())
}

#[test]
fn updates_content_linked_notes() -> Result<()> {
    let t = ZekeTester::new();
    t.setup_fs("subcommands/test_mv/updates_content_linked_notes/")?;

    t.zeke_mv("20201010-a.md", "C")?.assert().success();
    t.temp_dir
        .child("20201010-b.md")
        .assert(predicate::str::contains("[corrupti](20201010-a.md)").not())
        .assert(predicate::str::contains("[corrupti](20201010-c.md)"));

    Ok(())
}

#[test]
fn does_not_modify_other_parts_of_linked_notes() -> Result<()> {
    let t = ZekeTester::new();
    t.setup_fs("subcommands/test_mv/does_not_modify_other_parts_of_linked_notes/")?;
    let initial_content = fs::read_to_string(t.temp_dir.child("20201010-b.md").path())?;

    t.zeke_mv("20201010-a.md", "C")?.assert().success();
    t.temp_dir
        .child("20201010-b.md")
        .assert(initial_content.replace("links:\n- 20201010-a.md", "links:\n- 20201010-c.md"));

    Ok(())
}

#[test]
fn only_updates_links_in_markdown_files() -> Result<()> {
    let t = ZekeTester::new();
    t.setup_fs("subcommands/test_mv/only_updates_links_in_markdown_files")?;
    let initial_content = fs::read_to_string(t.temp_dir.child("20201010-b.txt").path())?;

    t.zeke_mv("20201010-a.md", "C")?.assert().success();
    t.temp_dir.child("20201010-b.txt").assert(initial_content);

    Ok(())
}

#[test]
fn does_not_update_links_in_hidden_files() -> Result<()> {
    let t = ZekeTester::new();
    t.setup_fs("subcommands/test_mv/does_not_update_links_in_hidden_files")?;
    let initial_content = fs::read_to_string(t.temp_dir.child(".20201010-b.md").path())?;

    t.zeke_mv("20201010-a.md", "C")?.assert().success();
    t.temp_dir.child(".20201010-b.md").assert(initial_content);

    Ok(())
}

#[test]
fn does_not_update_links_in_ignored_files() -> Result<()> {
    let t = ZekeTester::new();
    t.setup_fs("subcommands/test_mv/does_not_update_links_in_ignored_files")?;
    let initial_content = fs::read_to_string(t.temp_dir.child("20201010-b.md").path())?;

    t.zeke_mv("20201010-a.md", "C")?.assert().success();
    t.temp_dir.child("20201010-b.md").assert(initial_content);

    Ok(())
}
#[test]
fn does_not_update_links_in_gitignored_files_if_git() -> Result<()> {
    let t = ZekeTester::new();
    t.setup_fs("subcommands/test_mv/does_not_update_links_in_gitignored_files_if_git")?;
    let initial_content = fs::read_to_string(t.temp_dir.child("20201010-b.md").path())?;
    // The fake .gitignore and .git dir here are created programmatically, rather than via path
    // copy because they may not play nice with zeke's real git repo.
    t.temp_dir.child(".gitignore").write_str("20201010-b.md")?;
    t.temp_dir.child(".git").create_dir_all()?;

    t.zeke_mv("20201010-a.md", "C")?.assert().success();
    t.temp_dir.child("20201010-b.md").assert(initial_content);

    Ok(())
}

#[test]
fn updates_links_in_gitignored_files_if_no_git() -> Result<()> {
    let t = ZekeTester::new();
    t.setup_fs("subcommands/test_mv/updates_links_in_gitignored_files_if_no_git")?;
    // The fake .gitignore here is created programmatically, rather than via path
    // copy because it may not play nice with zeke's real git repo.
    t.temp_dir.child(".gitignore").write_str("20201010-b.md")?;

    t.zeke_mv("20201010-a.md", "C")?.assert().success();
    t.temp_dir
        .child("20201010-b.md")
        .assert(predicate::str::contains("links:\n- 20201010-a.md\n").not())
        .assert(predicate::str::contains("links:\n- 20201010-c.md\n"));

    Ok(())
}

#[test]
fn does_not_panic_on_directories() -> Result<()> {
    let t = ZekeTester::new();
    let path_a = "a.md";
    t.temp_dir.child(path_a).touch()?;
    t.temp_dir.child("subdir").create_dir_all()?;

    t.zeke_mv(path_a, "B")?.assert().success();
    Ok(())
}
