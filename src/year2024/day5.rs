//! https://adventofcode.com/2024/day/5

use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

fn parse_input(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let lines: Vec<&str> = input.lines().collect();
    let (rule_strs, update_strs) = lines.split(|line| line.is_empty()).collect_tuple().unwrap();

    let rules: Vec<(u32, u32)> = rule_strs
        .iter()
        .map(|rule_str| rule_str.split("|").collect_tuple().unwrap())
        .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
        .collect();

    let updates: Vec<Vec<u32>> = update_strs
        .iter()
        .map(|update_str| update_str.split(",").map(|n| n.parse().unwrap()).collect())
        .collect();

    (rules, updates)
}

/// Create a map of page -> pages that cannot be before that page
fn construct_rule_map(rules: &Vec<(u32, u32)>) -> HashMap<&u32, HashSet<&u32>> {
    rules
        .iter()
        .fold(HashMap::new(), |mut acc, (before, after)| {
            acc.entry(after)
                .or_insert_with(HashSet::default)
                .insert(before);
            acc
        })
}

fn cmp_by_rules(left: &u32, right: &u32, rules: &HashMap<&u32, HashSet<&u32>>) -> Ordering {
    if rules
        .get(right)
        .map_or(false, |must_before| must_before.contains(left))
    {
        Ordering::Less
    } else if rules
        .get(left)
        .map_or(false, |must_before| must_before.contains(right))
    {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

fn part_1(input: &str) -> usize {
    let (rules, updates) = parse_input(input);
    let rule_map = construct_rule_map(&rules);

    updates
        .iter()
        .filter(|update| {
            update
                .iter()
                .sorted_by(|left, right| cmp_by_rules(left, right, &rule_map))
                .eq(update.iter())
        })
        .map(|update| *update.get(update.len() / 2).unwrap() as usize)
        .sum()
}

fn part_2(input: &str) -> usize {
    let (rules, updates) = parse_input(input);
    let rule_map = construct_rule_map(&rules);

    updates
        .iter()
        .filter_map(|update| {
            let update_sorted: Vec<u32> = update
                .iter()
                .sorted_by(|left, right| cmp_by_rules(left, right, &rule_map))
                .copied()
                .collect();
            if !update_sorted.eq(update) {
                Some(update_sorted)
            } else {
                None
            }
        })
        .map(|update| *update.get(update.len() / 2).unwrap() as usize)
        .sum()
}

pub fn run(input: &str) {
    println!("part 1 solution: {}", part_1(input));
    println!("part 2 solution: {}", part_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        47|53\n\
        97|13\n\
        97|61\n\
        97|47\n\
        75|29\n\
        61|13\n\
        75|53\n\
        29|13\n\
        97|29\n\
        53|29\n\
        61|53\n\
        97|53\n\
        61|29\n\
        47|13\n\
        75|47\n\
        97|75\n\
        47|61\n\
        75|61\n\
        47|29\n\
        75|13\n\
        53|13\n\
\n\
        75,47,61,53,29\n\
        97,61,53,29,13\n\
        75,29,13\n\
        75,97,47,61,53\n\
        61,13,29\n\
        97,13,75,29,47";

    #[test]
    fn test_part_1() {
        assert_eq!(143, part_1(EXAMPLE))
    }

    #[test]
    fn test_part_2() {
        assert_eq!(123, part_2(EXAMPLE))
    }
}
