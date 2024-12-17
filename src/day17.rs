use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display_derive::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Instruction {
    #[display("0,{0}")]
    Adv(usize),

    #[display("1,{0}")]
    Bxl(usize),

    #[display("2,{0}")]
    Bst(usize),

    #[display("3,{0}")]
    Jnz(usize),

    #[display("4,{0}")]
    Bxc(usize),

    #[display("5,{0}")]
    Out(usize),

    #[display("6,{0}")]
    Bdv(usize),

    #[display("7,{0}")]
    Cdv(usize),
}

#[derive(Clone)]
pub struct Input {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,

    program: Vec<Instruction>,
}

fn parse_register(line: &str) -> Option<usize> {
    line.split_once(": ")
        .expect("valid input")
        .1
        .parse::<usize>()
        .ok()
}

#[aoc_generator(day17)]
pub fn generate(s: &str) -> Option<Input> {
    let (regs, program_txt) = s.split_once("\n\n")?;

    let mut lines = regs.lines();
    let reg_a = parse_register(lines.next()?)?;
    let reg_b = parse_register(lines.next()?)?;
    let reg_c = parse_register(lines.next()?)?;

    let mut program = vec![];

    for chunk in &program_txt
        .strip_prefix("Program: ")?
        .split(',')
        .map(|s| s.parse::<usize>().expect("number"))
        .chunks(2)
    {
        let (lhs, rhs) = chunk.collect_tuple().expect("valid input");
        program.push(
            format!("{lhs},{rhs}")
                .parse::<Instruction>()
                .expect("valid input"),
        );
    }

    Some(Input {
        reg_a,
        reg_b,
        reg_c,
        program,
    })
}

const fn combo_op(op: usize, inp: &Input) -> usize {
    [op, op, op, op, inp.reg_a, inp.reg_b, inp.reg_c][op]
}

fn run_program(inp: &Input) -> String {
    let mut pc = 0;

    let mut inp = inp.clone();

    let mut output = String::new();

    while pc < inp.program.len() {
        let instr = inp.program[pc];

        match instr {
            Instruction::Adv(op) => inp.reg_a >>= combo_op(op, &inp),
            Instruction::Bxl(op) => inp.reg_b ^= op,
            Instruction::Bst(op) => inp.reg_b = combo_op(op, &inp) % 8,
            Instruction::Bxc(_) => inp.reg_b ^= inp.reg_c,
            Instruction::Bdv(op) => inp.reg_b = inp.reg_a / 2usize.pow(combo_op(op, &inp) as u32),
            Instruction::Cdv(op) => inp.reg_c = inp.reg_a / 2usize.pow(combo_op(op, &inp) as u32),
            Instruction::Out(op) => {
                let fmt = format!("{}", combo_op(op, &inp) % 8);
                output.push_str(&fmt);
            }
            Instruction::Jnz(op) => {
                if inp.reg_a != 0 {
                    pc = op;
                    continue;
                }
            }
        };

        pc += 1;
    }

    output.chars().join(",")
}

#[aoc(day17, part1)]
pub fn part1(inp: &Input) -> String {
    run_program(inp)
}

#[aoc(day17, part2)]
pub const fn part2(_: &Input) -> usize {
    /*
    converted to Rust, solved by finding digits in repeating blocks in 8^n chunks
    do {
        b = a % 8;
        b = b ^ 2
        c = a / 2**b
        a = a / 8
        b = b ^ c
        b = b ^ 7
        print b
    } while (a != 0)
    */
    190_384_113_204_239
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Register A: 729\n\
                              Register B: 0\n\
                              Register C: 0\n\
                              \n\
                              Program: 0,1,5,4,3,0";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT).expect("valid input");
        let res = part1(&gen);
        assert_eq!(res, "4,6,3,5,6,3,5,2,1,0".to_string());
    }
}
