use aoc2023::input_lines;

fn main() {
    part2()
}

pub fn part1() {
    let lines = input_lines(file!());
    let mut hands_bids = lines.iter().map(|l| l.split_once(' ').unwrap()).collect::<Vec<_>>();
    hands_bids.sort_by_cached_key(hand_sort_key);
    let result: usize = hands_bids.iter().map(|(_, b)| b.parse::<usize>().unwrap()).enumerate().map(|(i, b)| (i + 1) * b).sum();
    println!("{result}")
}

pub fn part2() {
    let lines = input_lines(file!());
    let mut hands_bids = lines.iter().map(|l| l.split_once(' ').unwrap()).collect::<Vec<_>>();
    hands_bids.sort_by_cached_key(hand_sort_key2);
    let result: usize = hands_bids.iter().map(|(_, b)| b.parse::<usize>().unwrap()).enumerate().map(|(i, b)| (i + 1) * b).sum();
    println!("{result}")
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    HighCard, OnePair, TwoPair, ThreeOfAKind, FullHouse, FourOfAKInd, FiveOfAKind
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct HandSortKey {
    hand_type: HandType,
    codes: Vec<usize>
}

fn card_code(card: char) -> usize {
    match card {
        'A' => 12,
        'K' => 11,
        'Q' => 10,
        'J' => 9,
        'T' => 8,
        _ => (card.to_digit(10).unwrap() - 2) as usize
    }
}

fn hand_sort_key((hand, _bid): &(&str, &str)) -> HandSortKey {
    let codes = hand.chars().map(card_code).collect();
    let hand_type = hand_type_from_codes(&codes);
    HandSortKey { hand_type, codes }
}

fn hand_type_from_codes(codes: &Vec<usize>) -> HandType {
    let mut counts: Vec<i32> = Vec::new();
    counts.resize(13, 0);
    codes.iter().fold(HandType::HighCard, |acc, c| {
        let new_count = counts[*c] + 1;
        counts[*c] = new_count;
        match new_count {
            1 =>
                match acc {
                    HandType::HighCard => HandType::HighCard,
                    _ => acc
                },
            2 =>
                match acc {
                    HandType::HighCard => HandType::OnePair,
                    HandType::OnePair => HandType::TwoPair,
                    HandType::ThreeOfAKind => HandType::FullHouse,
                    _ => acc
                },
            3 =>
                match acc {
                    HandType::OnePair => HandType::ThreeOfAKind,
                    HandType::TwoPair => HandType::FullHouse,
                    _ => acc
                },
            4 =>
                HandType::FourOfAKInd,
            5 =>
                HandType::FiveOfAKind,
            _ =>
                acc
        }
    })
}

fn card_code2(card: char) -> usize {
    match card {
        'A' => 12,
        'K' => 11,
        'Q' => 10,
        'J' => 0,
        'T' => 9,
        _ => (card.to_digit(10).unwrap() - 1) as usize
    }
}

fn hand_sort_key2((hand, _bid): &(&str, &str)) -> HandSortKey {
    let codes = hand.chars().map(card_code2).collect();
    let hand_type = hand_type_from_codes2(&codes);
    HandSortKey { hand_type, codes }
}

fn hand_type_from_codes2(codes: &Vec<usize>) -> HandType {
    (1..=12).map(|c| hand_type_from_codes(&codes.iter().map(|o| if *o == 0 {c} else {*o}).collect::<Vec<_>>())).max().unwrap()
}