use aoc2023::input_lines;
use aoc2023::regex_groups;
use std::cmp::max;

fn main() {
    part2()
}

struct CubeSet {
    red: usize,
    green: usize,
    blue: usize
}

struct Game {
    id: usize,
    sets: Vec<CubeSet>
}

fn update_cubeset(mut cs: CubeSet, s: &str) -> CubeSet {
    let groups = regex_groups(r" *(\d+) +([a-z]+) *", s);
    let count = groups[0].parse::<usize>().unwrap();
    if groups[1] == "red" {
        cs.red = count
    }
    else if groups[1] == "green" {
        cs.green = count
    }
    else if groups[1] == "blue" {
        cs.blue = count
    }
    cs
}

fn read_cubeset(s: &str) -> CubeSet {
    s.split(',').fold(CubeSet{red:0, green:0, blue: 0}, update_cubeset)
}

fn read_game(line: &String) -> Game {
    let groups = regex_groups(r"Game (\d+):(.*)", line);
    let sets = groups[1].split(';').map(|s| read_cubeset(s)).collect::<Vec<CubeSet>>();
    Game{id: groups[0].parse::<usize>().unwrap(), sets: sets}
}

fn ok_for_part1(cs: &CubeSet) -> bool {
    cs.red <= 12 && cs.green <= 13 && cs.blue <= 14
}

pub fn part1() {
    let games: Vec<Game> = input_lines(file!()).iter().map(read_game).collect();
    let sum: usize = games.iter().filter(|g| g.sets.iter().all(ok_for_part1)).map(|g| g.id).sum();
    println!("{:?}", sum)
}

fn power(game: &Game) -> usize {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for cs in &game.sets {
        red = max(red, cs.red);
        green = max(green, cs.green);
        blue = max(blue, cs.blue);
    }
    red * green * blue
}

pub fn part2() {
    let games: Vec<Game> = input_lines(file!()).iter().map(read_game).collect();
    let sum: usize = games.iter().map(power).sum();
    println!("{:?}", sum)
}
