use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

#[aoc_generator(day22)]
pub fn generate(s: &str) -> Vec<usize> {
    s.lines().map(|l| l.parse().expect("number")).collect()
}

const fn prune(result: usize) -> usize {
    result % 16_777_216
}
const fn mix(secret: usize, result: usize) -> usize {
    secret ^ result
}

const fn calculate_next_number(secret: usize) -> usize {
    // Calculate the result of multiplying the secret number by 64. Then, mix this result into the secret number.
    // Finally, prune the secret number.
    let secret = prune(mix(secret, secret * 64));

    // Calculate the result of dividing the secret number by 32. Round the result down to the nearest integer.
    // Then, mix this result into the secret number. Finally, prune the secret number.
    let secret = prune(mix(secret, secret / 32));

    // Calculate the result of multiplying the secret number by 2048.
    // Then, mix this result into the secret number. Finally, prune the secret number.
    prune(mix(secret, secret * 2048))
}

#[aoc(day22, part1)]
pub fn part1(inp: &[usize]) -> usize {
    inp.iter().fold(0, |acc, it| {
        acc + (0..2000).fold(*it, |acc, _| calculate_next_number(acc))
    })
}

#[aoc(day22, part2)]
pub fn part2(inp: &[usize]) -> Option<usize> {
    let mut sequences = vec![vec![0; 2001]; inp.len()];

    for (buyer, mut i) in inp.iter().copied().enumerate() {
        let buyer = &mut sequences[buyer];
        buyer[0] = i % 10;
        for slot in buyer.iter_mut().skip(1) {
            let next_num = calculate_next_number(i);
            *slot = next_num % 10;
            i = next_num;
        }
    }

    let mut diffs = vec![vec![0isize; 2001]; inp.len()];
    for (buyer_idx, buyer) in sequences.iter().enumerate() {
        diffs[buyer_idx][0] = isize::try_from(buyer[0]).ok()?;

        for (idx, win) in buyer.windows(2).enumerate() {
            let diff = isize::try_from(win[1]).ok()? - isize::try_from(win[0]).ok()?;
            diffs[buyer_idx][idx + 1] = diff;
        }
    }

    let mut prices = HashMap::new();
    for (buyer_idx, diff) in diffs.iter().enumerate() {
        let mut seen = HashSet::new();
        for (idx, diff) in diff.windows(4).enumerate().skip(1) {
            if seen.contains(diff) {
                continue;
            }

            seen.insert(diff);
            *prices.entry(diff).or_insert(0) += sequences[buyer_idx][idx + 3];
        }
    }

    prices.values().max().copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_P1: &str = "1\n\
                                 10\n\
                                 100\n\
                                 2024";

    const TEST_INPUT_P2: &str = "1\n\
                                 2\n\
                                 3\n\
                                 2024";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT_P1);
        let res = part1(&gen);
        assert_eq!(res, 37_327_623);
    }

    #[test]
    fn test_p2() {
        let gen = generate(TEST_INPUT_P2);
        let res = part2(&gen);
        assert_eq!(res, Some(23));
    }
}
