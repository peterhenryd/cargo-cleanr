# cargo-cleanr

Recursively cleans `target` directories in the current directory and all subdirectories.

Unlike `cargo clean`, which only works for a given Cargo project, `cargo cleanr` will walk the entire directory tree and run `cargo clean` in every Cargo project it finds. This is useful for cleaning up disk space across a folder containing multiple independent Cargo projects.

## Installation and Usage

```bash
git clone https://github.com/peterhenryd/cargo-cleanr
cd cargo-cleanr
cargo install --path .
cargo cleanr
```