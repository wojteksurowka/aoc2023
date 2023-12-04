use aoc2023::{input_lines, fold_lines_chars};
use std::collections::HashMap;

fn main() {
    part2()
}

pub fn part1() {
    let lines = input_lines(file!());
    let symbols: HashMap<(i32, i32), char> = fold_lines_chars(&lines, HashMap::new(), |mut acc, x, y, c| {
        if c != '.' && !c.is_digit(10) {
            acc.insert((x, y), c);
            acc
        }
        else {
            acc
        }
    });
    let any_in_symbols = |lst: Vec::<(i32, i32)>| lst.iter().any(|xy| symbols.contains_key(xy));
    let (numbers, _number, _curr_y, _adj) = fold_lines_chars(&lines, (Vec::<i32>::new(), 0, -1, false), |acc, x, y, c| {
        let (mut numbers, number, curr_y, adj) = acc;
        if c.is_digit(10) {
            if curr_y == y {
                if adj {
                    let li = numbers.len() - 1;
                    numbers[li] = numbers[li] * 10 + c.to_digit(10).unwrap() as i32;
                    (numbers, 0, curr_y, true)
                }
                else {
                    if any_in_symbols(vec![(x + 1, y - 1), (x + 1, y), (x + 1, y + 1)]) {
                        numbers.push(number * 10 + c.to_digit(10).unwrap() as i32);
                        (numbers, 0, curr_y, true)
                    }
                    else {
                        (numbers, number * 10 + c.to_digit(10).unwrap() as i32, curr_y, false)
                    }
                }
            }
            else {
                if any_in_symbols(vec![(x - 1, y - 1), (x - 1, y), (x - 1, y + 1), (x, y - 1), (x, y + 1), (x + 1, y - 1), (x + 1, y), (x + 1, y + 1)]) {
                    numbers.push(c.to_digit(10).unwrap() as i32);
                    (numbers, 0, y, true)
                }
                else {
                    (numbers, c.to_digit(10).unwrap() as i32, y, false)
                }
            }
        }
        else {
            (numbers, 0, -1, true)
        }
    });
    println!("{}", numbers.iter().sum::<i32>())
}

pub fn part2() {
    let lines = input_lines(file!());
    let mut gears: HashMap<(i32, i32), Vec<(i32, i32)>> = fold_lines_chars(&lines, HashMap::new(), |mut acc, x, y, c| {
        if c == '*' {
            acc.insert((x, y), vec![]);
            acc
        }
        else {
            acc
        }
    });
    let (numbers, _curr_x, _curr_y) = fold_lines_chars(&lines, (HashMap::<(i32, i32), i32>::new(), -1, -1), |acc, x, y, c| {
        let (mut numbers, curr_x, curr_y) = acc;
        if c.is_digit(10) {
            if curr_y == y {
                numbers.insert((curr_x, curr_y), numbers.get(&(curr_x, curr_y)).unwrap() * 10 + c.to_digit(10).unwrap() as i32);
                for (gx, gy) in [(x + 1, y - 1), (x + 1, y), (x + 1, y + 1)] {
                    if let Some(sxy) = gears.get_mut(&(gx, gy)) {
                        sxy.push((curr_x, curr_y));
                    }
                }
                (numbers, curr_x, curr_y)
            }
            else {
                numbers.insert((x, y), c.to_digit(10).unwrap() as i32);
                for (gx, gy) in [(x - 1, y - 1), (x - 1, y), (x - 1, y + 1), (x, y - 1), (x, y + 1), (x + 1, y - 1), (x + 1, y), (x + 1, y + 1)] {
                    if let Some(sxy) = gears.get_mut(&(gx, gy)) {
                        sxy.push((x, y));
                    }
                }
                (numbers, x, y)
            }
        }
        else {
            (numbers, -1, -1)
        }
    });
    println!("{:?}", gears.values().filter(|n| n.len() == 2).map(|n| numbers.get(&n[0]).unwrap() * numbers.get(&n[1]).unwrap()).sum::<i32>())
}
