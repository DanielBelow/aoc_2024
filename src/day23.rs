use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Connection {
    lhs: String,
    rhs: String,
}

#[aoc_generator(day23)]
pub fn generate(s: &str) -> HashMap<String, Vec<String>> {
    let mut result = HashMap::new();

    for line in s.lines() {
        let (lhs, rhs) = line.split_once('-').expect("valid input");

        result
            .entry(lhs.to_string())
            .or_insert(vec![])
            .push(rhs.to_string());

        result
            .entry(rhs.to_string())
            .or_insert(vec![])
            .push(lhs.to_string());
    }

    result
}

fn extract_triplets(conns: &HashMap<String, Vec<String>>) -> HashSet<[&String; 3]> {
    let mut res = HashSet::new();

    for (k, v) in conns {
        for c1 in v {
            if let Some(c1s) = conns.get(c1) {
                for c2 in c1s {
                    if let Some(c2s) = conns.get(c2) {
                        if c2s.contains(k) {
                            let mut arr = [k, c1, c2];
                            arr.sort();
                            res.insert(arr);
                        }
                    }
                }
            }
        }
    }

    res
}

fn collect_cliques(conns: &HashMap<String, Vec<String>>) -> Vec<Vec<String>> {
    let mut all_cliques: Vec<Vec<String>> = vec![];
    for (node, succs) in conns.iter().sorted() {
        if let Some(slot) = all_cliques
            .iter_mut()
            .find(|cl| cl.iter().all(|c| succs.contains(c)))
        {
            slot.push(node.clone());
        } else {
            all_cliques.push(vec![node.clone()]);
        }
    }

    all_cliques
}

#[aoc(day23, part1)]
pub fn part1(conns: &HashMap<String, Vec<String>>) -> usize {
    let triplets = extract_triplets(conns);
    triplets
        .iter()
        .filter(|it| it[0].starts_with('t') || it[1].starts_with('t') || it[2].starts_with('t'))
        .count()
}

#[aoc(day23, part2)]
pub fn part2(conns: &HashMap<String, Vec<String>>) -> String {
    let all_cliques = collect_cliques(conns);

    let max_clique = all_cliques
        .iter()
        .max_by_key(|it| it.len())
        .expect("max exists");
    max_clique.iter().sorted().join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "kh-tc\n\
                              qp-kh\n\
                              de-cg\n\
                              ka-co\n\
                              yn-aq\n\
                              qp-ub\n\
                              cg-tb\n\
                              vc-aq\n\
                              tb-ka\n\
                              wh-tc\n\
                              yn-cg\n\
                              kh-ub\n\
                              ta-co\n\
                              de-co\n\
                              tc-td\n\
                              tb-wq\n\
                              wh-td\n\
                              ta-ka\n\
                              td-qp\n\
                              aq-cg\n\
                              wq-ub\n\
                              ub-vc\n\
                              de-ta\n\
                              wq-aq\n\
                              wq-vc\n\
                              wh-yn\n\
                              ka-de\n\
                              kh-ta\n\
                              co-tc\n\
                              wh-qp\n\
                              tb-vc\n\
                              td-yn";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT);
        let res = part1(&gen);
        assert_eq!(res, 7);
    }

    #[test]
    fn test_p2() {
        let gen = generate(TEST_INPUT);
        let res = part2(&gen);
        assert_eq!(res, "co,de,ka,ta".to_string());
    }
}
