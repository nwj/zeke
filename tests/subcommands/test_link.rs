use crate::tester::ZekeTester;
use anyhow::Result;
use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;

#[test]
fn links_both_notes() -> Result<()> {
    let t = ZekeTester::new();
    let path_a = "a.md";
    let path_b = "b.md";
    t.temp_dir.child(path_a).touch()?;
    t.temp_dir.child(path_b).touch()?;

    t.zeke_link(path_a, path_b)?.assert().success();
    t.temp_dir
        .child(path_a)
        .assert(predicate::str::contains(format!(
            "links:\n  - {}\n",
            path_b,
        )));
    t.temp_dir
        .child(path_b)
        .assert(predicate::str::contains(format!(
            "links:\n  - {}\n",
            path_a,
        )));

    Ok(())
}

#[test]
fn fails_without_extant_path_a() -> Result<()> {
    let t = ZekeTester::new();
    let path_a = "a.md";
    let path_b = "b.md";
    t.temp_dir.child(path_b).touch()?;

    t.zeke_link(path_a, path_b)?.assert().failure();
    t.temp_dir.child(path_a).assert(predicate::path::missing());
    t.temp_dir
        .child(path_b)
        .assert(predicate::str::contains(format!("links:\n  - {}\n", path_a)).not());

    Ok(())
}

#[test]
fn fails_without_extant_path_b() -> Result<()> {
    let t = ZekeTester::new();
    let path_a = "a.md";
    let path_b = "b.md";
    t.temp_dir.child(path_a).touch()?;

    t.zeke_link(path_a, path_b)?.assert().failure();
    t.temp_dir
        .child(path_a)
        .assert(predicate::str::contains(format!("links:\n  - {}\n", path_b)).not());
    t.temp_dir.child(path_b).assert(predicate::path::missing());

    Ok(())
}

#[test]
fn does_not_modify_other_file_content() -> Result<()> {
    let t = ZekeTester::new();
    let path_a = "a.md";
    let path_b = "b.md";
    let content = "---
title: Sint omnis et qui qui
created: \"2020-04-19T18:23:24.774140Z\"
tags:
  - quisquam
links: []
foobar:
  - foo
  - 123
  - false
---
Perspiciatis dolores corrupti sit.
Esse cumque saepe laboriosam.";

    t.temp_dir.child(path_a).write_str(content)?;
    t.temp_dir.child(path_b).write_str(content)?;

    t.zeke_link(path_a, path_b)?.assert().success();
    t.temp_dir
        .child(path_a)
        .assert(content.replace("links: []", &format!("links:\n  - {}", path_b)));
    t.temp_dir
        .child(path_b)
        .assert(content.replace("links: []", &format!("links:\n  - {}", path_a)));

    Ok(())
}

#[test]
fn idempotent_if_linked_repeatedly() -> Result<()> {
    let t = ZekeTester::new();
    let path_a = "a.md";
    let path_b = "b.md";
    t.temp_dir.child(path_a).touch()?;
    t.temp_dir.child(path_b).touch()?;

    t.zeke_link(path_a, path_b)?.assert().success();
    t.zeke_link(path_a, path_b)?.assert().success();

    t.temp_dir
        .child(path_a)
        .assert(predicate::str::contains(path_b).count(1));
    t.temp_dir
        .child(path_b)
        .assert(predicate::str::contains(path_a).count(1));

    Ok(())
}
