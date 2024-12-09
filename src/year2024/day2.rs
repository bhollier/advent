//! https://adventofcode.com/2024/day/2

fn parse_input<'a>(input: &'a String) -> impl Iterator<Item = Vec<i64>> + use<'a> {
    input.lines().filter(|line| !line.is_empty()).map(|line| {
        line.split_whitespace()
            .map(|str| str.parse().unwrap())
            .collect::<Vec<i64>>()
    })
}

fn valid_report(report: &Vec<i64>) -> bool {
    let diffs: Vec<i64> = report.windows(2).map(|w| w[0] - w[1]).collect();
    // Any two adjacent levels differ by at least one and at most three.
    let within_range = diffs.iter().all(|diff| (1..=3).contains(&diff.abs()));
    // The levels are either all increasing or all decreasing.
    let same_direction = diffs.windows(2).all(|w| w[0].signum() == w[1].signum());
    within_range && same_direction
}

fn part_1(input: &String) -> usize {
    parse_input(input).filter(valid_report).count()
}

fn part_2(input: &String) -> usize {
    parse_input(input)
        .filter(|report| {
            // Brute force check each permutation of the report without a level
            (0..report.len()).any(|i| {
                let mut copy = report.clone();
                copy.remove(i);
                valid_report(&copy)
            })
        })
        .count()
}

pub fn run(input: &String) {
    println!("part 1 solution: {}", part_1(input));
    println!("part 2 solution: {}", part_2(input));
}
