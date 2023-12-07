use crate::utils;
use std::iter::Iterator;

#[derive(Eq, PartialEq, Copy, Clone)]
struct Card(u8);
impl Card {
    pub const JOLLY: Card = Card(b'J');

    fn is_jolly(&self) -> bool {
        self.0 == b'J'
    }

    fn value(&self, with_jolly: bool) -> u32 {
        match self.0 {
            b'A' => 14,
            b'K' => 13,
            b'Q' => 12,
            b'J' if with_jolly => 0,
            b'J' => 11,
            b'T' => 10,
            b'9' => 9,
            b'8' => 8,
            b'7' => 7,
            b'6' => 6,
            b'5' => 5,
            b'4' => 4,
            b'3' => 3,
            b'2' => 2,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = self.0 as char;
        write!(f, "{c}")
    }
}

impl From<u8> for Card {
    fn from(value: u8) -> Self {
        match value {
            b'A' | b'K' | b'Q' | b'J' | b'T' | b'9' | b'8' | b'7' | b'6' | b'5' | b'4' | b'3' | b'2' => Card(value),
            c => panic!("{c} is not a valid card"),
        }
    }
}


#[derive(Debug)]
struct Hand<const WITH_JOLLY: bool> {
    bid: u64,
    score: u32,
}

impl<const WITH_JOLLY: bool> Hand<WITH_JOLLY> {
    pub fn parse(s: String) -> Option<Self> {
        let b = s.as_bytes();
        let cards = [
            b[0].into(),
            b[1].into(),
            b[2].into(),
            b[3].into(),
            b[4].into(),
        ];
        let bid = s[6..].parse::<u64>().unwrap();
        let score = cards_score(&cards, WITH_JOLLY);

        Some(Self {
            bid,
            score,
        })
    }
}

fn cards_score(cards: &[Card; 5], with_jolly: bool) -> u32 {
    kind_score(cards, with_jolly) << 20 | cards[0].value(with_jolly) << 16 | cards[1].value(with_jolly) << 12 | cards[2].value(with_jolly) << 8 | cards[3].value(with_jolly) << 4 | cards[4].value(with_jolly)
}

fn kind_score(cards: &[Card; 5], with_jolly: bool) -> u32 {
    const FIVE_OF_A_KIND: u32 = 6;
    const FOUR_OF_A_KIND: u32 = 5;
    const FULL_HOUSE: u32 = 4;
    const THREE_OF_A_KIND: u32 = 3;
    const TWO_PAIRS: u32 = 2;
    const ONE_PAIR: u32 = 1;
    const NOTHING_RELEVANT: u32 = 0;

    let mut slots: [Option<(Card, u32)>; 5] = [None; 5];

    let mut jolly_count = 0;

    for card in cards {
        if with_jolly && card.is_jolly() {
            jolly_count += 1;
            continue;
        }

        for slot in slots.iter_mut() {
            match slot {
                &mut Some((c, n)) if c == *card => {
                    *slot = Some((*card, n + 1));
                    break;
                }
                None => {
                    *slot = Some((*card, 1));
                    break;
                }
                &mut Some(_) => (),
            }
        }
    }

    let mut slots = slots.iter().filter_map(|s| *s).collect::<Vec<_>>();
    slots.sort_by_key(|s| s.1);
    slots.reverse();

    if with_jolly {
        if slots.is_empty() {
            slots.push((Card::JOLLY, 5));
        } else {
            slots[0] = (slots[0].0, slots[0].1 + jolly_count);
        }
    }

    if slots.len() == 1 {
        println!("{cards:?} -> FIVE_OF_A_KIND ({slots:?})");
        FIVE_OF_A_KIND
    } else if slots[0].1 == 4 {
        println!("{cards:?} -> FOUR_OF_A_KIND ({slots:?})");
        FOUR_OF_A_KIND
    } else if slots[0].1 == 3 && slots[1].1 == 2 {
        println!("{cards:?} -> FULL_HOUSE ({slots:?})");
        FULL_HOUSE
    } else if slots[0].1 == 3 {
        println!("{cards:?} -> THREE_OF_A_KIND ({slots:?})");
        THREE_OF_A_KIND
    } else if slots[0].1 == 2 && slots[1].1 == 2 {
        println!("{cards:?} -> TWO_PAIRS ({slots:?})");
        TWO_PAIRS
    } else if slots[0].1 == 2 {
        println!("{cards:?} -> ONE_PAIR ({slots:?})");
        ONE_PAIR
    } else {
        println!("{cards:?} -> NOTHING_RELEVANT ({slots:?})");
        NOTHING_RELEVANT
    }
}


pub fn test1() {
    let mut hands = utils::parse_lines::<Hand::<false>>("data/day7.txt", Hand::<false>::parse);
    hands.sort_by_key(|h| h.score);

    let total = hands.iter().enumerate().map(|(i, h)| h.bid * (i as u64 + 1)).sum::<u64>();
    println!("{total}");
}

pub fn test2() {
    let mut hands = utils::parse_lines::<Hand::<true>>("data/day7.txt", Hand::<true>::parse);
    hands.sort_by_key(|h| h.score);

    let total = hands.iter().enumerate().map(|(i, h)| h.bid * (i as u64 + 1)).sum::<u64>();
    println!("{total}");
}
