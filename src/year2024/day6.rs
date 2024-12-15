//! https://adventofcode.com/2024/day/6

use std::collections::HashSet;

type Coordinate = (i32, i32);

enum CardinalDirections {
    North,
    East,
    South,
    West,
}

impl CardinalDirections {
    fn dxy(&self) -> (i32, i32) {
        match self {
            CardinalDirections::North => (0, -1),
            CardinalDirections::East => (1, 0),
            CardinalDirections::South => (0, 1),
            CardinalDirections::West => (-1, 0),
        }
    }

    fn dx(&self) -> i32 {
        self.dxy().0
    }

    fn dy(&self) -> i32 {
        self.dxy().1
    }
}

fn parse_input(input: &str) -> (Coordinate, Vec<Vec<bool>>) {
    let guard_pos: Coordinate = input
        .lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| ((x, y), c)))
        .find_map(|((x, y), c)| match c {
            '^' => Some((x as i32, y as i32)),
            _ => None,
        })
        .unwrap();
    let obstacles = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();
    (guard_pos, obstacles)
}

fn part_1(input: &str) -> usize {
    let ((mut guard_x, mut guard_y), obstacles) = parse_input(input);
    let mut direction = CardinalDirections::North;
    let mut visited = HashSet::new();
    while let Some(next_space_occupied) = obstacles
        .get((guard_y + direction.dy()) as usize)
        .map(|row| row.get((guard_x + direction.dx()) as usize))
        .flatten()
    {
        if *next_space_occupied {
            direction = match direction {
                CardinalDirections::North => CardinalDirections::East,
                CardinalDirections::East => CardinalDirections::South,
                CardinalDirections::South => CardinalDirections::West,
                CardinalDirections::West => CardinalDirections::North,
            }
        } else {
            visited.insert((guard_x, guard_y));
            guard_x += direction.dx();
            guard_y += direction.dy();
        }
    }
    visited.insert((guard_x, guard_y));
    visited.len()
}

pub fn run(input: &str) {
    println!("part 1 solution: {}", part_1(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            41,
            part_1(
                "\
                ....#.....\n\
                .........#\n\
                ..........\n\
                ..#.......\n\
                .......#..\n\
                ..........\n\
                .#..^.....\n\
                ........#.\n\
                #.........\n\
                ......#..."
            )
        )
    }
}
