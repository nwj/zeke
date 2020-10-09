use crate::tester::ZekeTester;
use anyhow::Result;
use assert_fs::prelude::*;
use std::str;

#[test]
fn alphabetically_sorts_tags_list() -> Result<()> {
    let t = ZekeTester::new();
    t.setup_fs("subcommands/test_tags/alphabetically_sorts_tags_list/")?;

    let output = t.zeke_tags()?.output()?;
    let stdout = str::from_utf8(&output.stdout)?;
    assert_eq!(stdout, "blue\ngreen\nred\n");

    Ok(())
}

#[test]
fn dedupes_tags_list() -> Result<()> {
    let t = ZekeTester::new();
    t.setup_fs("subcommands/test_tags/dedupes_tags_list")?;

    let output = t.zeke_tags()?.output()?;
    let stdout = str::from_utf8(&output.stdout)?;
    assert_eq!(stdout, "red\n");

    Ok(())
}

#[test]
fn reads_files_in_subdirs() -> Result<()> {
    let t = ZekeTester::new();
    t.setup_fs("subcommands/test_tags/reads_files_in_subdirs")?;

    let output = t.zeke_tags()?.output()?;
    let stdout = str::from_utf8(&output.stdout)?;
    assert_eq!(stdout, "red\n");

    Ok(())
}

#[test]
fn only_reads_markdown_files() -> Result<()> {
    let t = ZekeTester::new();
    t.setup_fs("subcommands/test_tags/only_reads_markdown_files")?;

    let output = t.zeke_tags()?.output()?;
    let stdout = str::from_utf8(&output.stdout)?;
    assert_eq!(stdout, "");

    Ok(())
}

#[test]
fn does_not_read_hidden_files() -> Result<()> {
    let t = ZekeTester::new();
    t.setup_fs("subcommands/test_tags/does_not_read_hidden_files")?;

    let output = t.zeke_tags()?.output()?;
    let stdout = str::from_utf8(&output.stdout)?;
    assert_eq!(stdout, "");

    Ok(())
}

#[test]
fn does_not_read_files_in_ignore() -> Result<()> {
    let t = ZekeTester::new();
    t.setup_fs("subcommands/test_tags/does_not_read_files_in_ignore")?;

    let output = t.zeke_tags()?.output()?;
    let stdout = str::from_utf8(&output.stdout)?;
    assert_eq!(stdout, "");

    Ok(())
}

#[test]
fn does_not_read_files_in_gitignore_if_git() -> Result<()> {
    let t = ZekeTester::new();
    t.setup_fs("subcommands/test_tags/does_not_read_files_in_gitignore_if_git")?;
    // The fake .gitignore and .git dir here are created programmatically, rather than via path
    // copy because they may not play nice with zeke's real git repo.
    t.temp_dir.child(".gitignore").write_str("20201009-a.md")?;
    t.temp_dir.child(".git").create_dir_all()?;

    let output = t.zeke_tags()?.output()?;
    let stdout = str::from_utf8(&output.stdout)?;
    assert_eq!(stdout, "");

    Ok(())
}

#[test]
fn reads_files_in_gitignore_if_no_git() -> Result<()> {
    let t = ZekeTester::new();
    t.setup_fs("subcommands/test_tags/reads_files_in_gitignore_if_no_git")?;
    // The fake .gitignore here is created programmatically, rather than via path
    // copy because it may not play nice with zeke's real git repo.
    t.temp_dir.child(".gitignore").write_str("20201009-a.md")?;

    let output = t.zeke_tags()?.output()?;
    let stdout = str::from_utf8(&output.stdout)?;
    assert_eq!(stdout, "red\n");

    Ok(())
}
