//! https://adventofcode.com/2024/day/3

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::ops::{Not, Range};

lazy_static! {
    static ref MUL_REGEX: Regex = Regex::new(r"mul\((\d\d?\d?),(\d\d?\d?)\)").unwrap();
}

fn part_1(input: &String) -> i64 {
    MUL_REGEX
        .captures_iter(input.as_str())
        .map(|g| g.extract())
        .map(|(_, [left, right])| (left.parse::<i64>().unwrap(), right.parse::<i64>().unwrap()))
        .map(|(left, right)| left * right)
        .sum()
}

fn part_2(input: &String) -> i64 {
    // Construct a set of ranges between `don't()` and `do()` where matches should be ignored
    let disabled_regex = Regex::new(r"(don't\(\))[\s\S]+?(do\(\))").unwrap();
    let disabled_ranges: HashSet<Range<usize>> = HashSet::from_iter(
        disabled_regex
            .captures_iter(input.as_str())
            .map(|g| g.get(0).unwrap().range()),
    );
    MUL_REGEX
        .captures_iter(input.as_str())
        .filter(|g| {
            let pos = &g.get(0).unwrap().start();
            disabled_ranges
                .iter()
                .any(|range| range.contains(pos))
                .not()
        })
        .map(|g| g.extract())
        .map(|(_, [left, right])| (left.parse::<i64>().unwrap(), right.parse::<i64>().unwrap()))
        .map(|(left, right)| left * right)
        .sum()
}

pub fn run(input: &String) {
    println!("part 1 solution: {}", part_1(input));
    println!("part 2 solution: {}", part_2(input));
}
