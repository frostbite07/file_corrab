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


mod interface;
use crate::interface::read_files;

/// Init point for the program
///
/// # Panics
/// When faced with non-unicode strings and unexpected argument structure

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let files_to_corrab = read_files(std::env::args().collect())?;
    println!("{:?}", files_to_corrab);
    Ok(())
}