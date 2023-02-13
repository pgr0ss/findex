# findex

[![Rust](https://github.com/pgr0ss/findex/actions/workflows/rust.yml/badge.svg)](https://github.com/pgr0ss/findex/actions/workflows/rust.yml)

Recursively index files and store results in a sqlite database.

Note that this is mostly a project to help me learn Rust.

## Usage

Current operations:
- Add files to the index db: `findex add --verbose <path>`
- Dump the contents: `findex dump --verbose`

And then you can query the db with sql, such as finding duplications:

```bash
sqlite3 -line files.db "select size, sha256, group_concat(filename) from files group by size, sha256 having count(*) > 1 order by size"
```

Help text:

```
% findex -h
Recursively index files and store results in a sqlite database

Usage: findex <COMMAND>

Commands:
  add   Adds files to the index
  dump  Dumps file information from the index
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Development

Normal cargo operations:
- Run tests: `cargo test`
- Run app: `cargo run -- add .`
- Build debug version: `cargo build`
- Build release version: `cargo build --release`

A few helpers in the `Makefile`.
