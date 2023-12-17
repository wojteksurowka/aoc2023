use aoc2023::read_vvc;
use std::collections::HashSet;
use std::cmp::max;

fn main() {
    part2()
}

pub fn part1() {
    let layout = read_vvc(file!());
    println!("{}", compute(&layout, ((-1, 0), (1, 0))));
}

pub fn part2() {
    let layout = read_vvc(file!());
    let mut max_result = 0;
    let width = layout[0].len() as i32;
    let height = layout.len() as i32;
    for y in 0..layout.len() {
        max_result = max(max_result, compute(&layout, ((-1, y as i32), (1, 0))));
        max_result = max(max_result, compute(&layout, ((width, y as i32), (-1, 0))));
    }
    for x in 0..width {
        max_result = max(max_result, compute(&layout, ((x, -1), (0, 1))));
        max_result = max(max_result, compute(&layout, ((x, height), (0, -1))));
    }
    println!("{max_result}");
}

fn compute(layout: &Vec<Vec<char>>, start: ((i32, i32), (i32, i32))) -> usize {
    let mut starts: Vec<((i32, i32), (i32, i32))> = vec![start];
    let mut starts_seen = HashSet::<((i32, i32), (i32, i32))>::new();
    let mut energized = HashSet::<(i32, i32)>::new();
    while let Some((mut position, mut direction)) = starts.pop() {
        let mut path: HashSet<((i32, i32), (i32, i32))> = HashSet::new();
        loop {
            position = next(position, direction);
            if !inside(position, &layout) {
                break;
            }
            energized.insert(position);
            direction = match get(position, &layout) {
                '|' =>
                    match direction {
                        (1, 0) | (-1, 0) => {
                            if !starts_seen.contains(&(position, (0, -1))) {
                                starts.push((position, (0, -1)));
                                starts_seen.insert((position, (0, -1)));
                            }
                            (0, 1)
                        },
                        _ => direction
                    },
                '-' =>
                    match direction {
                        (0, 1) | (0, -1) => {
                            if !starts_seen.contains(&(position, (-1, 0))) {
                                starts.push((position, (-1, 0)));
                                starts_seen.insert((position, (-1, 0)));
                            }
                            (1, 0)
                        },
                        _ => direction
                    },
                '/' =>
                    match direction {
                        (1, 0) => (0, -1),
                        (0, 1) => (-1, 0),
                        (-1, 0) => (0, 1),
                        (0, -1) => (1, 0),
                        _ => direction
                    },
                '\\' =>
                    match direction {
                        (1, 0) => (0, 1),
                        (0, 1) => (1, 0),
                        (-1, 0) => (0, -1),
                        (0, -1) => (-1, 0),
                        _ => direction
                    },
                _ =>
                    direction
            };
            if path.contains(&(position, direction)) {
                break;
            }
            path.insert((position, direction));
        }
    }
    energized.len()
}

fn get((x, y): (i32, i32), layout: &Vec<Vec<char>>) -> char {
    layout[y as usize][x as usize]
}

fn next((x, y): (i32, i32), (ox, oy): (i32, i32)) -> (i32, i32) {
    (x + ox, y + oy)
}

fn inside((x, y): (i32, i32), layout: &Vec<Vec<char>>) -> bool {
    x >= 0 && (x as usize) < layout[0].len() && y >= 0 && (y as usize) < layout.len()
}