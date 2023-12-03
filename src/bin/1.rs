use aoc2023::input_lines;

fn main() {
    part2()
}

pub fn part1() {
    let mut sum: usize = 0;
    for line in input_lines(file!()) {
        let trimmed = line.to_string().trim_matches(|c| !char::is_numeric(c)).to_string();
        sum += usize::from((trimmed.as_bytes()[0] - 48) * 10 + trimmed.as_bytes()[trimmed.len() - 1] - 48);
    }
    println!("{}", sum)
}

pub fn part2() {
    let names = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let values = (0..names.len()).map(|i| (names[i].to_string(), i + 1)).chain((0..10).map(|i| (i.to_string(), i))).collect::<Vec<(String, usize)>>();
    let mut sum: usize = 0;
    for line in input_lines(file!()) {
        let mut min_index = usize::MAX;
        let mut first_digit: usize = 0;
        let mut max_index = usize::MAX;
        let mut last_digit: usize = 0;
        for (name, v) in &values {
            if let Some(index) = line.to_string().find(name) {
                if index < min_index {
                    min_index = index;
                    first_digit = *v;
                }
            }
            if let Some(rindex) = line.to_string().rfind(name) {
                if max_index == usize::MAX || rindex > max_index {
                    max_index = rindex;
                    last_digit = *v;
                }
            }
        }
        sum += first_digit * 10 + last_digit;
    }
    println!("{}", sum)
}