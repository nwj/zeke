use crate::tester::ZekeTester;
use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::error::Error;

#[test]
fn formats_note_graph_in_dot_format() -> Result<(), Box<dyn Error>> {
    let t = ZekeTester::new();
    let path1 = "a.md";
    let path2 = "b.md";
    let path3 = "c.md";
    let content1 = "---
title: A
created: \"2020-04-19T18:23:24.774140Z\"
tags: []
links:
  - b.md
  - c.md
--- ";
    let content2 = "---
title: B
created: \"2020-04-19T18:23:24.774140Z\"
tags: []
links:
  - a.md
  - c.md
--- ";
    let content3 = "---
title: C
created: \"2020-04-19T18:23:24.774140Z\"
tags: []
links:
  - a.md
  - b.md
--- ";

    t.temp_dir.child(path1).write_str(content1)?;
    t.temp_dir.child(path2).write_str(content2)?;
    t.temp_dir.child(path3).write_str(content3)?;

    t.zeke_graph()?
        .assert()
        .success()
        .stdout(predicate::str::contains("[ label = \"A\" ]\n"))
        .stdout(predicate::str::contains("[ label = \"B\" ]\n"))
        .stdout(predicate::str::contains("[ label = \"C\" ]\n"))
        .stdout(predicate::str::contains("0 -- 1 [ label = \"\" ]\n"))
        .stdout(predicate::str::contains("0 -- 2 [ label = \"\" ]\n"))
        .stdout(predicate::str::contains("1 -- 2 [ label = \"\" ]\n"));
    Ok(())
}

#[test]
fn includes_links_from_content_in_the_graph() -> Result<(), Box<dyn Error>> {
    let t = ZekeTester::new();
    let path1 = "a.md";
    let path2 = "b.md";
    let content1 = "---
title: A
created: \"2020-04-19T18:23:24.774140Z\"
tags: []
links: []
---
Lorem ipsum dolor sit amet, [consectetur](b.md) adipiscing elit.";
    let content2 = "---
title: B
created: \"2020-04-19T18:23:24.774140Z\"
tags: []
links: []
---";
    t.temp_dir.child(path1).write_str(content1)?;
    t.temp_dir.child(path2).write_str(content2)?;

    t.zeke_graph()?
        .assert()
        .success()
        .stdout(predicate::str::contains("[ label = \"A\" ]\n"))
        .stdout(predicate::str::contains("[ label = \"B\" ]\n"))
        .stdout(predicate::str::contains("1 -- 0 [ label = \"\" ]\n"));
    Ok(())
}

#[test]
fn works_with_uncleaned_paths() -> Result<(), Box<dyn Error>> {
    let t = ZekeTester::new();
    let path1 = "a.md";
    let path2 = "b.md";
    let content1 = "---
title: A
created: \"2020-04-19T18:23:24.774140Z\"
tags: []
links:
  - ./b.md
--- ";
    let content2 = "---
title: B
created: \"2020-04-19T18:23:24.774140Z\"
tags: []
links:
  - /foo/../a.md
--- ";

    t.temp_dir.child(path1).write_str(content1)?;
    t.temp_dir.child(path2).write_str(content2)?;

    t.zeke_graph()?
        .assert()
        .success()
        .stdout(predicate::str::contains("[ label = \"A\" ]\n"))
        .stdout(predicate::str::contains("[ label = \"B\" ]\n"))
        .stdout(predicate::str::contains("1 -- 0 [ label = \"\" ]\n"));

    Ok(())
}

#[test]
fn does_not_panic_on_directories() -> Result<(), Box<dyn Error>> {
    let t = ZekeTester::new();
    t.temp_dir.child("subdir").create_dir_all()?;
    t.zeke_graph()?.assert().success();
    Ok(())
}
