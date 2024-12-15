const CARDINAL_DIRECTIONS: [(i32, i32); 4] = [
    (0, -1), // N
    (1, 0),  // E
    (0, 1),  // S
    (-1, 0), // W
];

const INTERCARDINAL_DIRECTIONS: [(i32, i32); 4] = [
    (1, -1),  // NE
    (1, 1),   // SE
    (-1, 1),  // SW
    (-1, -1), // NW
];

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>()
}

fn check_direction(
    search_string: &str,
    grid: &Vec<Vec<char>>,
    mut x: i32,
    mut y: i32,
    dx: i32,
    dy: i32,
) -> bool {
    let mut i: usize = 0;
    while let Some(c) = grid
        .get(y as usize)
        .map(|row| row.get(x as usize))
        .flatten()
    {
        if *c as u8 != *search_string.as_bytes().get(i).unwrap() {
            return false;
        }
        x += dx;
        y += dy;
        i += 1;
        if i == search_string.len() {
            return true;
        }
    }
    false
}

fn part_1(input: &str) -> usize {
    let grid = parse_input(input);
    grid.iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                // Look for the start of XMAS
                .filter_map(|(x, c)| if *c == 'X' { Some(x) } else { None })
                // Look for XMAS in all 8 directions
                .map(|x| {
                    CARDINAL_DIRECTIONS
                        .iter()
                        .chain(INTERCARDINAL_DIRECTIONS.iter())
                        .map(|(dx, dy)| {
                            check_direction("XMAS", &grid, x as i32, y as i32, *dx, *dy) as usize
                        })
                        .sum::<usize>()
                })
                .sum::<usize>()
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    let grid = parse_input(input);
    grid.iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                // Look for the middle of MAS
                .filter_map(|(x, c)| if *c == 'A' { Some(x) } else { None })
                // Look for MAS (with an offset) in the intercardinal directions,
                // only accept if both directions match
                .filter(|x| {
                    INTERCARDINAL_DIRECTIONS
                        .iter()
                        .filter(|(dx, dy)| {
                            check_direction("MAS", &grid, *x as i32 - dx, y as i32 - dy, *dx, *dy)
                        })
                        .count()
                        == 2
                })
                .count()
        })
        .sum()
}

pub fn run(input: &str) {
    println!("part 1 solution: {}", part_1(input));
    println!("part 2 solution: {}", part_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            4,
            part_1(
                "\
            XMAS\n\
            M..A\n\
            A..M\n\
            SAMX"
            )
        );
        assert_eq!(
            2,
            part_1(
                "\
            X..S\n\
            .MA.\n\
            .MA.\n\
            X..S"
            )
        );
        assert_eq!(
            2,
            part_1(
                "\
            XMASAMX\n\
            .......\n\
            .......\n\
            .......\n\
            .......\n\
            .......\n\
            ......."
            )
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            1,
            part_2(
                "\
            M.S\n\
            .A.\n\
            M.S"
            )
        );
        assert_eq!(
            9,
            part_2(
                "\
            .M.S......\n\
            ..A..MSMS.\n\
            .M.S.MAA..\n\
            ..A.ASMSM.\n\
            .M.S.M....\n\
            ..........\n\
            S.S.S.S.S.\n\
            .A.A.A.A..\n\
            M.M.M.M.M.\n\
            .........."
            )
        );
    }
}
