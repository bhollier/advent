//! https://adventofcode.com/2024/day/9

use num_traits::{cast, NumCast};

type Id = usize;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone)]
enum DiskBlock {
    File(Id),
    Free(),
}

/// Entirely unnecessary iterator which maps block sizes to DiskBlocks
struct DiskBlockIter<N, I>
where
    N: NumCast,
    I: Iterator<Item = N>,
{
    input: I,
    inited: bool,
    is_file: bool,
    current_id: Id,
    remaining_blocks: usize,
}

impl<N, I> Iterator for DiskBlockIter<N, I>
where
    N: NumCast,
    I: Iterator<Item = N>,
{
    type Item = DiskBlock;

    fn next(&mut self) -> Option<Self::Item> {
        // while loop, as it's possible for a size of 0
        while self.remaining_blocks == 0 {
            self.remaining_blocks = cast(self.input.next()?).unwrap();
            if self.inited {
                if self.is_file {
                    self.current_id += 1;
                }
                self.is_file = !self.is_file;
            } else {
                self.inited = true;
            }
        }

        let block = if self.is_file {
            Some(DiskBlock::File(self.current_id))
        } else {
            Some(DiskBlock::Free())
        };
        self.remaining_blocks -= 1;

        block
    }
}

trait DiskBlockIterable<N, I>
where
    N: NumCast,
    I: Iterator<Item = N>,
{
    fn into_disk_blocks(self) -> DiskBlockIter<N, I>
    where
        Self: Sized;
}

impl<N, I> DiskBlockIterable<N, I> for I
where
    N: NumCast,
    I: Iterator<Item = N>,
{
    fn into_disk_blocks(self) -> DiskBlockIter<N, I>
    where
        Self: Sized,
    {
        DiskBlockIter {
            input: self,
            inited: false,
            is_file: true,
            remaining_blocks: 0,
            current_id: 0,
        }
    }
}

fn parse_input(input: &str) -> Vec<DiskBlock> {
    input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .into_disk_blocks()
        .collect()
}

fn checksum(blocks: &[DiskBlock]) -> usize {
    blocks
        .iter()
        .enumerate()
        .filter_map(|(i, d)| match d {
            DiskBlock::File(id) => Some(i * id),
            _ => None,
        })
        .sum()
}

fn part_1(input: &str) -> usize {
    let original = parse_input(input);

    let mut result = Vec::with_capacity(original.len());

    let mut remaining = &original[..];
    while remaining.len() > 0 {
        match &remaining[0] {
            // If the left block is filled, add it to result
            DiskBlock::File(id) => {
                result.push(DiskBlock::File(*id));
                remaining = &remaining[1..];
            }
            DiskBlock::Free() => {
                match &remaining[remaining.len() - 1] {
                    // If the right block is filled, add it to the result
                    DiskBlock::File(id) => {
                        result.push(DiskBlock::File(*id));
                        remaining = &remaining[1..];
                    }
                    _ => {}
                }
                remaining = &remaining[0..remaining.len() - 1];
            }
        }
    }

    checksum(&result)
}

pub fn run(input: &str) {
    println!("part 1 solution: {}", part_1(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "2333133121414131402";

    #[test]
    fn test_parse_input() {
        assert_eq!(
            vec![
                DiskBlock::File(0),
                DiskBlock::Free(),
                DiskBlock::Free(),
                DiskBlock::File(1),
                DiskBlock::File(1),
                DiskBlock::File(1),
                DiskBlock::Free(),
                DiskBlock::Free(),
                DiskBlock::Free(),
                DiskBlock::Free(),
                DiskBlock::File(2),
                DiskBlock::File(2),
                DiskBlock::File(2),
                DiskBlock::File(2),
                DiskBlock::File(2),
            ],
            parse_input("12345")
        )
    }

    #[test]
    fn test_part_1() {
        assert_eq!(1928, part_1(EXAMPLE));
    }
}
