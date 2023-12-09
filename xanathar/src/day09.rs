use crate::utils;
use std::iter::Iterator;

fn predict_prev(nums: &[i64]) -> i64 {
    if nums.iter().skip(1).all(|v| *v == nums[0]) {
        nums[0]
    } else {
        let prev = (0..nums.len() - 1).map(|i| nums[i + 1] - nums[i]).collect::<Vec<_>>();
        nums.first().unwrap() - predict_prev(&prev)
    }
}

fn predict_next(nums: &[i64]) -> i64 {
    if nums.iter().skip(1).all(|v| *v == nums[0]) {
        nums[0]
    } else {
        let next = (1..nums.len()).map(|i| nums[i] - nums[i - 1]).collect::<Vec<_>>();
        nums.last().unwrap() + predict_next(&next)
    }
}

fn predict_str_next(l: &str) -> i64 {
    predict_next(&l.split(' ').map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>())
}

fn predict_str_prev(l: &str) -> i64 {
    predict_prev(&l.split(' ').map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>())
}

pub fn test1() {
    let lines: Vec<_> = utils::read_lines("data/day9.txt").collect();
    let s = lines.iter().map(|l| predict_str_next(l)).sum::<i64>();
    println!("{s}");
}

pub fn test2() {
    let lines: Vec<_> = utils::read_lines("data/day9.txt").collect();
    let s = lines.iter().map(|l| predict_str_prev(l)).sum::<i64>();
    println!("{s}");
}
