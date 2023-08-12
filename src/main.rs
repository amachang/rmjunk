//! rmjunk is a tool to remove junk files such as `.DS_Store` and `Thumbs.db` from directories.
//! 
//! # Features
//! 
//! - Removes junk files from a specified directory.
//! - Option to search and remove junk files recursively within directories.
//! - Offers a dry-run mode to preview files that will be removed without actually deleting them.
//! 
//! # Installation
//! 
//! ```bash
//! cargo install rmjunk
//! ```
//! 
//! # Usage
//! Basic usage:
//! 
//! ```bash
//! rmjunk [DIR]
//! ```
//! 
//! # Options:
//! 
//! - `-r`, `--recursive`: Search and remove junk files from directories recursively.
//! - `--dry-run`: Preview files that will be removed without actually deleting them.
//! 
//! # Examples
//! 
//! Remove junk files from a specific directory:
//! 
//! ```bash
//! rmjunk ./my_folder
//! ```
//! 
//! Search and remove files recursively:
//! 
//! ```bash
//! rmjunk -r ./my_folder
//! ```
//! 
//! Use the dry-run mode:
//! 
//! ```bash
//! rmjunk --dry-run ./my_folder
//! ```

use std::{
    io,
    fs,
    path::{
        PathBuf,
        Path,
    },
    error::Error,
};
use clap::Parser;
use junk_file::is_junk;

/// Remove junk files such as .DS_Store and Thumbs.db in a directory.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Remove junk files in this dir.
    #[arg()]
    dir: PathBuf,

    /// Remove all descendant junk files in the dir.
    #[arg(long, short = 'r')]
    recursive: bool,

    /// Dry run
    #[arg(long)]
    dry_run: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    remove_junk_files_in_dir(args.dir, args.recursive, args.dry_run)?;
    Ok(())
}

fn remove_junk_files_in_dir(dir: impl AsRef<Path>, recursive: bool, dry_run: bool) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let is_dir = entry.file_type()?.is_dir();

        if is_junk(entry.file_name()) {
            if dry_run {
                println!("Removed: {}", entry.path().display());
            } else {
                let result = if is_dir {
                    fs::remove_dir_all(entry.path())
                } else {
                    fs::remove_file(entry.path())
                };
                if let Err(err) = result {
                    eprintln!("Failed removal ({}): {}", err, entry.path().display());
                }
            }
        } else {
            if recursive && is_dir {
                remove_junk_files_in_dir(entry.path(), recursive, dry_run)?
            }
        }
    }
    Ok(())
}

