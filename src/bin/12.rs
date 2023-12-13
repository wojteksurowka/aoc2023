use aoc2023::input_lines;

fn main() {
    part1()
}

pub fn part1() {
    let result = read_data().iter().map(|(c, l)| search(c, l)).sum::<usize>();
    println!("{}", result)
}

fn search(s: &str, lengths: &[usize]) -> usize {
    let mut total = 0;
    let first_starts = search_one(s, lengths[0]);
    for start in first_starts {
        if lengths.len() == 1 {
            if s[start + lengths[0]..].chars().all(|c| c != '#') {
                total += 1
            }
        }
        else {
            if start + lengths[0] + 1 < s.len() {
                let rests = search(&s[start + lengths[0] + 1..], &lengths[1..]);
                total += rests;
            }
        }
    }
    total
}

fn search_one(s: &str, length: usize) -> Vec<usize> {
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