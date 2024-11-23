use aoc_runner_derive::aoc;

#[aoc(day01, part1)]
pub const fn part1(_inp: &str) -> u32 {
    0
}

#[aoc(day01, part2)]
pub const fn part2(_inp: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_P1: &str = "";

    const TEST_INPUT_P2: &str = "";

    #[test]
    fn test_p1() {
        let res = part1(TEST_INPUT_P1);
        assert_eq!(res, 0);
    }

    #[test]
    fn test_p2() {
        let res = part2(TEST_INPUT_P2);
        assert_eq!(res, 0);
    }
}
