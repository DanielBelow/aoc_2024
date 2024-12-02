use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::iter::Iterator;

pub struct Input {
    levels: Vec<usize>,
}

#[aoc_generator(day02)]
pub fn generate(s: &str) -> Vec<Input> {
    let mut inputs = vec![];

    for line in s.lines() {
        let nums = line
            .split_whitespace()
            .map(|it| it.parse::<usize>().expect("whitespace separated numbers"))
            .collect_vec();
        inputs.push(Input { levels: nums });
    }

    inputs
}

impl Input {
    fn all_increasing(&self) -> bool {
        self.levels.windows(2).all(|it| it[0] < it[1])
    }

    fn all_decreasing(&self) -> bool {
        self.levels.windows(2).all(|it| it[0] > it[1])
    }

    fn adjacent_diff(&self) -> bool {
        self.levels.windows(2).all(|it| {
            let diff = it[0].abs_diff(it[1]);
            (1..=3).contains(&diff)
        })
    }

    fn is_safe(&self) -> bool {
        (self.all_increasing() || self.all_decreasing()) && self.adjacent_diff()
    }

    fn safe_with_removing(&self) -> bool {
        if self.is_safe() {
            return true;
        }

        for i in 0..self.levels.len() {
            let mut lvls = self.levels.clone();
            lvls.remove(i);

            let new_inp = Self { levels: lvls };

            if new_inp.is_safe() {
                return true;
            }
        }

        false
    }
}

#[aoc(day02, part1)]
pub fn part1(inp: &[Input]) -> usize {
    inp.iter().filter(|it| it.is_safe()).count()
}

#[aoc(day02, part2)]
pub fn part2(inp: &[Input]) -> usize {
    inp.iter().filter(|it| it.safe_with_removing()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "7 6 4 2 1\n\
                              1 2 7 8 9\n\
                              9 7 6 2 1\n\
                              1 3 2 4 5\n\
                              8 6 4 4 1\n\
                              1 3 6 7 9";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT);
        let res = part1(&gen);
        assert_eq!(res, 2);
    }

    #[test]
    fn test_p2() {
        let gen = generate(TEST_INPUT);
        let res = part2(&gen);
        assert_eq!(res, 4);
    }
}
