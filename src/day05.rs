use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use pathfinding::prelude::topological_sort;

#[derive(Copy, Clone, Debug)]
pub struct PageOrderRule {
    lhs: usize,
    rhs: usize,
}

#[derive(Clone, Debug)]
pub struct Update {
    numbers: Vec<usize>,
}

#[derive(Clone, Debug)]
pub struct ParsedInput {
    order_rules: Vec<PageOrderRule>,
    updates: Vec<Update>,
}

impl Update {
    fn topological_sort(&self, order_rules: &[PageOrderRule]) -> Self {
        let grouping_map = order_rules
            .iter()
            .filter(|r| self.numbers.contains(&r.lhs) && self.numbers.contains(&r.rhs))
            .map(|r| (r.lhs, r.rhs))
            .into_grouping_map()
            .collect();

        let numbers = topological_sort(&self.numbers, |node: &usize| {
            grouping_map
                .get(node)
                .map_or_else(Vec::new, std::clone::Clone::clone)
        })
        .expect("valid topo sort exists");

        Self { numbers }
    }

    fn is_valid(&self, order_rules: &[PageOrderRule]) -> bool {
        self.topological_sort(order_rules).numbers == self.numbers
    }

    fn get_middle_number(&self) -> usize {
        let len = self.numbers.len();
        self.numbers[(len - 1) / 2]
    }
}

#[aoc_generator(day05)]
pub fn generate(s: &str) -> ParsedInput {
    let spl = s.split("\n\n").collect_vec();

    let mut order_rules = vec![];

    for line in spl[0].lines() {
        let spl = line.split('|').collect_vec();
        let lhs = spl[0].parse::<usize>().expect("lhs|rhs");
        let rhs = spl[1].parse::<usize>().expect("lhs|rhs");
        order_rules.push(PageOrderRule { lhs, rhs });
    }

    let updates = spl[1]
        .lines()
        .map(|l| {
            let numbers = l
                .split(',')
                .map(|s| s.parse::<usize>().expect("update_number"))
                .collect_vec();

            Update { numbers }
        })
        .collect_vec();

    ParsedInput {
        order_rules,
        updates,
    }
}

#[aoc(day05, part1)]
pub fn part1(inp: &ParsedInput) -> usize {
    inp.updates.iter().fold(0, |acc, it| {
        acc + if it.is_valid(&inp.order_rules) {
            it.get_middle_number()
        } else {
            0
        }
    })
}

#[aoc(day05, part2)]
pub fn part2(inp: &ParsedInput) -> usize {
    inp.updates
        .iter()
        .filter(|u| !u.is_valid(&inp.order_rules))
        .fold(0, |acc, it| {
            acc + it.topological_sort(&inp.order_rules).get_middle_number()
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "47|53\n\
                              97|13\n\
                              97|61\n\
                              97|47\n\
                              75|29\n\
                              61|13\n\
                              75|53\n\
                              29|13\n\
                              97|29\n\
                              53|29\n\
                              61|53\n\
                              97|53\n\
                              61|29\n\
                              47|13\n\
                              75|47\n\
                              97|75\n\
                              47|61\n\
                              75|61\n\
                              47|29\n\
                              75|13\n\
                              53|13\n\
                              \n\
                              75,47,61,53,29\n\
                              97,61,53,29,13\n\
                              75,29,13\n\
                              75,97,47,61,53\n\
                              61,13,29\n\
                              97,13,75,29,47";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT);
        let res = part1(&gen);
        assert_eq!(res, 143);
    }

    #[test]
    fn test_p2() {
        let gen = generate(TEST_INPUT);
        let res = part2(&gen);
        assert_eq!(res, 123);
    }
}
