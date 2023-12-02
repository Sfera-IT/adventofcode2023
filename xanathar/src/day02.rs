use crate::utils;
use std::iter::Iterator;

#[derive(Default)]
struct Set {
    r: u32,
    g: u32,
    b: u32,
}

impl Set {
    fn parse(cubesets: &[&str]) -> Self {
        let mut this = Self::default();

        for cs in cubesets.iter() {
            let (num, color) = cs.split_once(' ').unwrap();
            let num = num.parse::<u32>().unwrap();

            match color {
                "red" => this.r = num,
                "green" => this.g = num,
                "blue" => this.b = num,
                _ => panic!("'{color}' is not a color"),
            }
        }

        this
    }

    fn possible_with(&self, avail: &Set) -> bool {
        self.r <= avail.r && self.g <= avail.g && self.b <= avail.b
    }

    fn power(&self) -> u32 {
        self.r * self.g * self.b
    }
}

struct Game {
    sets: Vec<Set>,
}

impl Game {
    fn parse(sets: &[&[&str]]) -> Self {
        Self { sets: sets.iter().map(|s| Set::parse(s)).collect() }
    }

    fn possible_with(&self, avail: &Set) -> bool {
        self.sets.iter().all(|s| s.possible_with(avail))
    }

    fn max_set(&self) -> Set {
        Set {
            r: self.sets.iter().map(|s| s.r).max().unwrap(),
            g: self.sets.iter().map(|s| s.g).max().unwrap(),
            b: self.sets.iter().map(|s| s.b).max().unwrap(),
        }
    }
}

fn load_games() -> Vec<Game> {
    crate::data::data02::LINES.iter().map(|l| Game::parse(l)).collect()
}

pub fn test1() {
    let games = load_games();
    let avail = Set { r: 12, g: 13, b: 14 };

    let res = games
        .iter()
        .enumerate()
        .filter(|(_, g)| g.possible_with(&avail))
        .map(|(i, _)| i + 1)
        .sum::<usize>();

    println!("{res}");
}

pub fn test2() {
    let games = load_games();

    let res = games.iter().map(|g| g.max_set().power()).sum::<u32>();

    println!("{res}");
}
