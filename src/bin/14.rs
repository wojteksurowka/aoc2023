use aoc2023::input_lines;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

fn main() {
    part2()
}

pub fn part1() {
    let mut pattern = read();
    north(&mut pattern);
    println!("{:?}", load(&pattern));
}

pub fn part2() {
    let mut pattern = read();
    let mut hash_init: Option<u64> = None;
    let mut cycle_length = 0;
    let skip = 1000;
    for i in 0..1000000000 {
        if i == skip {
            hash_init = Some(hash(&pattern));
        }
        north(&mut pattern);
        west(&mut pattern);
        south(&mut pattern);
        east(&mut pattern);
        if i > skip && hash_init.unwrap() == hash(&pattern) {
            cycle_length = i + 1 - skip;
            break;
        }
    }
    pattern = read();
    for _i in 0..skip + (1000000000 - skip) % cycle_length {
        north(&mut pattern);
        west(&mut pattern);
        south(&mut pattern);
        east(&mut pattern);
    }
    println!("{:?}", load(&pattern));
}

fn hash(pattern: &Vec<Vec<char>>) -> u64 {
    let mut hash = DefaultHasher::new();
    pattern.hash(&mut hash);
    hash.finish()
}

fn read() -> Vec<Vec<char>> {
    let mut pattern: Vec<Vec<char>> = Vec::new();
    for line in input_lines(file!()) {
        pattern.push(line.chars().collect());
    }
    pattern
}

fn north(pattern: &mut Vec<Vec<char>>) {
    for x in 0..pattern[0].len() {
        let mut first_dot: Option<usize> = None;
        for y in 0..pattern.len() {
            match pattern[y][x] {
                '.' =>
                    if first_dot.is_none() {
                        first_dot = Some(y);
                    },
                'O' =>
                    if let Some(fd) = first_dot {
                        pattern[fd][x] = 'O';
                        pattern[y][x] = '.';
                        first_dot = Some(fd + 1);
                    },
                _ =>
                    first_dot = None,
            }
        }
    }
}

fn south(pattern: &mut Vec<Vec<char>>) {
    for x in 0..pattern[0].len() {
        let mut first_dot: Option<usize> = None;
        for y in (0..pattern.len()).rev() {
            match pattern[y][x] {
                '.' =>
                    if first_dot.is_none() {
                        first_dot = Some(y);
                    },
                'O' =>
                    if let Some(fd) = first_dot {
                        pattern[fd][x] = 'O';
                        pattern[y][x] = '.';
                        first_dot = Some(fd - 1);
                    },
                _ =>
                    first_dot = None,
            }
        }
    }
}

fn west(pattern: &mut Vec<Vec<char>>) {
    for y in 0..pattern.len() {
        let mut first_dot: Option<usize> = None;
        for x in 0..pattern[y].len() {
            match pattern[y][x] {
                '.' =>
                    if first_dot.is_none() {
                        first_dot = Some(x);
                    },
                'O' =>
                    if let Some(fd) = first_dot {
                        pattern[y][fd] = 'O';
                        pattern[y][x] = '.';
                        first_dot = Some(fd + 1);
                    },
                _ =>
                    first_dot = None,
            }
        }
    }
}

fn east(pattern: &mut Vec<Vec<char>>) {
    for y in 0..pattern.len() {
        let mut first_dot: Option<usize> = None;
        for x in (0..pattern[y].len()).rev() {
            match pattern[y][x] {
                '.' =>
                    if first_dot.is_none() {
                        first_dot = Some(x);
                    },
                'O' =>
                    if let Some(fd) = first_dot {
                        pattern[y][fd] = 'O';
                        pattern[y][x] = '.';
                        first_dot = Some(fd - 1);
                    },
                _ =>
                    first_dot = None,
            }
        }
    }
}

fn load(pattern: &Vec<Vec<char>>) -> usize {
    let mut total = 0;
    for y in 0..pattern.len() {
        total += (pattern.len() - y) * pattern[y].iter().filter(|c| **c == 'O').count()
    }
    total
}