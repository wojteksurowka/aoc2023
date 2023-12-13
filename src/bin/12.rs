use aoc2023::input_lines;
use std::collections::HashMap;

fn main() {
    unsafe {
        part2()
    }
}

pub unsafe fn part1() {
    let result = read_data().iter().map(|(c, l)| search(c, 0, l, &mut HashMap::new())).sum::<usize>();
    println!("{}", result)
}

pub unsafe fn part2() {
    let result = read_data().iter().map(|(c, l)| search(&duplicate_string_5(c.to_string()), 0, &duplicate_5(l.to_vec()), &mut HashMap::new())).sum::<usize>();
    println!("{}", result)
}

fn duplicate_string_5(mut s: String) -> String {
    println!("HERE");
    s.push('?');
    let mut result = s.repeat(5);
    result.truncate(result.len() - 1);
    result
}

fn duplicate_5(v : Vec<usize>) -> Vec<usize> {
    [&v[..], &v[..], &v[..], &v[..], &v[..]].concat()
}

unsafe fn search<'lens>(s: &str, offset: usize, lengths: &'lens [usize], cache: &mut HashMap<(usize, &'lens [usize]), usize>) -> usize {
    match cache.get(&(offset, lengths)) {
        Some(result) =>
            *result,
        None => {
            let mut total = 0;
            let starts = search_one_sum(&s[offset..], lengths[0]);
            for start in starts {
                if lengths.len() == 1 {
                    if s[offset + start + lengths[0]..].chars().all(|c| c != '#') {
                        total += 1
                    }
                }
                else {
                    if offset + start + lengths[0] + 1 < s.len() {
                        total += search(s, offset + start + lengths[0] + 1, &lengths[1..], cache)
                    }
                }
            }
            cache.insert((offset, lengths), total);
            total
        }
    }
}

fn search_one_sum(s: &str, length: usize) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    if length <= s.len() {
        for i in 0..=s.len() - length as usize {
            if s[0..i].chars().all(|c| c != '#') {
                if starts_with_hashqm(&s[i..], length) {
                    result.push(i);
                }
            }
        }
    }
    result
}

fn starts_with_hashqm(s: &str, length: usize) -> bool {
    s[0..length].chars().all(|c| c == '#' || c == '?') && (s.len() == length || s[length..=length].chars().all(|c| c == '.' || c == '?'))
}

fn read_data() -> Vec<(String, Vec<usize>)> {
    input_lines(file!()).iter().map(|l| l.split_at(l.find(' ').unwrap())).
        map(|(c, l)| (c.to_string(), l.trim().split(',').map(|n| n.parse::<usize>().unwrap()).collect::<Vec<_>>())).
        collect::<Vec<_>>()
}