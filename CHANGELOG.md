# Unreleased

# v0.6.0

## Features

- **[Breaking Change]** `tags`, `backlink`, and `mv` (when updating links to a moved note) now ignore notes that are hidden files, that match patterns in a `.ignore` file, or that match patterns in a `.gitignore` file (if running in a path that has a git repository).
- **[Breaking Change]** Removed the `graph` subcommand.
- `tags`, `backlinks`, and `mv` now perform some file I/O in parallel.
- `new -e` can now uses the `EDITOR` environment variable. If both `ZEKE_EDITOR` and `EDITOR` are set, then `ZEKE_EDITOR` is used.
- Editor commands (set via `ZEKE_EDITOR` or `EDITOR`) may now include arguments. Prior to this behavior, any provided arguments were discarded.

# v0.5.1

## Bug Fixes

- `mv` should no longer panic if it encounters a directory or fails to parse a file while checking other notes for references to the target note.

# v0.5.0

## Features

- **[Breaking Change]** Error output now includes more context and should be much easier to understand.

# v0.4.2

## Features

- Introduced the `backlink` command, which adds a reference to the `links` field in a note's front-matter for every other note that references it.

# v0.4.1

## Features

- Modified `new` and `mv` so that they strip punctuation from a note's title when generating the file name for a note.
- Modified `mv` so that it updates links in the markdown of other notes that reference the note that is being moved. Previously it only updated references in the `links` field of note front-matter.
- `graph` now considers links in the markdown of notes in the graph that it constructs. Previously it only looked at the `links` field in the front-matter of notes.

## Bug Fixes

- `unlink` and `graph` now recognize links like `./foo.md`, and `bar/../foo.md` as equivalent to `foo.md` and will act on those links accordingly.

# v0.4.0

## Features

- **[Breaking Change]** The `tag` and `untag` command now support tagging/untagging of multiple files - e.g. `tag <your tag> <file1> <file2> ...` is now possible. This is a breaking change because the order of the tag and file arguments on these commands has swapped.
- Introduced the `mv` command, which renames a note file and also updates any links that reference the old file name in the front-matter of other notes.

# v0.3.1

## Features

- Introduced the `graph` command, which graphs the links between notes and outputs a representation of that graph in the DOT format.
- File-writing commands (such as `tag` or `link`) will no longer strip unrecognized fields from the YAML front-matter of notes. This behavior better aligns with the aim of having these commands make the minimum necessary modification to files.

# v0.3.0

## Features

- **[Breaking Change]** Changed the linking system from a directed system to an undirected system. This means that the `links_in` and `links_out` fields in note front-matter are no longer recognized and instead there is a single `links` field.

# v0.2.1

## Features

- Introduced the `tags` command.

# v0.2.0

## Features

- Introduced the `help`, `new`, `tag`, `untag`, `link`, and `unlink` commands.

# v0.1.0

- Initial release
