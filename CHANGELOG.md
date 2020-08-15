# Unreleased

## Features

- Introduced the `graph` command, which graphs the links between notes and outputs a representation of that graph in the DOT format.

# v0.3.0

## Features

- *[Breaking Change]* Changed the linking system from a directed system to an undirected system. This means that the `links_in` and `links_out` fields in note front-matter are no longer recognized and instead there is a single `links` field.

# v0.2.1

## Features

- Introduced the `tags` command.

# v0.2.0

## Features

- Introduced the `help`, `new`, `tag`, `untag`, `link`, and `unlink` commands.

# v0.1.0

- Initial release
