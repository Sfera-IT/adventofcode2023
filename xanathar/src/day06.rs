use crate::utils;
use std::iter::Iterator;


pub fn wins(button: u64, time: u64, dist: u64) -> bool {
    (time - button) * button > dist
}

pub fn ways_to_win(time: u64, dist: u64) -> u64 {
    (1..time).filter(|n| wins(*n, time, dist)).count() as u64
}

pub fn test1() {
    const DATA: &[(u64, u64)] = &[(45, 295), (98, 1734), (83, 1278), (73, 1210)];

    let sol = DATA.iter().map(|t| ways_to_win(t.0, t.1)).product::<u64>();
    println!("{sol}");
}

pub fn test2() {
    let sol = ways_to_win(45988373, 295173412781210);
    println!("{sol}");
}
