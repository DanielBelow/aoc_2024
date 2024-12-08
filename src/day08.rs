use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use pathfinding::prelude::Matrix;
use std::collections::HashMap;

fn get_node_coord_mapping(matrix: &Matrix<char>) -> HashMap<char, Vec<(usize, usize)>> {
    matrix
        .keys()
        .filter(|&pos| matrix[pos] != '.')
        .map(|pos| (matrix[pos], pos))
        .into_group_map()
}

#[aoc_generator(day08)]
pub fn generate(s: &str) -> Option<Matrix<char>> {
    let v = s.lines().map(|l| l.chars().collect_vec()).collect_vec();
    Matrix::from_rows(v).ok()
}

#[aoc(day08, part1)]
#[allow(clippy::cast_possible_wrap)]
pub fn part1(inp: &Matrix<char>) -> usize {
    let mut anti_nodes = Matrix::new(inp.rows, inp.columns, false);

    for (_, pos) in get_node_coord_mapping(inp) {
        for pair in pos.iter().combinations(2) {
            assert_eq!(pair.len(), 2);

            let (sr, sc) = *pair[0];
            let (tr, tc) = *pair[1];

            let row_dist = sr as isize - tr as isize;
            let col_dist = sc as isize - tc as isize;

            anti_nodes
                .in_direction((sr, sc), (row_dist, col_dist))
                .take(1)
                .for_each(|pos| anti_nodes[pos] = true);

            anti_nodes
                .in_direction((tr, tc), (-row_dist, -col_dist))
                .take(1)
                .for_each(|pos| anti_nodes[pos] = true);
        }
    }

    anti_nodes.values().filter(|it| **it).count()
}

#[aoc(day08, part2)]
#[allow(clippy::cast_possible_wrap)]
pub fn part2(inp: &Matrix<char>) -> usize {
    let mut anti_nodes = Matrix::new(inp.rows, inp.columns, false);

    for (_, pos) in get_node_coord_mapping(inp) {
        for pair in pos.iter().combinations(2) {
            assert_eq!(pair.len(), 2);

            let (sr, sc) = *pair[0];
            let (tr, tc) = *pair[1];

            anti_nodes[(sr, sc)] = true;
            anti_nodes[(tr, tc)] = true;

            let row_dist = sr as isize - tr as isize;
            let col_dist = sc as isize - tc as isize;

            anti_nodes
                .in_direction((sr, sc), (row_dist, col_dist))
                .for_each(|pos| anti_nodes[pos] = true);

            anti_nodes
                .in_direction((tr, tc), (-row_dist, -col_dist))
                .for_each(|pos| anti_nodes[pos] = true);
        }
    }

    anti_nodes.values().filter(|it| **it).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "............\n\
                              ........0...\n\
                              .....0......\n\
                              .......0....\n\
                              ....0.......\n\
                              ......A.....\n\
                              ............\n\
                              ............\n\
                              ........A...\n\
                              .........A..\n\
                              ............\n\
                              ............";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT).expect("valid input");
        let res = part1(&gen);
        assert_eq!(res, 14);
    }

    #[test]
    fn test_p2() {
        let gen = generate(TEST_INPUT).expect("valid input");
        let res = part2(&gen);
        assert_eq!(res, 34);
    }
}
