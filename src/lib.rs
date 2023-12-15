use std::path::Path;
use std::fs::read_to_string;
use regex::Regex;
use std::str::FromStr;

pub fn input_lines(filename: &str) -> Vec<String> {
    get_input_lines(filename).unwrap_or_default()
}

fn get_input_lines(filename: &str) -> Option<Vec<String>> {
    let path = Path::new(filename);
    let input_file = format!("input/{}.txt", path.file_stem()?.to_str()?);
    Some(read_to_string(input_file).ok()?.lines().map(String::from).collect())
}

pub fn fold_lines_chars<T>(lines: &Vec<String>, init: T, mut f: impl FnMut(T, i32, i32, char) -> T) -> T {
    (0..lines.len()).fold(init, |y_acc, y| {
        let (line_result, _end) = (0..lines[y].len()).fold((y_acc, lines[y].chars()), |x_acc_it, x| {
            let (x_acc, mut it) = x_acc_it;
            (f(x_acc, x as i32, y as i32, it.next().unwrap()), it)
        });
        line_result
    })
}

pub fn regex_groups(regex: &str, input: &str) -> Vec<String> {
    let re = Regex::new(regex).unwrap();
    re.captures(input).map(|c| (1..c.len()).map(|i| c.get(i).unwrap().as_str().to_string()).collect()).unwrap()
}

pub fn integers<T: FromStr>(s: &str) -> Vec<T> {
    s.split(' ').filter(|n| n.len() > 0).filter_map(|n| n.trim().parse().ok()).collect()
}

pub fn transpose(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let width = matrix[0].len();
    let mut transposed: Vec<Vec<char>> = Vec::new();
    let mut row_to_clone: Vec<char> = Vec::new();
    row_to_clone.resize(matrix.len(), 0 as char);
    transposed.resize(width, row_to_clone);
    for y in 0..matrix.len() {
        for x in 0..width {
            transposed[x][y] = matrix[y][x];
        }
    }
    transposed
}
