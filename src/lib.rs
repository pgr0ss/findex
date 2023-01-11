use camino::Utf8PathBuf;
use color_eyre::eyre::{self, eyre};
use rusqlite::Connection;
use sha256::try_digest;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

pub fn add(verbose: bool, path: &str) -> eyre::Result<()> {
    let conn = Connection::open("files.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS files (
            filename TEXT NOT NULL PRIMARY KEY,
            sha256 TEXT NOT NULL,
            size integer NOT NULL
        )",
        (),
    )?;

    let mut insert_statement =
        conn.prepare("INSERT OR REPLACE INTO files (filename, sha256, size) VALUES (?, ?, ?)")?;

    eprintln!("Recursively indexing path: {}", path);

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| !ignore_entry(e))
    {
        let path_buf = entry?.into_path();

        if path_buf.is_file() {
            let full_path = full_path(path_buf)?;
            let sha256 = try_digest(full_path.as_std_path())?;
            let size = full_path.metadata()?.len().to_string();

            if verbose {
                eprintln!("  {}, {}, {}", full_path, sha256, size);
            }

            insert_statement.execute([full_path.as_str(), sha256.as_str(), size.as_str()])?;
        }
    }

    Ok(())
}

pub fn dump(verbose: bool) -> eyre::Result<()> {
    let entries = query()?;

    for entry in entries {
        if verbose {
            println!("{:?}", entry);
        } else {
            println!("{}", entry.filename);
        }
    }
    Ok(())
}

#[derive(Debug)]
pub struct Entry {
    pub filename: String,
    pub sha256: String,
    pub size: u64,
}

pub fn query() -> eyre::Result<Vec<Entry>> {
    let conn = Connection::open("files.db")?;

    let mut stmt = conn
        .prepare("SELECT filename, sha256, size FROM files")
        .map_err(|e| eyre!("Unable to prepare statement: {:?}", e))?;

    let entry_iter = stmt.query_map([], |row| {
        Ok(Entry {
            filename: row.get(0)?,
            sha256: row.get(1)?,
            size: row.get(2)?,
        })
    })?;

    let mut entries = Vec::new();

    for entry in entry_iter {
        entries.push(entry?);
    }

    Ok(entries)
}

const IGNORE_PATTERNS: [&str; 4] = [".git", ".svn", "target", "vendor"];

fn ignore_entry(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| IGNORE_PATTERNS.contains(&s))
        .unwrap_or(false)
}

fn full_path(path_buf: PathBuf) -> eyre::Result<Utf8PathBuf> {
    Utf8PathBuf::from_path_buf(path_buf)
        .map_err(|e| eyre!("Unable to create path: {:?}", e))?
        .canonicalize_utf8()
        .map_err(|e| eyre!("unable to canonicalize path: {:?}", e))
}
