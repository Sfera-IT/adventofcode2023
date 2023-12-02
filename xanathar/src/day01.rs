use crate::utils;
use std::iter::Iterator;


pub fn test1() {
    let lines = utils::read_lines("data/day1.txt");

    let sum = lines.map(|l| {
        let first_digit = l.chars().filter(|c| c.is_numeric()).map(|c| (c as u8) - b'0').next().unwrap();
        let last_digit = l.chars().filter(|c| c.is_numeric()).map(|c| (c as u8) - b'0').last().unwrap();
        (first_digit * 10 + last_digit) as u64
    }).sum::<u64>();

    println!("{sum}");
}

pub fn test2() {
    let lines = utils::read_lines("data/day1.txt");

    let sum = lines.map(|l| digits(&l)).sum::<u64>();
    println!("{sum}");
}

fn digit_at(s: &str) -> Option<u64> {
    if s.len() == 0 {
        return None;
    }

    let fc = s.chars().next().unwrap();

    if fc.is_numeric() {
        Some(((fc as u8) - b'0') as u64)
    } else if s.starts_with("one") {
        Some(1)
    } else if s.starts_with("two") {
        Some(2)
    } else if s.starts_with("three") {
        Some(3)
    } else if s.starts_with("four") {
        Some(4)
    } else if s.starts_with("five") {
        Some(5)
    } else if s.starts_with("six") {
        Some(6)
    } else if s.starts_with("seven") {
        Some(7)
    } else if s.starts_with("eight") {
        Some(8)
    } else if s.starts_with("nine") {
        Some(9)
    } else {
        None
    }
}

fn digits(s: &str) -> u64 {
    let mut first = None;
    let mut last = None;

    for i in 0..s.len() {
        if let Some(d) = digit_at(&s[i..]) {
            if first.is_none() {
                first = Some(d);
            }
            last = Some(d);
        }
    }

    match (first, last) {
        (Some(d), Some(u)) => d * 10 + u,
        _ => panic!("{s} => {first:?} - {last:?}"),
    }
}
