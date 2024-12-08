use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use pathfinding::prelude::Matrix;
use std::collections::HashMap;

pub struct Input {
    grid: Matrix<char>,
    coord_mapping: HashMap<char, Vec<(usize, usize)>>,
}

#[aoc_generator(day08)]
pub fn generate(s: &str) -> Option<Input> {
    let v = s.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let grid = Matrix::from_rows(v).ok()?;
    let coord_mapping = grid
        .keys()
        .filter(|&pos| grid[pos] != '.')
        .map(|pos| (grid[pos], pos))
        .into_group_map();

    Some(Input {
        grid,
        coord_mapping,
    })
}

#[aoc(day08, part1)]
pub fn part1(inp: &Input) -> usize {
    let mut anti_nodes = Matrix::new(inp.grid.rows, inp.grid.columns, false);

    for pos in inp.coord_mapping.values() {
        for &(sr, sc) in pos {
            for &(tr, tc) in pos {
                if sr == tr && sc == tc {
                    continue;
                }

                if let Some(n) = anti_nodes.get_mut((2 * sr - tr, 2 * sc - tc)) {
                    *n = true;
                }

                if let Some(n) = anti_nodes.get_mut((2 * tr - sr, 2 * tc - sc)) {
                    *n = true;
                }
            }
        }
    }

    anti_nodes.values().filter(|it| **it).count()
}

#[aoc(day08, part2)]
#[allow(clippy::cast_possible_wrap)]
pub fn part2(inp: &Input) -> usize {
    let mut anti_nodes = Matrix::new(inp.grid.rows, inp.grid.columns, false);

    for pos in inp.coord_mapping.values() {
        for &(sr, sc) in pos {
            for &(tr, tc) in pos {
                if sr == tr && sc == tc {
                    continue;
                }

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
