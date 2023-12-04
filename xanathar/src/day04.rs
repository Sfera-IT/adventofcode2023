use crate::utils;
use std::{iter::Iterator, collections::HashSet};

fn score(card: &[&[u64]; 2]) -> u64 {
    let matches = count_matches(card);

    if matches == 0 {
        0
    } else {
        1 << (matches - 1)
    }
}

fn count_matches(card: &[&[u64]; 2]) -> usize {
    let left = card[0].iter().copied().collect::<HashSet<u64>>();
    let right = card[1].iter().copied().collect::<HashSet<u64>>();
    left.intersection(&right).count()
}

pub fn test1() {
    let data = crate::data::data04::DATA;
    let score: u64 = data.iter().map(|card| score(card)).sum();
    println!("{score}");
}


pub fn test2() {
    let data = crate::data::data04::DATA;
    let matches = data.iter().map(|card| count_matches(card)).collect::<Vec<_>>();
    let mut owned_cards = vec![1; matches.len()];

    for i in 0..matches.len() {
        let m = count_matches(&data[i]);

        for _ in 0..owned_cards[i] {
            for j in i+1..=i+m {
                owned_cards[j] += 1;
            }
        }
    }

    let owned_total: u64 = owned_cards.iter().sum();
    println!("{owned_total}")
}
