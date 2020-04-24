use crate::tester::ZekeTester;
use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::error::Error;

#[test]
fn unlinks_both_notes() -> Result<(), Box<dyn Error>> {
    let t = ZekeTester::new();
    let from_path = "a.md";
    let to_path = "b.md";
    let from_content = format!(
        "---
title: Sint omnis et qui qui
created: \"2020-04-19T18:23:24.774140Z\"
tags:
  - quisquam
links_in: []
links_out:
  - {}
---
Perspiciatis dolores corrupti sit.
Esse cumque saepe laboriosam.",
        to_path
    );
    let to_content = format!(
        "---
title: Sint omnis et qui qui
created: \"2020-04-19T18:23:24.774140Z\"
tags:
  - quisquam
links_in:
  - {}
links_out: []
---
Perspiciatis dolores corrupti sit.
Esse cumque saepe laboriosam.",
        from_path
    );
    t.temp_dir.child(from_path).write_str(&from_content)?;
    t.temp_dir.child(to_path).write_str(&to_content)?;

    t.zeke_unlink(from_path, to_path)?.assert().success();
    t.temp_dir
        .child(from_path)
        .assert(predicate::str::contains(format!("links_out:\n  - {}\n", to_path,)).not());
    t.temp_dir
        .child(to_path)
        .assert(predicate::str::contains(format!("links_in:\n  - {}\n", from_path,)).not());

    Ok(())
}

#[test]
fn fails_without_extant_from_path() -> Result<(), Box<dyn Error>> {
    let t = ZekeTester::new();
    let from_path = "a.md";
    let to_path = "b.md";
    t.temp_dir.child(to_path).touch()?;

    t.zeke_unlink(from_path, to_path)?.assert().failure();
    t.temp_dir
        .child(from_path)
        .assert(predicate::path::missing());
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
    t.temp_dir.child(from_path).touch()?;

    t.zeke_unlink(from_path, to_path)?.assert().failure();
    t.temp_dir
        .child(from_path)
        .assert(predicate::str::contains(format!("links_in:\n  - {}\n", to_path)).not());
    t.temp_dir.child(to_path).assert(predicate::path::missing());

    Ok(())
}

#[test]
fn does_not_modify_other_file_content() -> Result<(), Box<dyn Error>> {
    let t = ZekeTester::new();
    let from_path = "a.md";
    let to_path = "b.md";
    let from_content = format!(
        "---
title: Sint omnis et qui qui
created: \"2020-04-19T18:23:24.774140Z\"
tags:
  - quisquam
links_in: []
links_out:
  - {}
---
Perspiciatis dolores corrupti sit.
Esse cumque saepe laboriosam.",
        to_path
    );
    let to_content = format!(
        "---
title: Sint omnis et qui qui
created: \"2020-04-19T18:23:24.774140Z\"
tags:
  - quisquam
links_in:
  - {}
links_out: []
---
Perspiciatis dolores corrupti sit.
Esse cumque saepe laboriosam.",
        from_path
    );
    t.temp_dir.child(from_path).write_str(&from_content)?;
    t.temp_dir.child(to_path).write_str(&to_content)?;

    t.zeke_unlink(from_path, to_path)?.assert().success();
    t.temp_dir
        .child(from_path)
        .assert(from_content.replace(&format!("links_out:\n  - {}", to_path), "links_out: []"));
    t.temp_dir
        .child(to_path)
        .assert(to_content.replace(&format!("links_in:\n  - {}", from_path), "links_in: []"));

    Ok(())
}
