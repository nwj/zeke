use crate::tester::ZekeTester;
use anyhow::Result;
use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;

#[test]
fn untags_the_note() -> Result<()> {
    let t = ZekeTester::new();
    let path = "note.md";
    let tag = "cats";
    let content = format!(
        "---
title: Sint omnis et qui qui
created: 2020-04-19T18:23:24.774140Z
tags:
- {}
links: []
---
Perspiciatis dolores corrupti sit.
Esse cumque saepe laboriosam.",
        tag
    );
    t.temp_dir.child(path).write_str(&content)?;

    t.zeke_untag(tag, path)?.assert().success();
    t.temp_dir
        .child(path)
        .assert(predicate::str::contains(format!("tags:\n- {}\n", tag,)).not());

    Ok(())
}

#[test]
fn can_untag_multiple_notes() -> Result<()> {
    let t = ZekeTester::new();
    let path = "note.md";
    let path2 = "note2.md";
    let tag = "cats";
    let content = format!(
        "---
title: Sint omnis et qui qui
created: 2020-04-19T18:23:24.774140Z
tags:
- {}
links: []
---
Perspiciatis dolores corrupti sit.
Esse cumque saepe laboriosam.",
        tag
    );
    t.temp_dir.child(path).write_str(&content)?;
    t.temp_dir.child(path2).write_str(&content)?;

    t.zeke_untag(tag, path)?.arg(path2).assert().success();
    t.temp_dir
        .child(path)
        .assert(predicate::str::contains(format!("tags:\n- {}\n", tag,)).not());
    t.temp_dir
        .child(path2)
        .assert(predicate::str::contains(format!("tags:\n- {}\n", tag,)).not());

    Ok(())
}

#[test]
fn fails_without_extant_file() -> Result<()> {
    let t = ZekeTester::new();
    let path = "note.md";
    let tag = "cats";

    t.zeke_untag(tag, path)?.assert().failure();
    t.temp_dir.child(path).assert(predicate::path::missing());

    Ok(())
}

#[test]
fn does_not_modify_other_file_content() -> Result<()> {
    let t = ZekeTester::new();
    let path = "a.md";
    let tag = "dogs";
    let content = format!(
        "---
title: Sint omnis et qui qui
created: 2020-04-19T18:23:24.774140Z
tags:
- {}
links:
- b.md
color: purple
---
Perspiciatis dolores corrupti sit.
Esse cumque saepe laboriosam.",
        tag
    );
    t.temp_dir.child(path).write_str(&content)?;

    t.zeke_untag(tag, path)?.assert().success();
    t.temp_dir
        .child(path)
        .assert(content.replace(&format!("tags:\n- {}", tag), "tags: []"));

    Ok(())
}
