use crate::templates::{Template, TemplateType};
use regex::Regex;
use rev_buf_reader::RevBufReader;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::{Path, PathBuf};

/// Matches a file template with the passed challenge file
pub fn check_file(template: &Template, subject: &PathBuf) -> Vec<String> {
    let temp_file = read_lines(template.get_path());
    let sub_file = read_lines(subject);
    match template.get_kind() {
        TemplateType::Header => match_lines(
            BufReader::new(temp_file).lines(),
            BufReader::new(sub_file).lines(),
        ),
        TemplateType::Footer => match_lines(
            RevBufReader::new(temp_file).lines(),
            RevBufReader::new(sub_file).lines(),
        ),
    }
}

/// Tries to read a file via path
///
/// ## Panics
/// if file reading failed
fn read_lines<P>(filename: P) -> File
where
    P: AsRef<Path>,
{
    match File::open(&filename) {
        Ok(file) => file,
        Err(_) => {
            panic!("could not read file {}", filename.as_ref().display())
        }
    }
}

fn match_lines<T: Iterator<Item = Result<String>>>(template: T, mut sub: T) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for line in template {
        let sub_line = sub.next().unwrap().unwrap();
        if let Ok(ln) = line {
            if !Regex::new(&ln).unwrap().is_match(&sub_line) {
                result.push(format!("Mismatch: {} | {}", ln, sub_line));
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_matching() {
        let sub: Vec<Result<String>> = vec![Ok(String::from("Hello Murphy!"))];
        assert_eq!(
            match_lines(
                vec![Ok(String::from(r"Hello (?<name>\w+)!"))].into_iter(),
                sub.into_iter()
            )
            .get(0),
            None
        );

        let template = vec![Ok(String::from(r"Hello"))];
        let sub: Vec<Result<String>> = vec![Ok(String::from("Fail"))];
        assert_eq!(
            match_lines(template.into_iter(), sub.into_iter()).get(0),
            Some(&"Mismatch: Hello | Fail".to_string())
        );

        let template = vec![Ok(String::from(r"Hello")), Ok(String::from(r"Hello"))];
        let sub: Vec<Result<String>> = vec![Ok(String::from("Hello")), Ok(String::from("Fail"))];
        let result = match_lines(template.into_iter(), sub.into_iter());
        assert_eq!(
            result.get(0).unwrap(),
            &"Mismatch: Hello | Fail".to_string()
        );
    }
}
