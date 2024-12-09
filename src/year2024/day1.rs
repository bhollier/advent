//! https://adventofcode.com/2024/day/1

use std::collections::HashMap;

fn parse_input(input: &String) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .map(|parts| {
            (
                parts[0].parse::<i32>().unwrap(),
                parts[1].parse::<i32>().unwrap(),
            )
        })
        .unzip()
}

fn part_1(input: &String) -> i32 {
    let (mut left, mut right) = parse_input(input);

    left.sort();
    right.sort();

    left.iter()
        .zip(right)
        .map(|(left, right)| left.abs_diff(right) as i32)
        .sum()
}

fn part_2(input: &String) -> i32 {
    let (left, right) = parse_input(input);

    let occurrences: HashMap<&i32, i32> = right.iter().fold(HashMap::new(), |mut map, number| {
        map.insert(number, *map.get(number).unwrap_or(&0) + 1);
        map
    });

    left.iter()
        .map(|number| *number * *occurrences.get(number).unwrap_or(&0))
        .sum()
}

pub fn run(input: &String) {
    println!("part 1 solution: {}", part_1(input));
    println!("part 2 solution: {}", part_2(input));
}
