use aoc2023::{input_lines, regex_groups};
use std::collections::HashMap;
use num_integer::lcm;

fn main() {
    part2()
}

pub fn part1() {
    let lines = input_lines(file!());
    let nodes = read_nodes(&lines);
    let mut node = "AAA";
    let mut count = 0;
    for dir in read_instructions(&lines).chars().cycle() {
        count += 1;
        let next_node = next(dir, &node, &nodes);
        if next_node == "ZZZ" {
            println!("{count}");
            break
        }
        node = next_node;
    }
}

pub fn part2() {
    let lines = input_lines(file!());
    let instructions = read_instructions(&lines);
    let nodes = read_nodes(&lines);
    let a_nodes = nodes.keys().filter(|n| n.ends_with("A")).map(|n| &**n);
    let result = a_nodes.map(|n| cycle_length(&n.to_string(), &instructions, &nodes)).fold(1, |acc, c| lcm(acc, c));
    println!("{result}");
}

fn cycle_length(node: &String, instructions: &String, nodes: &HashMap<String, (String, String)>) -> usize {
    let mut current: &str = node;
    let mut count = 0;
    let mut prev = 0;
    let mut diff = 0;
    for dir in instructions.chars().cycle() {
        count += 1;
        current = next(dir, current, &nodes);
        if current.ends_with("Z") {
            if count - prev == diff {
                return diff
            }
            diff = count - prev;
            prev = count;
        }
    }
    0
}

fn next<'nodes>(dir: char, node: &str, nodes: &'nodes HashMap<String, (String, String)>) -> &'nodes str {
    match dir {
        'L' =>
            {
                let (l, _r) = nodes.get(node).unwrap();
                l
            },
        'R' =>
            {
                let (_l, r) = nodes.get(node).unwrap();
                r
            },
        _ =>
            ""
    }
}

fn read_instructions(lines: &Vec<String>) -> String {
    lines[0].to_string()
}

fn read_nodes(lines: &Vec<String>) -> HashMap<String, (String, String)> {
    let mut nodes: HashMap<String, (String, String)> = HashMap::new();
    for line in lines.iter().skip(1) {
        if !line.is_empty() {
            let groups = regex_groups(r"([A-Z0-9]{3}) *= *\(([A-Z0-9]{3}), *([A-Z0-9]{3})\)", line);
            nodes.insert(groups[0].to_string(), (groups[1].to_string(), groups[2].to_string()));
        }
    }
    nodes
}