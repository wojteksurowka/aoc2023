use aoc2023::{input_lines, regex_groups};

fn main() {
    part2()
}

pub fn part1() {
    println!("{}", read().into_iter().map(|s| hash(&s)).sum::<usize>());
}

pub fn part2() {
    let mut lenses: Vec<Vec<(String, usize)>> = Vec::new();
    lenses.resize(256, Vec::new());
    for lens in read() {
        let groups = regex_groups(r"([^-=]+)(.)(.*)", &lens);
        let index = hash(&groups[0]);
        if groups[1] == "-" {
            lenses[index] = lenses[index].iter().filter(|(s, _i)| s != &groups[0]).map(|(s, i)| (s.to_string(), *i)).collect();
        }
        else if groups[1] == "=" {
            let already = lenses[index].iter().filter(|(s, _i)| s == &groups[0]).count() > 0;
            let length = groups[2].parse::<usize>().unwrap();
            if already {
                lenses[index] = lenses[index].iter().map(|(s, i)| if s == &groups[0] {(s.to_string(), length)} else {(s.to_string(), *i)} ).collect();
            }
            else {
                lenses[index].push((groups[0].to_string(), length));
            }
        }
    }
    
    println!("{:?}", lenses.iter().enumerate().map(|(bi, seq)| seq.iter().enumerate().map(|(si, (_s, i))| (bi + 1) * (si + 1) * i).sum::<usize>()).sum::<usize>());
}

fn read() -> Vec<String> {
    input_lines(file!()).into_iter().next().unwrap().split(',').map(|s| s.to_string()).collect()
}

fn hash(s: &str) -> usize {
    s.chars().fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
}