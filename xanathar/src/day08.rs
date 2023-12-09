use crate::utils;
use std::{iter::Iterator, collections::{VecDeque, HashSet}};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Dir
{
    Left,
    Right
}

const fn alpha_to_num(s: &str) -> usize {
    let b = s.as_bytes();
    ((b[0] - b'A') as usize) << 10 | ((b[1] - b'A') as usize) << 5 | (b[2] - b'A') as usize
}

const MAX_ALPHA: usize = alpha_to_num("ZZZ") + 1;
const ZZZ_INDEX: usize = alpha_to_num("ZZZ");
const AAA_INDEX: usize = alpha_to_num("AAA");

fn ends_with_a(n: usize) -> bool {
    n & 31 == 0
}

fn ends_with_z(n: usize) -> bool {
    n & 31 == (b'Z' - b'A') as usize
}

fn read_data(mut lines: VecDeque<String>) -> (Vec<Dir>, Vec<(usize, usize)>, Vec<usize>) {
    let dirs: Vec<_> = lines.pop_front().unwrap().bytes().map(|c| if c == b'L' {
        Dir::Left
    } else if c == b'R' {
        Dir::Right
    } else {
        panic!("{c} in dir");
    }).collect();


    let mut places = vec![(0usize, 0usize); MAX_ALPHA];
    let mut a_places = Vec::new();

    while let Some(l) = lines.pop_front() {
        let tokens = l.split(' ').collect::<Vec<_>>();
        let (i, l, r) = (alpha_to_num(tokens[0]), alpha_to_num(tokens[1]), alpha_to_num(tokens[2]));

        places[i] = (l, r);

        if ends_with_a(i) {
            a_places.push(i);
        }
    }

    (dirs, places, a_places)
}

fn solve<F>(mut pos: usize, dirs: &[Dir], places: &[(usize, usize)], done: F) -> (usize, u64) 
where F: Fn(usize) -> bool
{
    let mut dir_idx = 0;
    let mut count = 0;

    loop {
        if dirs[dir_idx] == Dir::Left {
            pos = places[pos].0;
        } else {
            pos = places[pos].1;
        }
        dir_idx = (dir_idx + 1) % dirs.len();
        count += 1;

        if done(pos) {
            return (pos, count);
        }
    }
}


pub fn test1() {
    let lines: VecDeque<_> = utils::read_lines("data/day8.txt").collect();
    let (dirs, places, _) = read_data(lines);

    let (_, count) = solve(AAA_INDEX, &dirs, &places, |p| p == ZZZ_INDEX);

    println!("Moves: {count}");
}

pub fn test2() {
    let lines: VecDeque<_> = utils::read_lines("data/day8.txt").collect();
    let (dirs, places, positions) = read_data(lines);

    // This is the kind of aoc problem that I HATE, with passion.
    // The "solution" is not a solver algorithm but a kludge based
    // on assumptions that only work because the input is done a specific way.
    // That punishes anyone who has actually thought about the problem
    // (there is no reason the LCM should work) and feels just wrong.

    // Advent of code: 💩

    let sol = positions.iter().map(|p| solve(*p, &dirs, &places, ends_with_z).1).fold(1u64, |a, e| num::integer::lcm(a, e));
    println!("💩💩💩💩 {sol} 💩💩💩💩");

/*
    // Code I wrote to check the period and what not of the solutions, followed by
    // minutes of disbelief because WHAT THE FUCK the LCM is such a corner case
    // God, does today's problem suck.

    for start_pos in positions.iter() {
        let sol = solve(pos

        loop {
            let (new, c) = ;
            totc += c;
            println!("Solved {start_pos} ({pos}) at {new} after {totc} moves");
            if !found.insert(new) {
                break;
            }
            pos = new;
        }
    }
 */

/*
    // The brute force solver that doesn't work but would actually solve the
    // problem instead of "your lucky special case"

    let mut dir_idx = 0;
    let mut count = 0;

    loop {
        for pos in positions.iter_mut() {
            if dirs[dir_idx] == Dir::Left {
                *pos = places[*pos].0;
            } else {
                *pos = places[*pos].1;
            }
        }

        dir_idx = (dir_idx + 1) % dirs.len();
        count += 1;

        if positions.iter().all(|p| ends_with_z(*p)) {
            break;
        }
    }

    println!("Moves: {count}"); */
}
