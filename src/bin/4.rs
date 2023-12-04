use aoc2023::{input_lines, regex_groups, integers};
use std::collections::{HashSet, HashMap};
use std::cmp::min;

fn main() {
    part2()
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

pub fn part2() {
    let mut matching: HashMap<usize, usize> = HashMap::new();
    let mut counts: HashMap<usize, usize> = HashMap::new();
    for line in input_lines(file!()) {
        let id_winnings_haves = regex_groups(r"Card +(\d+):([^|]+)\|(.+)$", &line);
        let id = id_winnings_haves[0].parse::<usize>().unwrap();
        let winning_set: HashSet<i32> = integers(&id_winnings_haves[1]).iter().cloned().collect();
        let haves = integers(&id_winnings_haves[2]);
        matching.insert(id, haves.iter().filter(|h| winning_set.contains(&h)).count());
        counts.insert(id, 1);
    }
    let total = matching.len();
    (1..=total).for_each(|id| {
        let nmatching = matching.get(&id).unwrap();
        let count = *counts.get(&id).unwrap();
        ((id + 1)..(min(id + 1 + *nmatching, total + 1))).for_each(|n| {counts.insert(n, *counts.get(&n).unwrap() + count);});
    });
    println!("{}", counts.values().sum::<usize>())
}
