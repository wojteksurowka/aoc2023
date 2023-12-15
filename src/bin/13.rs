use aoc2023::{input_lines, transpose};
use std::cmp::min;

fn main() {
    part2()
}

pub fn part1() {
    let mut sum = 0;
    let mut pattern: Vec<Vec<char>> = Vec::new();
    for line in input_lines(file!()) {
        if line.is_empty() {
            sum += process_pattern(&pattern, usize::MAX).unwrap();
            pattern.clear();
        }
        else {
            pattern.push(line.chars().collect());
        }
    }
    sum += process_pattern(&pattern, usize::MAX).unwrap();
    println!("{sum}");
}

pub fn part2() {
    let mut sum = 0;
    let mut pattern: Vec<Vec<char>> = Vec::new();
    for line in input_lines(file!()) {
        if line.is_empty() {
            sum += process_pattern_2(&mut pattern);
            pattern.clear();
        }
        else {
            pattern.push(line.chars().collect());
        }
    }
    sum += process_pattern_2(&mut pattern);
    println!("{sum}");
}

fn process_pattern_2(pattern: &mut Vec<Vec<char>>) -> usize {
    let no_smudge = process_pattern(pattern, usize::MAX).unwrap();
    for y in 0..pattern.len() {
        for x in 0..pattern[y].len() {
            let previous_char = pattern[y][x];
            let new_char = match previous_char {
                '.' => '#',
                _ => '.'
            };
            pattern[y][x] = new_char;
            if let Some(r) = process_pattern(pattern, no_smudge) {
                return r;
            }
            pattern[y][x] = previous_char;
        }
    }
    usize::MAX
}

fn process_pattern(pattern: &Vec<Vec<char>>, skip: usize) -> Option<usize> {
    let mut horizontal_skip = usize::MAX;
    if skip >= 100 {
        horizontal_skip = skip / 100;
    }
    if let Some(h) = find_horizontal(pattern, horizontal_skip) {
        if h * 100 != skip {
            return Some(h * 100);
        }
    }
    let transposed = transpose(pattern);
    if let Some(v) = find_horizontal(&transposed, skip) {
        if v != skip {
            return Some(v);
        }
    }
    return None;
}

fn find_horizontal(pattern: &Vec<Vec<char>>, skip: usize) -> Option<usize> {
    for i in 1..pattern.len() {
        let range = min(i, pattern.len() - i);
        let mut checks = 0;
        for j in 0..range {
            let i1 = i - j - 1;
            let i2 = i + j;
            if pattern[i1] == pattern[i2] {
                checks += 1;
            }
            else {
                break;
            }
        }
        if checks == range && i != skip {
            return Some(i);
        }
    }
    return None;
}