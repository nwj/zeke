use crate::tester::ZekeTester;
use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::error::Error;

#[test]
fn links_both_notes() -> Result<(), Box<dyn Error>> {
    let t = ZekeTester::new();
    let from_path = "a.md";
    let to_path = "b.md";
    t.temp_dir.child(&from_path).touch()?;
    t.temp_dir.child(&to_path).touch()?;

    t.zeke_link(&from_path, &to_path)?.assert().success();
    t.temp_dir
        .child(from_path)
        .assert(predicate::str::contains(format!(
            "links_out:\n  - {}\n",
            to_path
        )));
    t.temp_dir
        .child(to_path)
        .assert(predicate::str::contains(format!(
            "links_in:\n  - {}\n",
            from_path
        )));

    Ok(())
}

#[test]
fn fails_without_extant_from_path() -> Result<(), Box<dyn Error>> {
    let t = ZekeTester::new();
    let from_path = "a.md";
    let to_path = "b.md";
    t.temp_dir.child(&to_path).touch()?;

    t.zeke_link(&from_path, &to_path)?.assert().failure();
    t.temp_dir
        .child(to_path)
        .assert(predicate::str::contains(format!("links_in:\n  - {}\n", from_path)).not());

    Ok(())
}

#[test]
fn fails_without_extant_to_path() -> Result<(), Box<dyn Error>> {
    let t = ZekeTester::new();
    let from_path = "a.md";
    let to_path = "b.md";
    t.temp_dir.child(&from_path).touch()?;

    t.zeke_link(&from_path, &to_path)?.assert().failure();
    t.temp_dir
        .child(from_path)
        .assert(predicate::str::contains(format!("links_in:\n  - {}\n", to_path)).not());

    Ok(())
}

#[test]
fn does_not_modify_other_file_content() -> Result<(), Box<dyn Error>> {
    let t = ZekeTester::new();
    let from_path = "a.md";
    let to_path = "b.md";
    let content = "---
title: Sint omnis et qui qui
created: \"2020-04-19T18:23:24.774140Z\"
tags:
  - quisquam
links_in: []
links_out: []
---
Perspiciatis dolores corrupti sit.
Esse cumque saepe laboriosam.";

    t.temp_dir.child(&from_path).write_str(&content)?;
    t.temp_dir.child(&to_path).write_str(&content)?;

    t.zeke_link(&from_path, &to_path)?.assert().success();
    t.temp_dir.child(&from_path).assert(content.replace("links_out: []", "links_out:\n  - b.md"));
    t.temp_dir.child(&to_path).assert(content.replace("links_in: []", "links_in:\n  - a.md"));

    Ok(())
}
