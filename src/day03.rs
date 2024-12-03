use aoc_runner_derive::aoc;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX_MUL: Regex =
        Regex::new(r"mul\((\d{1,3}),\s*(\d{1,3})\)").expect("valid regex");
    static ref REGEX_MUL_KEYWORDS: Regex =
        Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").expect("valid regex");
}

#[aoc(day03, part1)]
pub fn part1(inp: &str) -> usize {
    let mut result = 0;

    for line in inp.lines() {
        for capt in REGEX_MUL.captures_iter(line) {
            let a = capt[1].parse::<usize>().expect("valid number");
            let b = capt[2].parse::<usize>().expect("valid number");

            result += a * b;
        }
    }

    result
}

#[aoc(day03, part2)]
pub fn part2(inp: &str) -> usize {
    let mut result = 0;

    let all = inp
        .lines()
        .flat_map(|l| REGEX_MUL_KEYWORDS.find_iter(l))
        .map(|m| m.as_str());

    let mut ignore = false;
    for instr in all {
        match instr {
            "don't()" => ignore = true,
            "do()" => ignore = false,
            m if !ignore => {
                result += part1(m);
            }
            _ => {}
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_P1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const TEST_INPUT_P2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_p1() {
        let r = part1(TEST_INPUT_P1);
        assert_eq!(161, r);
    }

    #[test]
    fn test_p2() {
        let r = part2(TEST_INPUT_P2);
        assert_eq!(48, r);
    }
}
