use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::{Path, PathBuf};

/// Encapsulates arg related errors in the app
#[derive(Debug)]
pub enum ArgumentError {
    Missing,
    Invalid,
}

impl Display for ArgumentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} argument", self.to_string())
    }
}

impl Error for ArgumentError {}

/// Parses passed args and reads files accordingly.
/// Function will return a vector of all the files in the
/// directory and all subdirectories with required extension
pub fn read_files(args: Vec<String>) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let dir_path = match args.get(1) {
        None => {
            println!("usage requires at least target dir: file_corrab dir ext");
            return Err(Box::new(ArgumentError::Missing));
        }
        Some(dir) => {
            let path = Path::new(dir);
            if path.exists() && path.is_dir() {
                PathBuf::from(path)
            } else {
                println!("dir does not exist");
                return Err(Box::new(ArgumentError::Invalid));
            }
        }
    };
    loop_through_dir(
        &dir_path,
        match args.get(2) {
            None => "*",
            Some(dir) => dir,
        },
    )
}

fn loop_through_dir(
    dir_path: &PathBuf,
    extension_type: &str,
) -> Result<Vec<PathBuf>, Box<dyn Error>> {
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
            } else if extension_type.eq("*") || path.to_str().unwrap().ends_with(extension_type) {
                app_files.push(path);
            }
        }
    }
    Ok(app_files)
}
