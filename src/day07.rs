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

impl Equation {
    const fn concat_numbers(lhs: i64, rhs: i64) -> i64 {
        lhs * 10i64.pow(rhs.ilog10() + 1) + rhs
    }

    fn can_solve(&self, ops: &[Operator], available_ops: &[Operator]) -> bool {
        let req_ops = self.numbers.len() - 1;
        if ops.len() != req_ops {
            for &op in available_ops {
                let mut new_ops = ops.to_vec();
                new_ops.push(op);
                if self.can_solve(&new_ops, available_ops) {
                    return true;
                }
            }

            return false;
        }

        let mut res = self.numbers[0];
        for idx in 1..self.numbers.len() {
            if res > self.target {
                return false;
            }

            let op = ops[idx - 1];
            let n = self.numbers[idx];
            match op {
                Operator::Plus => res += n,
                Operator::Mul => res *= n,
                Operator::Concat => res = Self::concat_numbers(res, n),
            };
        }

        res == self.target
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
        let can_solve = eq.can_solve(&[], &[Operator::Plus, Operator::Mul]);
        acc + if can_solve { eq.target } else { 0 }
    })
}

#[aoc(day07, part2)]
pub fn part2(inp: &[Equation]) -> i64 {
    inp.iter().fold(0, |acc, eq| {
        let can_solve = eq.can_solve(&[], &[Operator::Plus, Operator::Mul, Operator::Concat]);
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
