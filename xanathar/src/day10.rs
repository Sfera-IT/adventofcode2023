use bidivec::{BidiVec, BidiArray};

use crate::utils;
use std::iter::Iterator;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    fn offs(self) -> (isize, isize) {
        use Dir::*;
        match self {
            N => (0, -1),
            E => (1, 0),
            S => (0, 1),
            W => (-1, 0),
        }
    }
}


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Cell {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

impl Cell {
    fn from_dirs(d1: Dir, d2: Dir) -> Self {
        use Cell::*;
        use Dir::*;
        match (d1, d2) {
            (N, S) | (S, N) => NS,
            (E, W) | (W, E) => EW,
            (N, E) | (E, N) => NE,
            (N, W) | (W, N) => NW,
            (S, W) | (W, S) => SW,
            (S, E) | (E, S) => SE,
            (d1, d2) => panic!("Can't cell {d1:?} {d2:?}"),
        }
    }

    fn from_char(c: char) -> Self {
        match c {
            '|' => Self::NS,
            '-' => Self::EW,
            'L' => Self::NE,
            'J' => Self::NW,
            '7' => Self::SW,
            'F' => Self::SE,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => panic!(),
        }
    }

    fn has_dir(self, dir: Dir) -> bool {
        use Cell::*;
        use Dir::*;
        match (self, dir) {
            (NS, N) => true,
            (NS, S) => true,
            (EW, E) => true,
            (EW, W) => true,
            (NE, N) => true,
            (NE, E) => true,
            (NW, N) => true,
            (NW, W) => true,
            (SW, S) => true,
            (SW, W) => true,
            (SE, S) => true,
            (SE, E) => true,
            _ => false,
        }
    }

    fn next(self, from: Dir) -> Dir {
        use Cell::*;
        use Dir::*;
        match (self, from) {
            (NS, N) => S,
            (NS, S) => N,
            (EW, E) => W,
            (EW, W) => E,
            (NE, S) | (NE, N) => E,
            (NE, W) | (NE, E) => N,
            (NW, S) | (NW, N) => W,
            (NW, E) | (NW, W) => N,
            (SW, N) | (SW, S) => W,
            (SW, E) | (SW, W) => S,
            (SE, N) | (SE, S) => E,
            (SE, W) | (SE, E) => S,
            _ => panic!("Requesed next for cell {self:?} from {from:?}"),
        }
    }

    fn start_dir(self, backward: bool) -> Dir {
        use Cell::*;
        use Dir::*;
        match (self, backward) {
            (NS, true) => S,
            (NS, false) => N,
            (EW, true) => W,
            (EW, false) => E,
            (NE, true) => W,
            (NE, false) => S,
            (NW, true) => E,
            (NW, false) => S,
            (SW, true) => E,
            (SW, false) => N,
            (SE, true) => W,
            (SE, false) => N,
            _ => panic!(),
        }
    }
}

fn parse_line(s: String) -> Option<Vec<Cell>> {
    Some(s.chars().map(Cell::from_char).collect::<Vec<_>>())
}

fn fix_start(map: &mut BidiArray<Cell>, x: usize, y: usize) {
    let n = if y > 0 { map.get(x, y - 1).unwrap_or(&Cell::Ground) } else { &Cell::Ground };
    let s = map.get(x, y + 1).unwrap_or(&Cell::Ground);
    let w = if x > 0 { map.get(x -  1, y).unwrap_or(&Cell::Ground) } else { &Cell::Ground };
    let e = map.get(x + 1, y).unwrap_or(&Cell::Ground);

    let dir1 = if n.has_dir(Dir::S) {
        Dir::N
    } else if s.has_dir(Dir::N) {
        Dir::S
    } else if e.has_dir(Dir::W) {
        Dir::E
    } else if w.has_dir(Dir::E) {
        Dir::W
    } else {
        panic!()
    };
    let dir2 = if n.has_dir(Dir::S) && dir1 != Dir::N {
        Dir::N
    } else if s.has_dir(Dir::N) && dir1 != Dir::S {
        Dir::S
    } else if e.has_dir(Dir::W) && dir1 != Dir::E {
        Dir::E
    } else if w.has_dir(Dir::E) && dir1 != Dir::W {
        Dir::W
    } else {
        panic!()
    };
    map[(x, y)] = Cell::from_dirs(dir1, dir2);
}

fn walk_map(map: &BidiArray<Cell>, sx: usize, sy: usize, backwards: bool) -> BidiArray<u32> {
    let mut distmap = BidiArray::with_elem(0, map.width(), map.height());

    let (mut x, mut y, mut last_dir, mut count) = (sx, sy, map[(sx, sy)].start_dir(backwards), 0);

    println!("Start is {:?} at {sx},{sy}, last_dir is {last_dir:?}", map[(sx, sy)]);

    loop {
        let dir = map[(x, y)].next(last_dir);
        println!("Next dir in {:?}  coming from {last_dir:?} ({x},{y}), is {dir:?}", map[(x, y)]);

        last_dir = dir;

        let (dx, dy) = last_dir.offs();
        count += 1;

        x = (x as isize + dx) as usize;
        y = (y as isize + dy) as usize;

        if x == sx && y == sy {
            break;
        }

        distmap[(x, y)] = count;
    }

    distmap
}


pub fn test1() {
    let map = crate::utils::parse_lines("data/day10.txt", parse_line);
    let mut map = BidiArray::from_iterator(map.iter().flat_map(|l| l.iter().copied()), map[0].len()).unwrap();

    let (sx, sy) = map.iter().with_coords().find_map(|(x, y, i)| if *i == Cell::Start { Some((x, y)) } else { None }).unwrap();

    fix_start(&mut map, sx, sy);

    let dist1 = walk_map(&map, sx, sy, false);
    let dist2 = walk_map(&map, sx, sy, true);
    println!("{dist1:?}");
    println!("{dist2:?}");

    let v = dist2.iter().with_coords().map(|(x, y, i)| i.saturating_sub(dist1[(x, y)])).max();


    println!("{v:?}");
}

pub fn test2() {}
