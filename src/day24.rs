use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display_derive::{Display, FromStr};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Display, FromStr, PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Operator {
    #[display("AND")]
    And,

    #[display("OR")]
    Or,

    #[display("XOR")]
    Xor,
}
#[derive(Display, FromStr, PartialEq, Eq, Hash, Clone, Debug)]
#[display("{lhs} {kind} {rhs} -> {output}")]
pub struct Operation {
    lhs: String,
    rhs: String,
    kind: Operator,
    output: String,
}

impl Operation {
    fn evaluate<'a>(
        &'a self,
        operations: &'a [Self],
        values: &mut HashMap<&'a str, usize>,
        eval_stack: &[&'a str],
    ) -> Option<usize> {
        if eval_stack.contains(&self.lhs.as_str()) || eval_stack.contains(&self.rhs.as_str()) {
            return None;
        }

        let lhs = values.get(self.lhs.as_str()).copied().or_else(|| {
            let op = operations
                .iter()
                .find(|op| op.output == self.lhs)
                .expect("exists");
            let mut eval_stack = eval_stack.to_vec();
            eval_stack.push(self.lhs.as_str());
            op.evaluate(operations, values, &eval_stack)
        });

        let rhs = values.get(self.rhs.as_str()).copied().or_else(|| {
            let op = operations
                .iter()
                .find(|op| op.output == self.rhs)
                .expect("exists");
            let mut eval_stack = eval_stack.to_vec();
            eval_stack.push(self.rhs.as_str());
            op.evaluate(operations, values, &eval_stack)
        });

        if let Some(lhs) = lhs {
            if let Some(rhs) = rhs {
                let result = match self.kind {
                    Operator::And => lhs & rhs,
                    Operator::Or => lhs | rhs,
                    Operator::Xor => lhs ^ rhs,
                };

                values.entry(self.output.as_str()).insert_entry(result);
                return Some(result);
            }
        }

        None
    }
}

#[derive(Clone, Debug)]
pub struct Input {
    initial_values: HashMap<String, usize>,
    operations: Vec<Operation>,
}

#[aoc_generator(day24)]
pub fn generate(s: &str) -> Option<Input> {
    let (initial, insts) = s.split_once("\n\n")?;

    let mut initial_values = HashMap::new();
    for line in initial.lines() {
        let (name, value) = line.split_once(": ")?;
        let value = value.parse::<usize>().ok()?;

        initial_values.insert(name.to_string(), value);
    }

    let mut operations = vec![];
    for op in insts.lines() {
        let inst = op.parse::<Operation>().ok()?;
        operations.push(inst);
    }

    Some(Input {
        initial_values,
        operations,
    })
}

fn evaluate<'a>(
    operations: &'a [Operation],
    values: &mut HashMap<&'a str, usize>,
) -> Option<usize> {
    let z_wires = operations
        .iter()
        .filter(|op| op.output.starts_with('z'))
        .sorted_by_key(|op| &op.output)
        .collect_vec();

    let mut result = 0;
    for op in z_wires.iter().rev() {
        let bit = op.evaluate(operations, values, &[])?;
        result = (result << 1) | bit;
    }

    Some(result)
}

#[aoc(day24, part1)]
pub fn part1(conns: &Input) -> Option<usize> {
    let mut values = HashMap::new();
    for (k, v) in &conns.initial_values {
        values.entry(k.as_str()).insert_entry(*v);
    }

    evaluate(&conns.operations, &mut values)
}

#[aoc(day24, part2)]
pub fn part2(conns: &Input) -> Option<String> {
    let mut invalid = HashSet::new();

    for op in &conns.operations {
        if op.output.starts_with('z') && op.kind != Operator::Xor && op.output != "z45" {
            invalid.insert(op.output.clone());
        }

        if op.kind == Operator::Xor
            && !['x', 'y', 'z'].contains(&op.output.chars().next()?)
            && !['x', 'y', 'z'].contains(&op.lhs.chars().next()?)
            && !['x', 'y', 'z'].contains(&op.rhs.chars().next()?)
        {
            invalid.insert(op.output.clone());
        }

        if op.kind == Operator::And && op.lhs != "x00" && op.rhs != "x00" {
            for other_op in &conns.operations {
                if (op.output == other_op.lhs || op.output == other_op.rhs)
                    && other_op.kind != Operator::Or
                {
                    invalid.insert(op.output.clone());
                }
            }
        }

        if op.kind == Operator::Xor {
            for other_op in &conns.operations {
                if (op.output == other_op.lhs || op.output == other_op.rhs)
                    && other_op.kind == Operator::Or
                {
                    invalid.insert(op.output.clone());
                }
            }
        }
    }

    assert_eq!(invalid.len(), 8);

    Some(invalid.iter().sorted().join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_P1: &str = "x00: 1\n\
                                 x01: 1\n\
                                 x02: 1\n\
                                 y00: 0\n\
                                 y01: 1\n\
                                 y02: 0\n\
                                 \n\
                                 x00 AND y00 -> z00\n\
                                 x01 XOR y01 -> z01\n\
                                 x02 OR y02 -> z02";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT_P1).expect("valid input");
        let res = part1(&gen);
        assert_eq!(res, Some(4));
    }
}
