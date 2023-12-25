use aoc2023::{input_lines, regex_groups};
use std::collections::HashMap;

fn main() {
    part2()
}

pub fn part1() {
    let (workflows, parts) = read_data();
    let total: usize = parts.iter().filter(|p| run(p, &workflows)).map(ratings_total).sum();
    println!("{total}");
}

pub fn part2() {
    let (workflows, _parts) = read_data();
    let x_ranges = ranges(Rating::X, &workflows);
    let m_ranges = ranges(Rating::M, &workflows);
    let a_ranges = ranges(Rating::A, &workflows);
    let s_ranges = ranges(Rating::S, &workflows);
    let mut total = 0;
    for (x, xto) in &x_ranges {
        for (m, mto) in &m_ranges {
            for (a, ato) in &a_ranges {
                for (s, sto) in &s_ranges {
                    if run(&Part{x: *x, m: *m, a: *a, s: *s}, &workflows) {
                        total += (xto - x) * (mto - m) * (ato - a) * (sto - s);
                    }
                }
            }
        }
    }
    println!("{total}");
}

struct Workflow {
    rules: Vec<Rule>
}

#[derive(Eq, PartialEq)]
enum Rating { X, M, A, S}

struct Condition {
    rating: Rating,
    gt: bool,
    value: usize
}

struct Rule {
    condition: Option<Condition>,
    target: String
}

struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize
}

fn ranges(rating: Rating, workflows: &HashMap<String, Workflow>) -> Vec<(usize, usize)> {
    let mut stops: Vec<usize> = Vec::new();
    for workflow in workflows.values() {
        for rule in &workflow.rules {
            if !rule.condition.is_none() {
                let condition = rule.condition.as_ref().unwrap();
                if condition.rating == rating {
                    let value = if condition.gt { condition.value + 1} else { condition.value};
                    if value > 1 && value < 4001 {
                        stops.push(value as usize);
                    }
                }
            }
        }
    }
    stops.sort();
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    let mut previous = 1;
    for stop in stops {
        ranges.push((previous, stop));
        previous = stop;
    }
    ranges.push((previous, 4001));
    ranges
}

fn run(part: &Part, workflows: &HashMap<String, Workflow>) -> bool {
    let mut workflow = workflows.get("in").unwrap();
    loop {
        let next = run_workflow(part, workflow);
        match &*next {
            "A" => return true,
            "R" => return false,
            _ => {
                workflow = workflows.get(&next).unwrap();
            }
        }
    }
}

fn ratings_total(part: &Part) -> usize {
    part.x + part.m + part.a + part.s
}

fn run_workflow(part: &Part, workflow: &Workflow) -> String {
    for rule in &workflow.rules {
        if rule.condition.is_none() {
            return rule.target.to_string();
        }
        else {
            let condition = rule.condition.as_ref().unwrap();
            let rating = match condition.rating {
                Rating::X => part.x,
                Rating::M => part.m,
                Rating::A => part.a,
                Rating::S => part.s,
            };
            if !condition.gt && rating < condition.value {
                return rule.target.to_string();
            }
            else if condition.gt && rating > condition.value {
                return rule.target.to_string();
            }
        }
    }
    panic!("Out of rules");
}

fn read_data() -> (HashMap<String, Workflow>, Vec<Part>) {
    let mut ratings_part = false;
    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    let mut ratings: Vec<Part> = Vec::new();
    for line in input_lines(file!()) {
        if line.is_empty() {
            ratings_part = true;
        }
        else if ratings_part {
            let xmas = regex_groups(r"\{x=([0-9]+),m=([0-9]+),a=([0-9]+),s=([0-9]+)\}", &line);
            ratings.push(create_ratings(&xmas[0], &xmas[1], &xmas[2], &xmas[3]));
        }
        else {
            let nr = regex_groups(r"([a-z]+)\{(.+)\}", &line);
            workflows.insert(nr[0].to_string(), create_workflow(&nr[1]));
        }
    }
    (workflows, ratings)
}

fn create_workflow(rules_str: &str) -> Workflow {
    Workflow {rules: rules_str.split(",").map(create_rule).collect()}
}

fn create_rule(rule_str: &str) -> Rule {
    if rule_str.contains(":") {
        let ct = regex_groups(r"(.+):(.+)", rule_str);
        Rule {condition: Some(create_condition(&ct[0])), target: ct[1].to_string()}
    }
    else {
        Rule { condition: None, target: rule_str.to_string() }
    }
}

fn create_condition(condition_str: &str) -> Condition {
    let rov = regex_groups(r"([xmas])(<|>)([0-9]+)", condition_str);
    let rating = match &*rov[0] {
        "x" => Rating::X,
        "m" => Rating::M,
        "a" => Rating::A,
        "s" => Rating::S,
        _ => panic!("Must be x m a s")
    };
    Condition {rating, gt: rov[1] == ">", value: rov[2].parse::<usize>().unwrap()}
}

fn create_ratings(x_str: &str, m_str: &str, a_str: &str, s_str: &str) -> Part {
    Part {
        x: x_str.parse::<usize>().unwrap(),
        m: m_str.parse::<usize>().unwrap(),
        a: a_str.parse::<usize>().unwrap(),
        s: s_str.parse::<usize>().unwrap()
    }
}