use std::path::Path;
use std::fs::read_to_string;
use regex::Regex;

pub fn input_lines(filename: &str) -> Vec<String> {
    get_input_lines(filename).unwrap_or_default()
}

fn get_input_lines(filename: &str) -> Option<Vec<String>> {
    let path = Path::new(filename);
    let input_file = format!("input/{}.txt", path.file_stem()?.to_str()?);
    Some(read_to_string(input_file).ok()?.lines().map(String::from).collect())
}

pub fn regex_groups(regex: &str, input: &str) -> Vec<String> {
    let re = Regex::new(regex).unwrap();
    re.captures(input).map(|c| (1..c.len()).map(|i| c.get(i).unwrap().as_str().to_string()).collect()).unwrap()
}
