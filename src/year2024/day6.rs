//! https://adventofcode.com/2024/day/6

use std::collections::HashSet;

type Coordinate = (i32, i32);

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
enum CardinalDirections {
    North,
    East,
    South,
    West,
}

impl CardinalDirections {
    fn turn_right(&self) -> Self {
        match self {
            CardinalDirections::North => CardinalDirections::East,
            CardinalDirections::East => CardinalDirections::South,
            CardinalDirections::South => CardinalDirections::West,
            CardinalDirections::West => CardinalDirections::North,
        }
    }

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

fn get_obstacle<'a>(obstacles: &'a Vec<Vec<bool>>, (x, y): &Coordinate) -> Option<&'a bool> {
    obstacles
        .get(*y as usize)
        .map(|row| row.get(*x as usize))
        .flatten()
}

fn is_loop((mut guard_x, mut guard_y): &Coordinate, obstacles: &Vec<Vec<bool>>) -> bool {
    let mut direction = CardinalDirections::North;
    let mut visited = HashSet::new();
    while let Some(next_space_occupied) = get_obstacle(
        &obstacles,
        &(guard_x + direction.dx(), guard_y + direction.dy()),
    ) {
        if *next_space_occupied {
            direction = direction.turn_right();
        } else if visited.contains(&((guard_x, guard_y), direction)) {
            return true;
        } else {
            visited.insert(((guard_x, guard_y), direction));
            guard_x += direction.dx();
            guard_y += direction.dy();
        }
    }
    false
}

fn part_1(input: &str) -> HashSet<Coordinate> {
    let ((mut guard_x, mut guard_y), obstacles) = parse_input(input);
    let mut direction = CardinalDirections::North;
    let mut visited = HashSet::new();
    while let Some(next_space_occupied) = get_obstacle(
        &obstacles,
        &(guard_x + direction.dx(), guard_y + direction.dy()),
    ) {
        if *next_space_occupied {
            direction = direction.turn_right();
        } else {
            visited.insert((guard_x, guard_y));
            guard_x += direction.dx();
            guard_y += direction.dy();
        }
    }
    visited.insert((guard_x, guard_y));
    visited
}

/// Couldn't get a smart algorithm working,
/// so this just looks at every space that is visited
/// and sees if placing an obstacle causes a loop
/// (brute force basically)
fn part_2(input: &str) -> HashSet<Coordinate> {
    let visited = part_1(input);

    let (guard_pos, obstacles) = parse_input(input);
    obstacles
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, has_obstacle)| ((x as i32, y as i32), has_obstacle))
                .filter(|(pos, has_obstacle)| {
                    !*has_obstacle
                        && *pos != guard_pos
                        && *pos != (guard_pos.0, guard_pos.1 - 1)
                        && visited.contains(pos)
                })
                .map(|(pos, _)| pos)
        })
        .filter(|(x, y)| {
            let mut copy = obstacles.clone();
            copy[*y as usize][*x as usize] = true;
            is_loop(&guard_pos, &copy)
        })
        .collect()
}

pub fn run(input: &str) {
    println!("part 1 solution: {}", part_1(input).len());
    println!("part 2 solution: {}", part_2(input).len());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        ....#.....\n\
        .........#\n\
        ..........\n\
        ..#.......\n\
        .......#..\n\
        ..........\n\
        .#..^.....\n\
        ........#.\n\
        #.........\n\
        ......#...";

    #[test]
    fn test_part_1() {
        assert_eq!(41, part_1(EXAMPLE).len());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(6, part_2(EXAMPLE).len());
        assert_eq!(
            2,
            part_2(
                "\
                ..##..\n\
                .....#\n\
                ..^...\n\
                ......\n\
                ....#.\n\
                "
            )
            .len()
        );
        assert_eq!(
            1,
            part_2(
                "\
                ..#...\n\
                .....#\n\
                ..^...\n\
                ...#..\n\
                ....#.\n\
                "
            )
            .len()
        );
        assert_eq!(
            1,
            part_2(
                "\
                ..#...\n\
                .....#\n\
                ..^..#\n\
                ...#..\n\
                ....#.\n\
                "
            )
            .len()
        );
    }
}
