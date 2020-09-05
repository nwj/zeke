use crate::tester::ZekeTester;
use anyhow::Result;
use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;

#[test]
fn renames_note() -> Result<()> {
    let t = ZekeTester::new();
    let old_path = "20200819-foo.md";
    let content = "---
title: Foo
created: \"2020-08-19T18:23:24.774140Z\"
tags: []
links: []
--- ";
    let new_title = "Bar";
    let new_path = "20200819-bar.md";

    t.temp_dir.child(old_path).write_str(content)?;

    t.zeke_mv(old_path, new_title)?.assert().success();

    t.temp_dir
        .child(new_path)
        .assert(predicate::path::is_file());
    t.temp_dir
        .child(old_path)
        .assert(predicate::path::missing());

    Ok(())
}

#[test]
fn updates_note_title() -> Result<()> {
    let t = ZekeTester::new();
    let old_path = "20200819-foo.md";
    let content = "---
title: Foo
created: \"2020-08-19T18:23:24.774140Z\"
tags: []
links: []
--- ";
    let new_title = "Bar";
    let new_path = "20200819-bar.md";

    t.temp_dir.child(old_path).write_str(content)?;

    t.zeke_mv(old_path, new_title)?.assert().success();

    t.temp_dir
        .child(new_path)
        .assert(predicate::str::contains("---\ntitle: Bar"));

    Ok(())
}

#[test]
fn does_not_modify_other_content_of_the_moved_note() -> Result<()> {
    let t = ZekeTester::new();
    let path = "a.md";
    let content = "---
title: A
created: \"2020-08-19T18:23:24.774140Z\"
tags:
  - quisquam
links:
  - bar.md
foo: 12345
---
Perspiciatis dolores corrupti sit.
Esse cumque saepe laboriosam.";
    let new_title = "C";
    let new_path = "20200819-c.md";

    t.temp_dir.child(path).write_str(&content)?;

    t.zeke_mv(path, new_title)?.assert().success();

    t.temp_dir
        .child(new_path)
        .assert(content.replace("title: A", "title: C"));

    Ok(())
}

#[test]
fn updates_front_matter_linked_notes() -> Result<()> {
    let t = ZekeTester::new();
    let path_a = "a.md";
    let path_b = "b.md";
    let content_b = format!(
        "---
title: B
created: \"2020-08-19T18:23:24.774140Z\"
tags: []
links:
  - {}
---
Perspiciatis dolores corrupti sit.
Esse cumque saepe laboriosam.",
        path_a
    );
    t.temp_dir.child(path_a).touch()?;
    t.temp_dir.child(path_b).write_str(&content_b)?;

    t.zeke_mv(path_a, "C")?.assert().success();
    t.temp_dir
        .child(path_b)
        .assert(predicate::str::contains(format!("links:\n  - {}\n", path_a,)).not())
        .assert(predicate::str::contains("links:\n  - c.md\n"));

    Ok(())
}

#[test]
fn updates_content_linked_notes() -> Result<()> {
    let t = ZekeTester::new();
    let path_a = "a.md";
    let path_b = "b.md";
    let content_b = format!(
        "---
title: B
created: \"2020-08-19T18:23:24.774140Z\"
tags: []
links: []
---
Perspiciatis dolores [corrupti]({}) sit.
Esse cumque saepe laboriosam.",
        path_a
    );
    t.temp_dir.child(path_a).touch()?;
    t.temp_dir.child(path_b).write_str(&content_b)?;

    t.zeke_mv(path_a, "C")?.assert().success();
    t.temp_dir
        .child(path_b)
        .assert(predicate::str::contains(format!("[corrupti]({})", path_a,)).not())
        .assert(predicate::str::contains("[corrupti](c.md)"));

    Ok(())
}

#[test]
fn does_not_modify_other_aspects_of_linked_notes() -> Result<()> {
    let t = ZekeTester::new();
    let path_a = "a.md";
    let path_b = "b.md";
    let content_b = format!(
        "---
title: B
created: \"2020-08-19T18:23:24.774140Z\"
tags:
  - a.md
links:
  - {}
other_stuff: whatever
---
Perspiciatis dolores corrupti sit.
Esse cumque saepe laboriosam.",
        path_a
    );
    t.temp_dir.child(path_a).touch()?;
    t.temp_dir.child(path_b).write_str(&content_b)?;

    t.zeke_mv(path_a, "C")?.assert().success();
    t.temp_dir
        .child(path_b)
        .assert(content_b.replace(&format!("links:\n  - {}", path_a), "links:\n  - c.md"));

    Ok(())
}
