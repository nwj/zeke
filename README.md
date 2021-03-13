# zeke

Zeke is a CLI for managing a knowledge base built on a local folder of plain text Markdown files.

## Philosophy

A file system, a text editor, and Markdown can provide most of the functionality of more "feature rich" note taking applications. These technologies have survived the test of time, will likely continue to work (even years from now), and leave you in full control of your data. Since notes are just text files, you are free to use any of the wide variety of sync, encryption, and data processing tools that work with text. It is an ideal system for projects that aim at longevity, such as personal knowledge bases and zettelkasten.

Zeke aims to augment these technologies by automating some of the more manual or error prone tasks that are required to organize and manage a large collection of text notes.

## Usage

### Creating Notes

```
$ zeke new "My Note"
Created `20200502-my_note.md` note file

$ cat 20200502-my_note.md
 ---
 title: My Note
 created: "2020-05-02T21:48:27.450966Z"
 tags: []
 links: []
 ---
```

Notes are Markdown files, named using a date and a formatted version of the note title. Each note includes YAML front-matter with basic meta-data.

The optional `-e` flag will open the new note in whichever editor is specified by your `EDITOR` or `ZEKE_EDITOR` environment variables. If both are set, `ZEKE_EDITOR` will take precedence over `EDITOR`.

```
$ export EDITOR="vim"
$ zeke new -e "My Note"
```

### Tagging Notes
```
$ zeke tag my-tag 20200502-my_note.md
Tagged `20200502-my_note.md` with `my-tag`

$ cat 20200502-my_note.md
---
title: My Note
created: "2020-05-02T22:06:27.479009Z"
tags:
  - my-tag
links: []
---
```

Tags are recorded in the front-matter of notes.

You can untag notes in a similar manner:

```
$ zeke untag my-tag 20200502-my_note.md
Untagged `20200502-my_note.md` from `my-tag`
```

You can also generate a simple list of all the tags in your notes:

```
$ zeke tags
my-tag
another-tag
yet-another-tag
```

### Linking Notes

```
$ zeke link one.md two.md
Linked `one.md` to `two.md`

$ cat one.md
---
title: one
created: "2020-05-02T22:24:35.253945Z"
tags: []
links: []
  - two.md
---

$ cat two.md
---
title: two
created: "2020-05-02T22:24:38.706218Z"
tags: []
links:
  - one.md
---
```

Links are recorded in the front-matter of both notes.

Unlinking notes works in a similar manner:

```
$ zeke unlink one.md two.md
```

## Installation

### From Source Code

zeke is written in Rust, so you'll need to [install that](https://www.rust-lang.org/tools/install) in order to compile it.

To build:
```
$ git clone git@github.com:nwj/zeke.git
$ cd zeke
$ cargo build --release
$ ./target/release/zeke --version
```
