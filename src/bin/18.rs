use aoc2023::{input_lines, regex_groups};

fn main() {
    part2()
}

pub fn part1() {
    let (mut cx, mut cy): (i64, i64) = (0, 0);
    let mut area = 0;
    let mut border = 0;
    for line in input_lines(file!()) {
        let ds = regex_groups(r"^([RLUD]) (\d+)", &line);
        let len = ds[1].parse::<i64>().unwrap();
        let (nx, ny) = move_by((cx, cy), &ds[0], len);
        area += cx * ny - nx * cy;
        border += (nx - cx).abs() + (ny - cy).abs();
        (cx, cy) = (nx, ny);
    }
    println!("{}", (area + border) / 2 + 1)
}

pub fn part2() {
    let (mut cx, mut cy): (i64, i64) = (0, 0);
    let mut area = 0;
    let mut border = 0;
    for line in input_lines(file!()) {
        let ds = regex_groups(r"#([0-9a-f]{5})([0-3])", &line);
        let len = i64::from_str_radix(&ds[0], 16).unwrap();
        let (nx, ny) = move_by((cx, cy), &ds[1], len);
        area += cx * ny - nx * cy;
        border += (nx - cx).abs() + (ny - cy).abs();
        (cx, cy) = (nx, ny);
    }
    println!("{}", (area + border) / 2 + 1)
}

fn move_by((x, y): (i64, i64), dir: &str, len: i64) -> (i64, i64) {
    match dir {
        "L" | "2" => (x - len, y),
        "R" | "0" => (x + len, y),
        "U" | "3" => (x, y - len),
        "D" | "1" => (x, y + len),
        _ => (x, y)
    }
}