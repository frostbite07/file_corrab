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
/// Function will return a Path to the required
/// directory and the extension requested
pub fn parse_args(args: Vec<String>) -> Result<(PathBuf, String, bool), Box<dyn Error>> {
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
    let ext = match args.get(2) {
        None => String::from("*"),
        Some(dir) => dir.to_owned(),
    };
    let recurse = match args.get(3) {
        None => true,
        Some(dir) => !dir.eq("norecurse")
    };
    Ok((dir_path, ext, recurse))
}


/// Returns all file paths in the passed directory with the required extension type
pub fn read_dir_files(
    dir_path: &PathBuf,
    extension_type: &str,
    recurse: bool
) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut app_files = vec![];
    for entry in fs::read_dir(dir_path)? {
        if let Ok(entry) = entry {
            if entry.metadata()?.permissions().readonly() {
                println!("Permissions denied: {}", entry.path().display());
                continue;
            }
            let path = entry.path();
            if path.is_dir() && recurse {
                let sub_files = read_dir_files(&path, &extension_type, true);
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
