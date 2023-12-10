use aoc2023::{input_lines, integers};

fn main() {
    part2()
}

pub fn part1() {
    let result: i32 = input_lines(file!()).iter().map(|l| predict(&l)).sum();
    println!("{result}");
}

pub fn part2() {
    let result: i32 = input_lines(file!()).iter().map(|l| predict2(&l)).sum();
    println!("{result}");
}

fn predict(line: &String) -> i32 {
    let mut rows: Vec<Vec<i32>> = Vec::new();
    rows.push(integers(line));
    for i in 0..rows[0].len() {
        let next = next_row(&rows[rows.len() - 1], rows[0].len() - i);
        if next.iter().all(|n| *n == 0) {
            break
        }
        rows.push(next);
    }
    let last_last = rows[0].len() - rows.len();
    let mut sum = 0;
    for i in last_last..rows[0].len() {
        let row_index = ((rows.len() + last_last) as i32 - i as i32 - 1) as usize;
        sum += rows[row_index][i];
    }
    sum
}

fn predict2(line: &String) -> i32 {
    let mut rows: Vec<Vec<i32>> = Vec::new();
    rows.push(integers(line));
    for i in 0..rows[0].len() {
        let next = next_row(&rows[rows.len() - 1], rows[0].len() - i);
        if next.iter().all(|n| *n == 0) {
            break
        }
        rows.push(next);
    }
    let last_last = rows[0].len() - rows.len();
    let mut sum = 0;
    for i in last_last..rows[0].len() {
        let row_index = ((rows.len() + last_last) as i32 - i as i32 - 1) as usize;
        sum = rows[row_index][0] - sum;
    }
    sum
}

fn next_row(row: &Vec<i32>, len: usize) -> Vec<i32> {
    let mut next: Vec<i32> = Vec::new();
    for i in 0..len - 1 {
        next.push(row[i + 1] - row[i])
    }
    next
}