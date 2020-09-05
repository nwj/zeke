use crate::tester::ZekeTester;
use anyhow::Result;
use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;

#[test]
fn adds_links_to_notes_references_by_other_notes() -> Result<()> {
    let t = ZekeTester::new();
    let path1 = "a.md";
    let path2 = "b.md";
    let path3 = "c.md";
    let content1 = "---
title: A
created: \"2020-04-19T18:23:24.774140Z\"
tags: []
links: []
---
This is a reference to [B](b.md) and [C](c.md)
";
    let content2 = "---
title: B
created: \"2020-04-19T18:23:24.774140Z\"
tags: []
links: []
---
No links to be found here.";
    let content3 = "---
title: C
created: \"2020-04-19T18:23:24.774140Z\"
tags: []
links: []
---
This is a reference to [somewhere else](b.md)";

    t.temp_dir.child(path1).write_str(content1)?;
    t.temp_dir.child(path2).write_str(content2)?;
    t.temp_dir.child(path3).write_str(content3)?;

    t.zeke_backlink()?.assert().success();

    t.temp_dir
        .child(path2)
        .assert(predicate::str::contains(path1));
    t.temp_dir
        .child(path2)
        .assert(predicate::str::contains(path3));
    t.temp_dir
        .child(path3)
        .assert(predicate::str::contains(path1));

    Ok(())
}

#[test]
fn does_not_panic_on_directories() -> Result<()> {
    let t = ZekeTester::new();
    t.temp_dir.child("subdir").create_dir_all()?;
    t.zeke_backlink()?.assert().success();
    Ok(())
}

#[test]
fn idempotent_if_invoked_repeatedly() -> Result<()> {
    let t = ZekeTester::new();
    let path1 = "a.md";
    let path2 = "b.md";
    let content1 = "---
title: A
created: \"2020-04-19T18:23:24.774140Z\"
tags: []
links: []
---
This is a reference to [B](b.md)
";
    t.temp_dir.child(path1).write_str(content1)?;
    t.temp_dir.child(path2).touch()?;

    t.zeke_backlink()?.assert().success();
    t.zeke_backlink()?.assert().success();

    t.temp_dir
        .child(path2)
        .assert(predicate::str::contains(path1).count(1));

    Ok(())
}
