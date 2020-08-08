use crate::tester::ZekeTester;
use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::error::Error;

#[test]
fn unlinks_both_notes() -> Result<(), Box<dyn Error>> {
    let t = ZekeTester::new();
    let path_a = "a.md";
    let path_b = "b.md";
    let content_a = format!(
        "---
title: Sint omnis et qui qui
created: \"2020-04-19T18:23:24.774140Z\"
tags:
  - quisquam
links:
  - {}
---
Perspiciatis dolores corrupti sit.
Esse cumque saepe laboriosam.",
        path_b
    );
    let content_b = format!(
        "---
title: Sint omnis et qui qui
created: \"2020-04-19T18:23:24.774140Z\"
tags:
  - quisquam
links:
  - {}
---
Perspiciatis dolores corrupti sit.
Esse cumque saepe laboriosam.",
        path_a
    );
    t.temp_dir.child(path_a).write_str(&content_a)?;
    t.temp_dir.child(path_b).write_str(&content_b)?;

    t.zeke_unlink(path_a, path_b)?.assert().success();
    t.temp_dir
        .child(path_a)
        .assert(predicate::str::contains(format!("links:\n  - {}\n", path_b,)).not());
    t.temp_dir
        .child(path_b)
        .assert(predicate::str::contains(format!("links:\n  - {}\n", path_a,)).not());

    Ok(())
}

#[test]
fn fails_without_extant_path_a() -> Result<(), Box<dyn Error>> {
    let t = ZekeTester::new();
    let path_a = "a.md";
    let path_b = "b.md";
    t.temp_dir.child(path_b).touch()?;

    t.zeke_unlink(path_a, path_b)?.assert().failure();
    t.temp_dir.child(path_a).assert(predicate::path::missing());
    t.temp_dir
        .child(path_b)
        .assert(predicate::str::contains(format!("links:\n  - {}\n", path_a)).not());

    Ok(())
}

#[test]
fn fails_without_extant_path_b() -> Result<(), Box<dyn Error>> {
    let t = ZekeTester::new();
    let path_a = "a.md";
    let path_b = "b.md";
    t.temp_dir.child(path_a).touch()?;

    t.zeke_unlink(path_a, path_b)?.assert().failure();
    t.temp_dir
        .child(path_a)
        .assert(predicate::str::contains(format!("links:\n  - {}\n", path_b)).not());
    t.temp_dir.child(path_b).assert(predicate::path::missing());

    Ok(())
}

#[test]
fn does_not_modify_other_file_content() -> Result<(), Box<dyn Error>> {
    let t = ZekeTester::new();
    let path_a = "a.md";
    let path_b = "b.md";
    let content_a = format!(
        "---
title: Sint omnis et qui qui
created: \"2020-04-19T18:23:24.774140Z\"
tags:
  - quisquam
links:
  - {}
---
Perspiciatis dolores corrupti sit.
Esse cumque saepe laboriosam.",
        path_b
    );
    let content_b = format!(
        "---
title: Sint omnis et qui qui
created: \"2020-04-19T18:23:24.774140Z\"
tags:
  - quisquam
links:
  - {}
---
Perspiciatis dolores corrupti sit.
Esse cumque saepe laboriosam.",
        path_a
    );
    t.temp_dir.child(path_a).write_str(&content_a)?;
    t.temp_dir.child(path_b).write_str(&content_b)?;

    t.zeke_unlink(path_a, path_b)?.assert().success();
    t.temp_dir
        .child(path_a)
        .assert(content_a.replace(&format!("links:\n  - {}", path_b), "links: []"));
    t.temp_dir
        .child(path_b)
        .assert(content_b.replace(&format!("links:\n  - {}", path_a), "links: []"));

    Ok(())
}
