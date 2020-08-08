# zeke

A CLI for managing a personal note archive in a future-resilient way.

## Usage

### Create a new note

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

Notes are markdown files, named using a date and a formatted version of the note title. Each note includes YAML front-matter with various meta information.

### Create a new note and open it in a text editor

```
$ export ZEKE_EDITOR="vim"
$ zeke new -e "My Note"
```
The `-e` flag will open the new note in whichever editor you specify using the `ZEKE_EDITOR` environment variable.

### Tag a note
```
$ zeke tag 20200502-my_note.md my-tag
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

### Link two notes
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
Links are bi-directional and are recorded in the front-matter of both notes.

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
