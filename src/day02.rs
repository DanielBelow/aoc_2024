use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::iter::Iterator;

#[derive(Clone, Debug)]
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
    #[allow(clippy::cast_possible_wrap)]
    fn pairwise_diff(&self) -> Vec<isize> {
        self.levels
            .windows(2)
            .map(|it| it[0] as isize - it[1] as isize)
            .collect()
    }

    fn is_safe(&self) -> bool {
        let diff = self.pairwise_diff();
        diff.iter().all(|it| (1..=3).contains(it)) || diff.iter().all(|it| (-3..=-1).contains(it))
    }

    fn safe_with_removing(&self) -> bool {
        if self.is_safe() {
            return true;
        }

        (0..self.levels.len()).any(|i| {
            let mut new_inp = self.clone();
            new_inp.levels.remove(i);
            new_inp.is_safe()
        })
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
