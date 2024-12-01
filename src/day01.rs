use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day01)]
pub fn generate(s: &str) -> Option<(Vec<usize>, Vec<usize>)> {
    let mut left = vec![];
    let mut right = vec![];

    for line in s.lines() {
        let whs = line.split_whitespace().collect::<Vec<_>>();

        let l = whs[0].parse::<usize>().ok()?;
        let r = whs[1].parse::<usize>().ok()?;

        left.push(l);
        right.push(r);
    }

    Some((left, right))
}

#[aoc(day01, part1)]
pub fn part1((left, right): &(Vec<usize>, Vec<usize>)) -> usize {
    let mut left = left.clone();
    let mut right = right.clone();

    left.sort_unstable();
    right.sort_unstable();

    left.iter().zip(right).map(|(l, r)| l.abs_diff(r)).sum()
}

#[aoc(day01, part2)]
pub fn part2((left, right): &(Vec<usize>, Vec<usize>)) -> usize {
    let counts = right.iter().counts();
    left.iter()
        .map(|it| it * counts.get(it).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3   4
                              4   3\n\
                              2   5\n\
                              1   3\n\
                              3   9\n\
                              3   3";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT).expect("failed to parse input");
        let res = part1(&gen);
        assert_eq!(res, 11);
    }

    #[test]
    fn test_p2() {
        let gen = generate(TEST_INPUT).expect("failed to parse input");
        let res = part2(&gen);
        assert_eq!(res, 31);
    }
}
