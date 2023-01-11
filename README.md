# findex

Recursively index files and store results in a sqlite database.

Note that this is mostly a project to help me learn Rust.

## Usage

Current operations:
- Add files to the index db: `findex --verbose add <path>`
- Dump the contents: `findex --verbose dump`

And then you can query the db with sql, such as finding duplications:

```bash
sqlite3 -line files.db "select size, sha256, group_concat(filename) from files group by size, sha256 having count(*) > 1 order by size"
```

Help text:

```
% findex -h
Recursively index files and store results in a sqlite database

Usage: findex [OPTIONS] <COMMAND>

Commands:
  add   Adds files to the index
  dump  Dumps file information from the index
  help  Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose
  -h, --help     Print help information
  -V, --version  Print version information
```

## Development

Normal cargo operations:
- Run tests: `cargo test`
- Run app: `cargo run -- add .`
- Build debug version: `cargo build`
- Build release version: `cargo build --release`

A few helpers in the `Makefile`.
