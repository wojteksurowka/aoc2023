use aoc2023::{input_lines, regex_groups, integers};
use std::collections::HashSet;

fn main() {
    part1()
}

pub fn part1() {
    let mut sum = 0;
    for line in input_lines(file!()) {
        let winnings_haves = regex_groups(r"Card +\d+:([^|]+)\|(.+)$", &line);
        let winning_set: HashSet<i32> = integers(&winnings_haves[0]).iter().cloned().collect();
        let haves = integers(&winnings_haves[1]);
        let mut points = 0;
        for have in haves {
            if winning_set.contains(&have) {
                if points == 0 {
                    points = 1
                }
                else {
                    points *= 2
                }
            }
        }
        sum += points
    }
    println!("{sum}")
}
