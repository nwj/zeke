# Unreleased

# v0.3.1

## Features

- Introduced the `graph` command, which graphs the links between notes and outputs a representation of that graph in the DOT format.
- File-writing commands (such as `tag` or `link`) will no longer strip unrecognized fields from the YAML front-matter of notes. This behavior better aligns with the aim of having these commands make the minimum necessary modification to files.

# v0.3.0

## Features

- __[Breaking Change]__ Changed the linking system from a directed system to an undirected system. This means that the `links_in` and `links_out` fields in note front-matter are no longer recognized and instead there is a single `links` field.

# v0.2.1

## Features

- Introduced the `tags` command.

# v0.2.0

## Features

- Introduced the `help`, `new`, `tag`, `untag`, `link`, and `unlink` commands.

# v0.1.0

- Initial release
