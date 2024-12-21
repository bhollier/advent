//! https://adventofcode.com/2024/day/8

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type Coordinate = (i32, i32);

fn parse_input(input: &str) -> (HashMap<char, Vec<Coordinate>>, usize) {
    let grid_size = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().filter(|c| !c.is_whitespace()).count())
        .next()
        .unwrap();

    let antenna_map = input
        .lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices()
                .filter(|(_, c)| c.is_alphanumeric())
                .map(move |(x, c)| (c, (x as i32, y as i32)))
        })
        .into_group_map();

    (antenna_map, grid_size)
}

struct AntinodeIterator {
    curr: Coordinate,
    dx: i32,
    dy: i32,
}

impl Iterator for AntinodeIterator {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.curr;
        self.curr = (x + self.dx, y + self.dy);
        Some((x, y))
    }
}

fn calculate_antinodes(
    (left_x, left_y): &Coordinate,
    (right_x, right_y): &Coordinate,
) -> AntinodeIterator {
    AntinodeIterator {
        curr: (*left_x, *left_y),
        dx: left_x - right_x,
        dy: left_y - right_y,
    }
}

fn calculate_antinode(left: &Coordinate, right: &Coordinate) -> Coordinate {
    calculate_antinodes(left, right).skip(1).next().unwrap()
}

fn is_valid((x, y): Coordinate, grid_size: usize) -> bool {
    x >= 0 && x < grid_size as i32 && y >= 0 && y < grid_size as i32
}

fn part_1(input: &str) -> usize {
    let mut antinode_locations = HashSet::new();
    let (antenna_map, grid_size) = parse_input(input);

    antenna_map.values().for_each(|antennas| {
        antennas
            .iter()
            .flat_map(|left| antennas.iter().map(move |right| (left, right)))
            .filter(|(left, right)| left != right)
            .for_each(|(left, right)| {
                let (antinode_1, antinode_2) = (
                    calculate_antinode(left, right),
                    calculate_antinode(right, left),
                );
                if is_valid(antinode_1, grid_size) {
                    antinode_locations.insert(antinode_1);
                }
                if is_valid(antinode_2, grid_size) {
                    antinode_locations.insert(antinode_2);
                }
            })
    });

    antinode_locations.len()
}

fn part_2(input: &str) -> usize {
    let mut antinode_locations = HashSet::new();
    let (antenna_map, grid_size) = parse_input(input);

    antenna_map.values().for_each(|antennas| {
        antennas
            .iter()
            .flat_map(|left| antennas.iter().map(move |right| (left, right)))
            .filter(|(left, right)| left != right)
            .for_each(|(left, right)| {
                calculate_antinodes(left, right)
                    .take_while(|pos| is_valid(*pos, grid_size))
                    .for_each(|pos| {
                        antinode_locations.insert(pos);
                    });
                calculate_antinodes(right, left)
                    .take_while(|pos| is_valid(*pos, grid_size))
                    .for_each(|pos| {
                        antinode_locations.insert(pos);
                    });
            })
    });

    antinode_locations.len()
}

pub fn run(input: &str) {
    println!("part 1 solution: {}", part_1(input));
    println!("part 2 solution: {}", part_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        ............\n\
        ........0...\n\
        .....0......\n\
        .......0....\n\
        ....0.......\n\
        ......A.....\n\
        ............\n\
        ............\n\
        ........A...\n\
        .........A..\n\
        ............\n\
        ............";

    #[test]
    fn test_calculate_antinode() {
        assert_eq!((3, 1), calculate_antinode(&(4, 3), &(5, 5)));
        assert_eq!((6, 7), calculate_antinode(&(5, 5), &(4, 3)));
    }

    #[test]
    fn test_calculate_antinodes() {
        let mut iter = calculate_antinodes(&(4, 3), &(5, 5));
        assert_eq!(Some((4, 3)), iter.next());
        assert_eq!(Some((3, 1)), iter.next());
        assert_eq!(Some((2, -1)), iter.next());
    }

    #[test]
    fn test_part_1() {
        assert_eq!(14, part_1(EXAMPLE));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(34, part_2(EXAMPLE));
    }
}
