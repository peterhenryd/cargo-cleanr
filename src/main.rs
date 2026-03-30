use std::env::current_dir;
use std::fs::read_dir;
use std::path::PathBuf;
use std::process::Command;

fn main() -> std::io::Result<()> {
    cargo_clean(current_dir()?)
}

fn cargo_clean(path: PathBuf) -> std::io::Result<()> {
    if path.join("Cargo.toml").is_file() {
        Command::new("cargo")
            .arg("clean")
            .current_dir(path)
            .status()?;

        return Ok(());
    }

    for entry in read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_dir() {
            cargo_clean(entry_path)?;
        }
    }

    Ok(())
}