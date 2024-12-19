//! A simple file corroborator, makes sure that files in directory:
//! - have required header lines
//! - have required footer
//! - end without trailing spaces
//! ```file_corrab <dir> <ext> <?norecurse>```
//! # Example
//! To check files in the bin directory with extension ps1, run:
//!
//! ```file_corrab ./bin .ps1```
//!
//! First argument is mandatory and provides the path.
//! Leaving out the second argument will check all files in dir.

mod interface;
mod matcher;
mod templates;

use crate::interface::{parse_args, read_dir_files};
use crate::matcher::check_file;
use crate::templates::get_templates;

/// Init point for the program
///
/// # Panics
/// When faced with non-unicode strings and unexpected argument structure

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (dir_path, ext, recurse) = parse_args(std::env::args().collect())?;
    let templates = get_templates(&ext).expect("no templates found in /cfg");
    let subjects = read_dir_files(&dir_path, &ext, recurse).expect("no target files matching extension");
    for sub in subjects {
        let mut problems: Vec<String> = Vec::new();
        for temp in templates.iter() {
            problems.append(&mut check_file(temp, &sub));
        }
        if problems.len() > 0 {
            println!("{}", sub.as_os_str().to_str().unwrap());
            for warn in problems {
                println!("{}", warn);
            }
            println!();
        }
    }
    Ok(())
}
