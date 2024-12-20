//! https://adventofcode.com/2024/day/7

use itertools::Itertools;
use std::rc::Rc;

enum Symbol {
    Constant(u64),
    Add(Rc<Symbol>, Rc<Symbol>),
    Multiply(Rc<Symbol>, Rc<Symbol>),
    Concatenate(Rc<Symbol>, Rc<Symbol>),
}

impl Symbol {
    fn evaluate(&self) -> u64 {
        match self {
            Symbol::Constant(n) => *n,
            Symbol::Add(left, right) => left.evaluate() + right.evaluate(),
            Symbol::Multiply(left, right) => left.evaluate() * right.evaluate(),
            Symbol::Concatenate(left, right) => {
                left.evaluate() * 10_u64.pow(right.evaluate().ilog10() + 1) + right.evaluate()
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split(": ").collect_tuple().unwrap())
        .map(|(solution, operands)| {
            (
                solution.parse().unwrap(),
                operands.split(" ").map(|s| s.parse().unwrap()).collect(),
            )
        })
        .collect()
}

fn solve<'a>(
    solution: &u64,
    symbol: Rc<Symbol>,
    remaining: &[u64],
    concat: bool,
) -> Option<Rc<Symbol>> {
    match remaining {
        [] => {
            if symbol.evaluate() == *solution {
                Some(symbol)
            } else {
                None
            }
        }
        &[n] => {
            let c = Rc::from(Symbol::Constant(n));
            if Symbol::Add(symbol.clone(), c.clone()).evaluate() == *solution {
                Some(Rc::from(Symbol::Add(symbol.clone(), c)))
            } else if Symbol::Multiply(symbol.clone(), c.clone()).evaluate() == *solution {
                Some(Rc::from(Symbol::Multiply(symbol.clone(), c)))
            } else if concat
                && Symbol::Concatenate(symbol.clone(), c.clone()).evaluate() == *solution
            {
                Some(Rc::from(Symbol::Concatenate(symbol.clone(), c)))
            } else {
                None
            }
        }
        ns => {
            let c = Rc::from(Symbol::Constant(ns[0]));
            if let Some(e) = solve(
                solution,
                Rc::from(Symbol::Add(symbol.clone(), c.clone())),
                &ns[1..],
                concat,
            ) {
                Some(e)
            } else if let Some(e) = solve(
                solution,
                Rc::from(Symbol::Multiply(symbol.clone(), c.clone())),
                &ns[1..],
                concat,
            ) {
                Some(e)
            } else {
                if concat {
                    if let Some(e) = solve(
                        solution,
                        Rc::from(Symbol::Concatenate(symbol.clone(), c)),
                        &ns[1..],
                        true,
                    ) {
                        Some(e)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        }
    }
}

fn solve_calibrations(input: &str, concat: bool) -> u64 {
    parse_input(input)
        .iter()
        .filter_map(|(solution, values)| {
            solve(
                solution,
                Rc::from(Symbol::Constant(values[0])),
                &values[1..],
                concat,
            )
            .map(|_| solution)
        })
        .sum()
}

pub fn run(input: &str) {
    println!("part 1 solution: {}", solve_calibrations(input, false));
    println!("part 2 solution: {}", solve_calibrations(input, true));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        190: 10 19\n\
        3267: 81 40 27\n\
        83: 17 5\n\
        156: 15 6\n\
        7290: 6 8 6 15\n\
        161011: 16 10 13\n\
        192: 17 8 14\n\
        21037: 9 7 18 13\n\
        292: 11 6 16 20";

    #[test]
    fn test_part_1() {
        assert_eq!(3749, solve_calibrations(EXAMPLE, false));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(11387, solve_calibrations(EXAMPLE, true));
    }
}
