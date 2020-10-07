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
title: B
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
title: B
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
fn reads_files_in_subdirectories() -> Result<()> {
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

    t.temp_dir.child("subdir").create_dir_all()?;
    t.setup_fs(vec![("subdir/a.md", &content1)])?;
    let output = t.zeke_tags()?.output()?;
    let stdout = str::from_utf8(&output.stdout)?;
    let test = t.temp_dir.into_persistent();
    println!("{:?}", test.path().display());
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
title: B
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
fn does_not_read_hidden_files() -> Result<()> {
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
title: B
created: \"2020-04-19T18:23:24.774140Z\"
tags:
  - green
  - blue
links: []
--- ";
    t.setup_fs(vec![("a.md", &content1), (".hidden.md", &content2)])?;

    let output = t.zeke_tags()?.output()?;
    let stdout = str::from_utf8(&output.stdout)?;

    assert_eq!(stdout, format!("{}\n", tag1));
    Ok(())
}

#[test]
fn does_not_read_files_specified_via_ignore() -> Result<()> {
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
    let path2 = "b.md";
    let content2 = "---
title: B
created: \"2020-04-19T18:23:24.774140Z\"
tags:
  - green
  - blue
links: []
--- ";
    let ignore_content = format!("{}", path2);
    t.setup_fs(vec![
        ("a.md", &content1),
        (path2, &content2),
        (".ignore", &ignore_content),
    ])?;

    let output = t.zeke_tags()?.output()?;
    let stdout = str::from_utf8(&output.stdout)?;

    assert_eq!(stdout, format!("{}\n", tag1));
    Ok(())
}

#[test]
fn does_not_read_files_specified_via_gitignore_if_git() -> Result<()> {
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
    let path2 = "b.md";
    let content2 = "---
title: B
created: \"2020-04-19T18:23:24.774140Z\"
tags:
  - green
  - blue
links: []
--- ";
    let ignore_content = format!("{}", path2);
    t.setup_fs(vec![
        ("a.md", &content1),
        (path2, &content2),
        (".gitignore", &ignore_content),
    ])?;
    // This is done to fake that we're in a git repo
    t.temp_dir.child(".git").create_dir_all()?;

    let output = t.zeke_tags()?.output()?;
    let stdout = str::from_utf8(&output.stdout)?;

    assert_eq!(stdout, format!("{}\n", tag1));
    Ok(())
}

#[test]
fn reads_files_specified_via_gitignore_if_no_git() -> Result<()> {
    let t = ZekeTester::new();
    let tag1 = "red";
    let tag2 = "blue";
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
    let path2 = "b.md";
    let content2 = format!(
        "---
title: B
created: \"2020-04-19T18:23:24.774140Z\"
tags:
  - {}
links: []
--- ",
        tag2
    );
    let ignore_content = format!("{}", path2);
    t.setup_fs(vec![
        ("a.md", &content1),
        (path2, &content2),
        (".gitignore", &ignore_content),
    ])?;

    let output = t.zeke_tags()?.output()?;
    let stdout = str::from_utf8(&output.stdout)?;

    assert_eq!(stdout, format!("{}\n{}\n", tag2, tag1));
    Ok(())
}

#[test]
fn does_not_panic_on_directories() -> Result<()> {
    let t = ZekeTester::new();
    t.temp_dir.child("subdir").create_dir_all()?;
    t.zeke_tags()?.assert().success();
    Ok(())
}
