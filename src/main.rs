//! A simple file corroborator, makes sure that files in directory:
//! - have required header lines
//! - have required footer
//! - end without trailing spaces

use std::env;
use std::string::ParseError;

/// Init point for the program
///
/// # Panics
/// When faced with non-unicode strings
///
/// # Example
/// To check files in the bin directory with extension ps1, run:
///
/// ```file_corrab ./1bin .ps1```
///
fn main() -> Result((), ParseError) {
    let args: Vec<String> = env::args().collect();
    let dir = args.get(0)?;
}
