use crate::tester::ZekeTester;
use anyhow::Result;
use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use std::str;

#[test]
fn lists_tags() -> Result<()> {
    let t = ZekeTester::new();
    let path1 = "a.md";
    let path2 = "b.md";
    let content1 = "---
title: A
created: \"2020-04-19T18:23:24.774140Z\"
tags:
  - cats
  - zebras
links: []
--- ";
    let content2 = "---
title: A
created: \"2020-04-19T18:23:24.774140Z\"
tags:
  - cats
  - dogs
  - monkeys
links: []
--- ";

    t.temp_dir.child(path1).write_str(content1)?;
    t.temp_dir.child(path2).write_str(content2)?;

    let output = t.zeke_tags()?.output()?;
    let stdout = str::from_utf8(&output.stdout)?;
    assert_eq!(stdout, "cats\ndogs\nmonkeys\nzebras\n");

    Ok(())
}

#[test]
fn does_not_panic_on_directories() -> Result<()> {
    let t = ZekeTester::new();
    t.temp_dir.child("subdir").create_dir_all()?;

    t.zeke_tags()?.assert().success();
    Ok(())
}
