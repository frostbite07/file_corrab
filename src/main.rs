//! A simple file corroborator, makes sure that files in directory:
//! - have required header lines
//! - have required footer
//! - end without trailing spaces
//!
//! # Example
//! To check files in the bin directory with extension ps1, run:
//!
//! ```file_corrab ./bin .ps1```
//!
//! First argument is mandatory and provides the path.
//! Leaving out the second argument will check all files in dir.

mod errors;

use crate::errors::ArgumentError;
use std::path::{Path, PathBuf};
use std::{env, fs};

/// Init point for the program
///
/// # Panics
/// When faced with non-unicode strings
///

fn main() -> Result<(), ArgumentError> {
    let args: Vec<String> = env::args().collect();
    let dir_path = match args.get(1) {
        None => {
            println!("usage requires at least target dir: file_corrab dir ext");
            return Err(ArgumentError::Missing);
        }
        Some(dir) => {
            let path = Path::new(dir);
            if path.exists() && path.is_dir() {
                PathBuf::from(path)
            } else {
                println!("dir does not exist");
                return Err(ArgumentError::Invalid);
            }
        }
    };
    let files = loop_through_dir(
        &dir_path,
        match args.get(2) {
            None => "*",
            Some(dir) => dir,
        },
    );
    Ok(())
}

/// Function is recursive and will return a vector of all the files in the
/// directory and all subdirectories with required extension
///
/// #### Arguments
/// * `dir_path`: The path to the directory to be searched
/// * `file_extension`: The file extension to be searched for
fn loop_through_dir(
    dir_path: &PathBuf,
    extension_type: &str,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut app_files = vec![];
    for entry in fs::read_dir(dir_path)? {
        if let Ok(entry) = entry {
            if entry.metadata()?.permissions().readonly() {
                println!("Permissions denied: {}", entry.path().display());
                continue;
            }
            let path = entry.path();
            if path.is_dir() {
                let sub_files = loop_through_dir(&path, &extension_type);
                if sub_files.is_ok() {
                    app_files.append(&mut sub_files?);
                }
            } else if extension_type.eq("*") {
                app_files.push(path);
            } else if path.to_str().unwrap().ends_with(extension_type) {
                println!("Pushing application: {:?}", path);
                app_files.push(path);
            }
        }
    }
    Ok(app_files)
}
