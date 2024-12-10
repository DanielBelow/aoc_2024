use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use pathfinding::matrix::Matrix;

#[derive(Clone)]
pub struct Input {
    grid: Matrix<usize>,
    start_end_nodes: StartEndPoints,
}

#[derive(Clone)]
pub struct StartEndPoints {
    starts: Vec<(usize, usize)>,
    ends: Vec<(usize, usize)>,
}

#[aoc_generator(day10)]
pub fn generate(s: &str) -> Option<Input> {
    let v = s
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).expect("valid digit") as usize)
                .collect_vec()
        })
        .collect_vec();
    let grid = Matrix::from_rows(v).ok()?;

    let starts = grid.keys().filter(|&pos| grid[pos] == 0).collect_vec();
    let ends = grid.keys().filter(|&pos| grid[pos] == 9).collect_vec();
    Some(Input {
        grid,
        start_end_nodes: StartEndPoints { starts, ends },
    })
}

fn successors((r, c): (usize, usize), grid: &Matrix<usize>) -> Vec<(usize, usize)> {
    let mut succs = vec![];

    let next_height = grid[(r, c)] + 1;

    for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        if let (Some(new_r), Some(new_c)) = (r.checked_add_signed(dr), c.checked_add_signed(dc)) {
            if let Some(node) = grid.get((new_r, new_c)) {
                if *node == next_height {
                    succs.push((new_r, new_c));
                }
            }
        }
    }

    succs
}

#[aoc(day10, part1)]
pub fn part1(inp: &Input) -> usize {
    let StartEndPoints { starts, ends } = &inp.start_end_nodes;

    starts.iter().fold(0, |acc, &start| {
        acc + pathfinding::prelude::dijkstra_all(&start, |&pos| {
            successors(pos, &inp.grid)
                .into_iter()
                .zip(std::iter::repeat(1))
                .collect_vec()
        })
        .iter()
        .filter(|(node, _)| ends.contains(node))
        .count()
    })
}

#[aoc(day10, part2)]
pub fn part2(inp: &Input) -> usize {
    let StartEndPoints { starts, ends } = &inp.start_end_nodes;

    starts.iter().fold(0, |acc, &start| {
        acc + pathfinding::prelude::count_paths(
            start,
            |&pos| successors(pos, &inp.grid),
            |goal| ends.contains(goal),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "89010123\n\
                              78121874\n\
                              87430965\n\
                              96549874\n\
                              45678903\n\
                              32019012\n\
                              01329801\n\
                              10456732";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT).expect("valid input");
        let res = part1(&gen);
        assert_eq!(res, 36);
    }

    #[test]
    fn test_p2() {
        let gen = generate(TEST_INPUT).expect("valid input");
        let res = part2(&gen);
        assert_eq!(res, 81);
    }
}
