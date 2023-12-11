use aoc2023::input_lines;
use std::collections::HashSet;

fn main() {
    part2()
}

pub fn part1() {
    let map = input_lines(file!());
    let start = find_s(&map);
    let mut from = start;
    let mut to = choose_next(start, &map);
    let mut steps = 1;
    while to != start {
        steps += 1;
        let next = step(from, to, &map);
        from = to;
        to = next;
    }
    println!("{}", steps / 2);
}

pub fn part2() {
    let mut map = input_lines(file!());
    let start = find_s(&map);
    let mut path: HashSet<(usize, usize)> = HashSet::new();
    let mut from = start;
    path.insert(from);
    let mut to = choose_next(start, &map);
    let (xstart, ystart) = start;
    let (xsecond, ysecond) = to;
    while to != start {
        path.insert(to);
        let next = step(from, to, &map);
        from = to;
        to = next;
    }
    let (xlast, ylast) = from;
    let mut for_start = ".";
    if xsecond == xstart && ysecond + 1 == ystart {
        if xlast == xstart + 1 && ylast == ystart {
            for_start = "L";
        }
        else if xlast == xstart && ylast == ystart + 1 {
            for_start = "|";
        }
        else if xlast + 1 == xstart && ylast == ystart {
            for_start = "J";
        }
    }
    else if xsecond == xstart + 1 && ysecond == ystart {
        if xlast == xstart && ylast == ystart + 1 {
            for_start = "F";
        }
        else if xlast + 1 == xstart && ylast == ystart {
            for_start = "-";
        }
        else if xlast == xstart && ylast + 1 == ystart {
            for_start = "L";
        }
    }
    else if xsecond == xstart && ysecond == ystart + 1 {
        if xlast + 1 == xstart && ylast == ystart {
            for_start = "7";
        }
        else if xlast == xstart && ylast + 1 == xstart {
            for_start = "|";
        }
        else if xlast == xstart + 1 && ylast == ystart {
            for_start = "F";
        }
    }
    else if xsecond + 1 == xstart && ysecond == ystart {
        if xlast == xstart && ylast + 1 == ystart {
            for_start = "J";
        }
        else if xlast == xstart + 1 && ylast == ystart {
            for_start = "-";
        }
        else if xlast == xstart && ylast == ystart + 1 {
            for_start = "7";
        }
    }
    map[ystart].replace_range(xstart..xstart + 1, for_start);
    println!("{}", area(&path, &map));
}

enum State { Outside, OnFromNorthOutside, OnFromSouthOutside, OnFromNorthInside, OnFromSouthInside, Inside }

fn area(path: &HashSet<(usize, usize)>, map: &Vec<String>) -> usize {
    let mut area = 0;
    for y in 0..map.len() {
        let mut state = State::Outside;
        for x in 0..map[y].len() {
            let tc = map[y].as_bytes()[x] as char;
            match &state {
                State::Outside =>
                    if path.contains(&(x, y)) {
                        match tc {
                            '|' => state = State::Inside,
                            'L' => state = State::OnFromNorthOutside,
                            'F' => state = State::OnFromSouthOutside,
                            _ => ()
                        }
                    }
                State::OnFromNorthOutside =>
                    match tc {
                        '-' => (),
                        'J' => state = State::Outside,
                        '7' => state = State::Inside,
                        _ => panic!("Cannot be 2")
                    }
                State::OnFromSouthOutside =>
                    match tc {
                        '-' => (),
                        'J' => state = State::Inside,
                        '7' => state = State::Outside,
                        _ => panic!("Cannot be 3")
                    }
                State::OnFromNorthInside =>
                    match tc {
                        '-' => (),
                        'J' => state = State::Inside,
                        '7' => state = State::Outside,
                        _ => panic!("Cannot be 2")
                    }
                State::OnFromSouthInside =>
                    match tc {
                        '-' => (),
                        'J' => state = State::Outside,
                        '7' => state = State::Inside,
                        _ => panic!("Cannot be 3")
                    }
                State::Inside =>
                    if path.contains(&(x, y)) {
                        match tc {
                            '|' => state = State::Outside,
                            'L' => state = State::OnFromNorthInside,
                            'F' => state = State::OnFromSouthInside,
                            '.' => (),
                            _ => panic!("Cannot be 1")
                        }
                    }
                    else {
                        area += 1;
                    }
            }
        }
    }
    area
}

fn step((xf, yf): (usize, usize), (xt, yt): (usize, usize), map: &Vec<String>) -> (usize, usize) {
    let tc = map[yt].as_bytes()[xt] as char;
    if xt == xf && yt + 1 == yf {
        match tc {
            '|' => (xt, yt - 1),
            '7' => (xt - 1, yt),
            'F' => (xt + 1, yt),
            _ => (xt, yt)
        }
    }
    else if xt == xf + 1 && yt == yf {
        match tc {
            '-' => (xt + 1, yt),
            'J' => (xt, yt - 1),
            '7' => (xt, yt + 1),
            _ => (xt, yt)
        }
    }
    else if xt == xf && yt == yf + 1 {
        match tc {
            '|' => (xt, yt + 1),
            'L' => (xt + 1, yt),
            'J' => (xt - 1, yt),
            _ => (xt, yt)
        }
    }
    else if xt + 1 == xf && yt == yf {
        match tc {
            '-' => (xt - 1, yt),
            'L' => (xt, yt - 1),
            'F' => (xt, yt + 1),
            _ => (xt, yt)
        }
    }
    else {
        (xt, yt)
    }
}

fn choose_next((x, y): (usize, usize), map: &Vec<String>) -> (usize, usize) {
    if y > 0 && can_go((x, y), (x, y - 1), map) {
        (x, y - 1)
    }
    else if x + 1 < map[y].len() && can_go((x, y), (x + 1, y), map) {
        (x + 1, y)
    }
    else if y + 1 < map.len() && can_go((x, y), (x, y + 1), map) {
        (x, y + 1)
    }
    else if x > 0 && can_go((x, y), (x - 1, y), map) {
        (x - 1, y)
    }
    else {
        (x, y)
    }
}

fn can_go(from: (usize, usize), to: (usize, usize), map: &Vec<String>) -> bool {
    let next = step(from, to, map);
    next != to
}

fn find_s(map: &Vec<String>) -> (usize, usize) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y].as_bytes()[x] as char == 'S' {
                return (x, y);
            }
        }
    }
    (0, 0)
}