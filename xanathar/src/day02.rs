use crate::utils;
use std::iter::Iterator;

#[derive(Default)]
struct Round {
    r: u32,
    g: u32,
    b: u32,
}

impl Round {
    fn parse(cubes: &[&str]) -> Self {
        let mut this = Self::default();

        for s in cubes.iter() {
            let (num, color) = s.split_once(' ').unwrap();
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

    fn possible_with(&self, avail: &Round) -> bool {
        self.r <= avail.r && self.g <= avail.g && self.b <= avail.b
    }
}

struct Game {
    rounds: Vec<Round>,
}

impl Game {
    fn parse(rounds: &[&[&str]]) -> Self {
        Self {
            rounds: rounds.iter().map(|r| Round::parse(r)).collect(),
        }
    }

    fn possible_with(&self, avail: &Round) -> bool {
        self.rounds.iter().all(|r| r.possible_with(avail))
    }
}

fn load_games() -> Vec<Game> {
    crate::data::data02::LINES.iter().map(|l| Game::parse(l)).collect()
}


pub fn test1() {
    let games = load_games();
    let avail = Round { r: 12, g: 13, b: 14 };

    let res = games.iter().enumerate().filter(|(_, g)| g.possible_with(&avail)).map(|(i, _)| i + 1).sum::<usize>();

    println!("{res}");
}

pub fn test2() {

}
