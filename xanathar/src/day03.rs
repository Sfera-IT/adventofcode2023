use bidivec::{BidiVec, BidiRect};

use crate::utils;
use std::{iter::Iterator, collections::{HashMap, hash_map::Entry}};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Cell {
    Empty,
    Symbol,
    Gear,
    Digit(u32, bool, Option<(usize, usize)>),
}

#[derive(Debug)]
struct Number {
    val: u32,
    geared: Option<(usize, usize)>,
    adjacent: bool,
}

impl Cell {
    fn from_char(c: char) -> Self {
        let c = c as u8;

        match c {
            b'0'..=b'9' => Self::Digit((c - b'0') as u32, false, None),
            b'.' => Self::Empty,
            b'*' => Self::Gear,
            _ => Self::Symbol,
        }
    }

    fn mark_adjacent(&mut self) {
        match self {
            Self::Digit(_, adj, _) => *adj = true,
            _ => (),
        }
    }

    fn mark_geared(&mut self, x: usize, y: usize) {
        match self {
            Self::Digit(_, _, geared) => *geared = Some((x, y)),
            _ => (),
        }
    }
}

fn load() -> Vec<Number> {
    let lines: Vec<_> = utils::read_lines("data/day3.txt").collect();
    let mut map = BidiVec::from_iterator(lines.iter().flat_map(|l| l.chars()).map(|c| Cell::from_char(c)), lines[0].len()).unwrap();
    let mut numbers = Vec::<Number>::new();

    for y in 0..map.height() {
        for x in 0..map.width() {
            if map[(x, y)] == Cell::Symbol {
                for i in map.iter_mut().on_neighbours(x, y, bidivec::BidiNeighbours::Bordering) {
                    i.mark_adjacent();
                }
            }
            if map[(x, y)] == Cell::Gear {
                for i in map.iter_mut().on_neighbours(x, y, bidivec::BidiNeighbours::Bordering) {
                    i.mark_geared(x, y);
                }
            }
        }
    }

    for y in 0..map.height() {
        for x in 0..map.width() {
            if matches!(map[(x, y)], Cell::Digit(_, _, _)) && (x + 1 >= map.width() || !matches!(map[(x + 1, y)], Cell::Digit(_, _, _))) {
                let mut val = 0;
                let mut exp = 1;
                let mut adjacent = false;
                let mut geared = None;
                let mut xx = x;

                while let Cell::Digit(v, adj, gear) = map[(xx, y)] {
                    val += v * exp;
                    exp *= 10;

                    if xx == 0 {
                        break;
                    }
                    xx -= 1;
                    adjacent |= adj;
                    geared = match (geared, gear) {
                        (None, v) => v,
                        (Some(_), None) => geared,
                        (Some(p1), Some(p2)) if p1 == p2 => geared,
                        (Some(p1), Some(p2)) => panic!("num at {x},{y} is double geared? ({p1:?} -- {p2:?})"),
                    }
                }

                numbers.push(Number { val, geared, adjacent });
            }
        }
    }

    numbers
}

pub fn test1() {
    println!("{}", load().iter().filter(|n| n.adjacent || n.geared.is_some()).map(|n| n.val).sum::<u32>());
}

pub fn test2() {
    let mut gears = HashMap::<(usize, usize), Vec<Number>>::new();
    let mut numbers = load();
    let mut sum = 0;

    for n in numbers.drain(..) {
        match n.geared {
            None => (),
            Some((gx, gy)) => {
                match gears.entry((gx, gy)) {
                    Entry::Occupied(mut v) => v.get_mut().push(n),
                    Entry::Vacant(e) => { e.insert(vec![n]); },
                }
            }
        }
    }

    for (_, v) in gears.drain() {
        match v.len() {
            0 => unreachable!(),
            1 => (),
            2 => sum += v[0].val * v[1].val,
            _ => panic!("many geared numbers!"),
        }
    }

    println!("{sum}");
}
