use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct Equation {
    target: i64,
    numbers: Vec<i64>,
}

#[derive(Copy, Clone, Debug)]
pub enum Operator {
    Plus,
    Mul,
    Concat,
}

const fn concat_numbers(lhs: i64, rhs: i64) -> i64 {
    lhs * 10i64.pow(rhs.ilog10() + 1) + rhs
}

impl Operator {
    const fn execute(self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Self::Plus => lhs + rhs,
            Self::Mul => lhs * rhs,
            Self::Concat => concat_numbers(lhs, rhs),
        }
    }
}

impl Equation {
    fn can_solve_impl(&self, current: i64, operands: &[i64], available_ops: &[Operator]) -> bool {
        if operands.is_empty() {
            return self.target == current;
        }

        if current > self.target {
            return false;
        }

        let (next_op, rest) = operands.split_first().expect("non-empty operands");
        available_ops.iter().any(|op| {
            let next = op.execute(current, *next_op);
            self.can_solve_impl(next, rest, available_ops)
        })
    }

    fn can_solve(&self, available_ops: &[Operator]) -> bool {
        let operands = self.numbers.clone();
        self.can_solve_impl(0, &operands, available_ops)
    }
}

#[aoc_generator(day07)]
pub fn generate(s: &str) -> Vec<Equation> {
    s.lines()
        .map(|l| {
            let mut spl = l.split(": ");

            let target = spl
                .next()
                .and_then(|n| n.parse::<i64>().ok())
                .expect("lhs: rhs1...");
            let numbers = spl
                .next()
                .and_then(|n| {
                    n.split_ascii_whitespace()
                        .filter_map(|n| n.parse::<i64>().ok())
                        .collect_vec()
                        .into()
                })
                .expect("lhs: rhs1...");

            Equation { target, numbers }
        })
        .collect_vec()
}

#[aoc(day07, part1)]
pub fn part1(inp: &[Equation]) -> i64 {
    inp.iter().fold(0, |acc, eq| {
        let can_solve = eq.can_solve(&[Operator::Plus, Operator::Mul]);
        acc + if can_solve { eq.target } else { 0 }
    })
}

#[aoc(day07, part2)]
pub fn part2(inp: &[Equation]) -> i64 {
    inp.iter().fold(0, |acc, eq| {
        let can_solve = eq.can_solve(&[Operator::Plus, Operator::Mul, Operator::Concat]);
        acc + if can_solve { eq.target } else { 0 }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "190: 10 19\n\
                              3267: 81 40 27\n\
                              83: 17 5\n\
                              156: 15 6\n\
                              7290: 6 8 6 15\n\
                              161011: 16 10 13\n\
                              192: 17 8 14\n\
                              21037: 9 7 18 13\n\
                              292: 11 6 16 20";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT);
        let res = part1(&gen);
        assert_eq!(res, 3749);
    }

    #[test]
    fn test_p2() {
        let gen = generate(TEST_INPUT);
        let res = part2(&gen);
        assert_eq!(res, 11387);
    }
}
