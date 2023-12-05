use aoc2023::{input_lines, regex_groups, integers};

fn main() {
    part2()
}

pub fn part1() {
    let mut lines = input_lines(file!());
    let seeds = integers(&regex_groups(r"seeds: (.+)$", &lines.remove(0))[0]);
    let mut maps: Vec<Vec<(i64, i64, i64)>> = Vec::new();
    for line in lines {
        if line.ends_with("map:") {
            maps.push(Vec::new())
        }
        else if !line.is_empty() {
            let dsl = integers(&line);
            let last = maps.len() - 1;
            maps[last].push((dsl[0], dsl[1], dsl[2]));
        }
    }
    println!("{:?}", seeds.iter().map(|s| do_maps(*s, &maps)).min().unwrap());
}

pub fn part2() {
    let mut lines = input_lines(file!());
    let seeds = integers(&regex_groups(r"seeds: (.+)$", &lines.remove(0))[0]);
    let mut maps: Vec<Vec<(i64, i64, i64)>> = Vec::new();
    for line in lines {
        if line.ends_with("map:") {
            maps.push(Vec::new())
        }
        else if !line.is_empty() {
            let dsl = integers(&line);
            let last = maps.len() - 1;
            maps[last].push((dsl[0], dsl[1], dsl[2]));
        }
    }
    println!("{:?}", seeds.chunks(2).map(|c| (c[0]..c[0] + c[1]).map(|s| do_maps(s, &maps)).min().unwrap()).min().unwrap());
}

fn do_map(value: i64, map: &Vec<(i64, i64, i64)>) -> i64 {
    for (d, s, l) in map {
        if value >= *s && value < s + l {
            return d + value - s;
        }
    }
    return value;
}

fn do_maps(seed: i64, maps: &Vec<Vec<(i64, i64, i64)>>) -> i64 {
    maps.iter().fold(seed, do_map)
}