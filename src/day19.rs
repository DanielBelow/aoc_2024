use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Input {
    towels: Vec<String>,
    stripes: Vec<String>,
}

#[aoc_generator(day19)]
pub fn generate(s: &str) -> Option<Input> {
    let (towels, stripes) = s.split_once("\n\n")?;

    let towels = towels.split(", ").map(ToString::to_string).collect_vec();
    let stripes = stripes.lines().map(ToString::to_string).collect_vec();

    Some(Input { towels, stripes })
}

fn count_num_possibilities<'a>(
    stripe: &'a str,
    towels: &[String],
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if stripe.is_empty() {
        return 1;
    }

    if let Some(&count) = cache.get(stripe) {
        return count;
    }

    let count = towels.iter().fold(0, |acc, towel| {
        let sub_count = stripe
            .strip_prefix(towel)
            .map(|rem| count_num_possibilities(rem, towels, cache))
            .unwrap_or_default();
        acc + sub_count
    });

    cache.insert(stripe, count);

    count
}

#[aoc(day19, part1)]
pub fn part1(inp: &Input) -> usize {
    let mut cache = HashMap::new();

    inp.stripes.iter().fold(0, |acc, stripe| {
        let num_possibilities = count_num_possibilities(stripe, &inp.towels, &mut cache);
        acc + usize::from(num_possibilities > 0)
    })
}

#[aoc(day19, part2)]
pub fn part2(inp: &Input) -> usize {
    let mut cache = HashMap::new();

    inp.stripes.iter().fold(0, |acc, stripe| {
        let num_possibilities = count_num_possibilities(stripe, &inp.towels, &mut cache);
        acc + num_possibilities
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "r, wr, b, g, bwu, rb, gb, br\n\
                              \n\
                              brwrr\n\
                              bggr\n\
                              gbbr\n\
                              rrbgbr\n\
                              ubwu\n\
                              bwurrg\n\
                              brgr\n\
                              bbrgwb";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT).expect("valid input");
        let res = part1(&gen);
        assert_eq!(res, 6);
    }

    #[test]
    fn test_p2() {
        let gen = generate(TEST_INPUT).expect("valid input");
        let res = part2(&gen);
        assert_eq!(res, 16);
    }
}
