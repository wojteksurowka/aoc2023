use std::path::Path;
use std::fs::read_to_string;

pub fn input_lines(filename: &str) -> Vec<String> {
    get_input_lines(filename).unwrap_or_default()
}

fn get_input_lines(filename: &str) -> Option<Vec<String>> {
    let path = Path::new(filename);
    let input_file = format!("input/{}.txt", path.file_stem()?.to_str()?);
    Some(read_to_string(input_file).ok()?.lines().map(String::from).collect())
}