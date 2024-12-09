//! https://adventofcode.com/2024/day/3

use regex::Regex;

fn part_1(input: &String) -> i64 {
    let regex = Regex::new(r"mul\((\d\d?\d?),(\d\d?\d?)\)").unwrap();
    regex
        .captures_iter(input.as_str())
        .map(|g| g.extract())
        .map(|(_, [left, right])| (left.parse::<i64>().unwrap(), right.parse::<i64>().unwrap()))
        .map(|(left, right)| left * right)
        .sum()
}

pub fn run(input: &String) {
    println!("part 1 solution: {}", part_1(input));
}
