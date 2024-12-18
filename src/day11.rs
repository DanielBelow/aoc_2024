use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use memoize::memoize;
use num::Integer;

#[aoc_generator(day11)]
pub fn generate(s: &str) -> Vec<usize> {
    s.split_ascii_whitespace()
        .map(|s| s.parse::<usize>().expect("number"))
        .collect_vec()
}

#[memoize]
fn transform_digit(stone: usize, max_steps: usize) -> usize {
    if max_steps == 0 {
        return 1;
    }

    if stone == 0 {
        return transform_digit(1, max_steps - 1);
    }

    let num_digits = 1 + stone.ilog10();
    if num_digits.is_even() {
        let divisor = 10usize.pow(num_digits / 2);

        return transform_digit(stone / divisor, max_steps - 1)
            + transform_digit(stone % divisor, max_steps - 1);
    }

    transform_digit(stone * 2024, max_steps - 1)
}

#[aoc(day11, part1)]
pub fn part1(inp: &[usize]) -> usize {
    inp.iter()
        .fold(0usize, |acc, stone| acc + transform_digit(*stone, 25))
}

#[aoc(day11, part2)]
pub fn part2(inp: &[usize]) -> usize {
    inp.iter()
        .fold(0usize, |acc, stone| acc + transform_digit(*stone, 75))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "125 17";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT);
        let res = part1(&gen);
        assert_eq!(res, 55312);
    }

    #[test]
    fn test_p2() {
        let gen = generate(TEST_INPUT);
        let res = part2(&gen);
        assert_eq!(res, 65_601_038_650_482);
    }
}
