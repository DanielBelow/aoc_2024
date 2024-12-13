use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display_derive::{Display, FromStr};

const BUTTON_A_COST: usize = 3;
const BUTTON_B_COST: usize = 1;

#[derive(Display, FromStr, Clone, Debug)]
#[display("Button {name}: X+{x_offset}, Y+{y_offset}")]
pub struct Button {
    name: String,
    x_offset: f64,
    y_offset: f64,
}

#[derive(Display, FromStr, Copy, Clone, Debug)]
#[display("Prize: X={x}, Y={y}")]
pub struct Prize {
    x: f64,
    y: f64,
}

#[derive(Clone, Debug)]
pub struct InputData {
    button_a: Button,
    button_b: Button,
    prize: Prize,
}

#[aoc_generator(day13, part1)]
pub fn generate_p1(s: &str) -> Vec<InputData> {
    s.split("\n\n")
        .filter_map(|s| {
            let mut lines = s.lines();

            let button_a = lines.next()?.parse::<Button>().ok()?;
            let button_b = lines.next()?.parse::<Button>().ok()?;
            let prize = lines.next()?.parse::<Prize>().ok()?;

            Some(InputData {
                button_a,
                button_b,
                prize,
            })
        })
        .collect_vec()
}

#[aoc_generator(day13, part2)]
pub fn generate_p2(s: &str) -> Vec<InputData> {
    let mut parsed = generate_p1(s);

    for input in &mut parsed {
        input.prize.x += 10_000_000_000_000.0;
        input.prize.y += 10_000_000_000_000.0;
    }

    parsed
}

// solve linear equations

// ax * i + bx * j = px
// ay * i + by * j = py

// (1) * by, (2) * bx

// ax * by * i + bx * by * j = px * by
// ay * bx * i + bx * by * j = py * bx

// subtract
// ax * by * i - ay * bx * i = px * by - py * bx

// factor out i
// i * (ax * by - ay * bx) = px * by - py * bx

// divide
// i = (px * by - py * bx) / (ax * by - ay * bx)

// ax * i + bx * j = px

// subtract ax * i
// bx * j = px - ax * i

// divide by bx
// j = (px - ax * i) / bx

fn calculate_num_presses(input: &InputData) -> (usize, usize) {
    let Prize { x: px, y: py } = input.prize;

    let Button {
        x_offset: ax,
        y_offset: ay,
        ..
    } = &input.button_a;

    let Button {
        x_offset: bx,
        y_offset: by,
        ..
    } = &input.button_b;

    // i = (px * by - py * bx) / (ax * by - ay * bx)
    let a_presses = (px * by - py * bx) / (ax * by - ay * bx);

    if a_presses.fract() != 0.0 {
        return (0, 0);
    }

    // j = (px - ax * i) / bx
    let b_presses = (px - ax * a_presses) / bx;

    (a_presses as usize, b_presses as usize)
}

#[aoc(day13, part1)]
pub fn part1(inp: &[InputData]) -> usize {
    inp.iter().fold(0usize, |acc, input| {
        let (a_presses, b_presses) = calculate_num_presses(input);
        acc + a_presses * BUTTON_A_COST + b_presses * BUTTON_B_COST
    })
}

#[aoc(day13, part2)]
pub fn part2(inp: &[InputData]) -> usize {
    inp.iter().fold(0usize, |acc, input| {
        let (a_presses, b_presses) = calculate_num_presses(input);
        acc + a_presses * BUTTON_A_COST + b_presses * BUTTON_B_COST
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Button A: X+94, Y+34\n\
                              Button B: X+22, Y+67\n\
                              Prize: X=8400, Y=5400\n\
                              \n\
                              Button A: X+26, Y+66\n\
                              Button B: X+67, Y+21\n\
                              Prize: X=12748, Y=12176\n\
                              \n\
                              Button A: X+17, Y+86\n\
                              Button B: X+84, Y+37\n\
                              Prize: X=7870, Y=6450\n\
                              \n\
                              Button A: X+69, Y+23\n\
                              Button B: X+27, Y+71\n\
                              Prize: X=18641, Y=10279";

    #[test]
    fn test_p1() {
        let gen = generate_p1(TEST_INPUT);
        let res = part1(&gen);
        assert_eq!(res, 480);
    }
}
