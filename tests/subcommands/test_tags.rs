use crate::tester::ZekeTester;
use anyhow::Result;
use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use std::str;

#[test]
fn lists_tags_sorted_alphabetically() -> Result<()> {
    let t = ZekeTester::new();
    let tag1 = "red";
    let tag2 = "green";
    let tag3 = "blue";
    let content1 = format!(
        "---
title: A
created: \"2020-04-19T18:23:24.774140Z\"
tags:
  - {}
  - {}
links: []
--- ",
        tag1, tag2
    );
    let content2 = format!(
        "---
title: A
created: \"2020-04-19T18:23:24.774140Z\"
tags:
  - {}
links: []
--- ",
        tag3
    );
    t.setup_fs(vec![("a.md", &content1), ("b.md", &content2)])?;
    let output = t.zeke_tags()?.output()?;
    let stdout = str::from_utf8(&output.stdout)?;
    assert_eq!(stdout, format!("{}\n{}\n{}\n", tag3, tag2, tag1));
    Ok(())
}

#[test]
fn dedupes_list_of_tags() -> Result<()> {
    let t = ZekeTester::new();
    let tag1 = "red";
    let content1 = format!(
        "---
title: A
created: \"2020-04-19T18:23:24.774140Z\"
tags:
  - {}
  - {}
links: []
--- ",
        tag1, tag1
    );
    let content2 = format!(
        "---
title: A
created: \"2020-04-19T18:23:24.774140Z\"
tags:
  - {}
links: []
--- ",
        tag1
    );
    t.setup_fs(vec![("a.md", &content1), ("b.md", &content2)])?;
    let output = t.zeke_tags()?.output()?;
    let stdout = str::from_utf8(&output.stdout)?;
    assert_eq!(stdout, format!("{}\n", tag1));
    Ok(())
}

#[test]
fn does_not_read_non_markdown_files() -> Result<()> {
    let t = ZekeTester::new();
    let tag1 = "red";
    let content1 = format!(
        "---
title: A
created: \"2020-04-19T18:23:24.774140Z\"
tags:
  - {}
links: []
--- ",
        tag1
    );
    let content2 = "---
title: A
created: \"2020-04-19T18:23:24.774140Z\"
tags:
  - green
  - blue
links: []
--- ";
    t.setup_fs(vec![("a.md", &content1), ("b.txt", &content2)])?;
    let output = t.zeke_tags()?.output()?;
    let stdout = str::from_utf8(&output.stdout)?;
    assert_eq!(stdout, format!("{}\n", tag1));
    Ok(())
}

#[test]
fn does_not_panic_on_directories() -> Result<()> {
    let t = ZekeTester::new();
    t.temp_dir.child("subdir").create_dir_all()?;
    t.zeke_tags()?.assert().success();
    Ok(())
}
