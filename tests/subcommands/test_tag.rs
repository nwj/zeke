use crate::tester::ZekeTester;
use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::error::Error;

#[test]
fn tags_the_note() -> Result<(), Box<dyn Error>> {
    let t = ZekeTester::new();
    let path = "note.md";
    let tag = "cats";
    t.temp_dir.child(path).touch()?;

    t.zeke_tag(path, tag)?.assert().success();
    t.temp_dir
        .child(path)
        .assert(predicate::str::contains(format!("tags:\n  - {}\n", tag,)));

    Ok(())
}

#[test]
fn fails_without_extant_file() -> Result<(), Box<dyn Error>> {
    let t = ZekeTester::new();
    let path = "note.md";
    let tag = "cats";

    t.zeke_tag(path, tag)?.assert().failure();
    t.temp_dir.child(path).assert(predicate::path::missing());

    Ok(())
}

#[test]
fn does_not_modify_other_file_content() -> Result<(), Box<dyn Error>> {
    let t = ZekeTester::new();
    let path = "a.md";
    let tag = "dogs";
    let content = "---
title: Sint omnis et qui qui
created: \"2020-04-19T18:23:24.774140Z\"
tags: []
links:
  - b.md
foo: bar
---
Perspiciatis dolores corrupti sit.
Esse cumque saepe laboriosam.";

    t.temp_dir.child(path).write_str(content)?;

    t.zeke_tag(path, tag)?.assert().success();
    t.temp_dir
        .child(path)
        .assert(content.replace("tags: []", &format!("tags:\n  - {}", tag)));

    Ok(())
}

#[test]
fn idempotent_if_tagged_repeatedly() -> Result<(), Box<dyn Error>> {
    let t = ZekeTester::new();
    let path = "note.md";
    let tag = "cats";
    t.temp_dir.child(path).touch()?;

    t.zeke_tag(path, tag)?.assert().success();
    t.zeke_tag(path, tag)?.assert().success();
    t.temp_dir
        .child(path)
        .assert(predicate::str::contains(tag).count(1));

    Ok(())
}
