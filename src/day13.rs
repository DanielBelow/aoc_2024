use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display_derive::{Display, FromStr};
use z3::ast::Ast;

const BUTTON_A_COST: u64 = 3;
const BUTTON_B_COST: u64 = 1;

#[derive(Display, FromStr, Clone, Debug)]
#[display("Button {name}: X+{x_offset}, Y+{y_offset}")]
pub struct Button {
    name: String,
    x_offset: u64,
    y_offset: u64,
}

#[derive(Display, FromStr, Copy, Clone, Debug)]
#[display("Prize: X={x}, Y={y}")]
pub struct Prize {
    x: u64,
    y: u64,
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
    generate_p1(s)
        .iter()
        .map(|i| InputData {
            button_a: i.button_a.clone(),
            button_b: i.button_b.clone(),
            prize: Prize {
                x: i.prize.x + 10_000_000_000_000,
                y: i.prize.y + 10_000_000_000_000,
            },
        })
        .collect_vec()
}

fn solve_constraints(input: &InputData, ctx: &z3::Context) -> Option<u64> {
    let optimizer = z3::Optimize::new(ctx);

    let a_cost = z3::ast::Int::from_u64(ctx, BUTTON_A_COST);
    let b_cost = z3::ast::Int::from_u64(ctx, BUTTON_B_COST);

    let prize_x = z3::ast::Int::from_u64(ctx, input.prize.x);
    let prize_y = z3::ast::Int::from_u64(ctx, input.prize.y);

    let b_a_x = z3::ast::Int::from_u64(ctx, input.button_a.x_offset);
    let b_b_x = z3::ast::Int::from_u64(ctx, input.button_b.x_offset);

    let b_a_y = z3::ast::Int::from_u64(ctx, input.button_a.y_offset);
    let b_b_y = z3::ast::Int::from_u64(ctx, input.button_b.y_offset);

    let num_a = z3::ast::Int::new_const(ctx, "num_a");
    let num_b = z3::ast::Int::new_const(ctx, "num_b");

    let final_x_pos = num_a.clone() * b_a_x + num_b.clone() * b_b_x;
    let final_y_pos = num_a.clone() * b_a_y + num_b.clone() * b_b_y;
    optimizer.assert(&final_x_pos._eq(&prize_x));
    optimizer.assert(&final_y_pos._eq(&prize_y));

    let total_cost = num_a.clone() * a_cost + num_b.clone() * b_cost;
    optimizer.minimize(&total_cost);

    optimizer.check(&[]);

    let model = optimizer.get_model()?;

    let a_presses = model
        .eval(&num_a, true)
        .and_then(|i| i.as_u64())
        .expect("solution");
    let b_presses = model
        .eval(&num_b, true)
        .and_then(|i| i.as_u64())
        .expect("solution");

    Some(a_presses * BUTTON_A_COST + b_presses * BUTTON_B_COST)
}

#[aoc(day13, part1)]
pub fn part1(inp: &[InputData]) -> u64 {
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);

    inp.iter().filter_map(|i| solve_constraints(i, &ctx)).sum()
}

#[aoc(day13, part2)]
pub fn part2(inp: &[InputData]) -> u64 {
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);

    inp.iter().filter_map(|i| solve_constraints(i, &ctx)).sum()
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
