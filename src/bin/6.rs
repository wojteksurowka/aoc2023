use aoc2023::{input_lines, regex_groups, integers};
use std::iter::zip;

fn main() {
    part2()
}

pub fn part1() {
    println!("{}", time_distance().into_iter().map(count_ways).product::<i64>())
}

pub fn part2() {
    println!("{:?}", count_ways(time_distance2()));
}

fn count_ways((t, d): (i64, i64)) -> i64 {
    let delta = t * t - 4 * d;
    if delta > 0 {
        let x1 = ((t as f64) - (delta as f64).sqrt()) / 2.0;
        let x2 = ((t as f64) + (delta as f64).sqrt()) / 2.0;
        let epsilon = 0.00000001;
        ((x2 - epsilon).floor() as i64) - ((x1 + epsilon).ceil() as i64) + 1
    }
    else {
        0
    }
}

fn time_distance() -> Vec<(i64, i64)> {
    let mut times: Vec<i64> = Vec::new();
    let mut distances: Vec<i64> = Vec::new();
    for line in input_lines(file!()) {
        let groups = regex_groups(r"(.+):(.+)", &line);
        if groups[0] == "Time" {
            times = integers(&groups[1]);
        }
        else if groups[0] == "Distance" {
            distances = integers(&groups[1]);
        }
    }
    zip(times, distances).collect()
}

fn time_distance2() -> (i64, i64) {
    let mut time: i64 = 0;
    let mut distance: i64 = 0;
    for line in input_lines(file!()) {
        let groups = regex_groups(r"(.+):(.+)", &line);
        if groups[0] == "Time" {
            time = groups[1].replace(" ", "").parse::<i64>().unwrap();
        }
        else if groups[0] == "Distance" {
            distance = groups[1].replace(" ", "").parse::<i64>().unwrap();
        }
    }
    (time, distance)
}
