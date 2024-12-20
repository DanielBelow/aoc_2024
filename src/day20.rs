use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use pathfinding::matrix::Matrix;

#[aoc_generator(day20)]
pub fn generate(s: &str) -> Option<Matrix<char>> {
    let v = s.lines().map(|l| l.chars().collect_vec()).collect_vec();
    Matrix::from_rows(v).ok()
}

fn find_node(pos: char, grid: &Matrix<char>) -> Option<(usize, usize)> {
    grid.keys().find(|&(r, c)| grid[(r, c)] == pos)
}

fn successors((r, c): (usize, usize), grid: &Matrix<char>) -> Vec<((usize, usize), usize)> {
    let mut res = vec![];

    for (dr, dc) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        if let (Some(new_r), Some(new_c)) = (r.checked_add_signed(dr), c.checked_add_signed(dc)) {
            if grid.get((new_r, new_c)).is_some_and(|c| *c == '.') {
                res.push(((new_r, new_c), 1));
            }
        }
    }

    res
}

fn run_part1(inp: &Matrix<char>, cost_diff: usize) -> Option<usize> {
    let start = find_node('S', inp)?;
    let end = find_node('E', inp)?;

    let mut grid = inp.clone();

    grid[start] = '.';
    grid[end] = '.';

    let paths = pathfinding::prelude::dijkstra_all(&start, |pos| successors(*pos, &grid));

    let mut count = 0;

    for (&(r, c), (_, cost)) in &paths {
        for (dr, dc) in [(-2, 0), (2, 0), (0, -2), (0, 2)] {
            if let (Some(new_r), Some(new_c)) = (r.checked_add_signed(dr), c.checked_add_signed(dc))
            {
                if let Some((_, next_cost)) = paths.get(&(new_r, new_c)) {
                    if next_cost.saturating_sub(*cost) >= cost_diff + 2 {
                        count += 1;
                    }
                }
            }
        }
    }

    Some(1 + count)
}

fn run_part2(inp: &Matrix<char>, cost_diff: usize) -> Option<usize> {
    let start = find_node('S', inp)?;
    let end = find_node('E', inp)?;

    let mut grid = inp.clone();

    grid[start] = '.';
    grid[end] = '.';

    let (path, _) =
        pathfinding::prelude::dijkstra(&start, |pos| successors(*pos, &grid), |&pos| pos == end)?;

    let mut count = 0;

    for cheat_start_idx in 0..path.len() {
        for cheat_end_idx in cheat_start_idx + 1..path.len() {
            let n1 = path[cheat_start_idx];
            let n2 = path[cheat_end_idx];
            let distance = n1.0.abs_diff(n2.0) + n1.1.abs_diff(n2.1);
            if distance <= 20 && cheat_end_idx - cheat_start_idx >= cost_diff + distance {
                count += 1;
            }
        }
    }

    Some(count)
}

#[aoc(day20, part1)]
pub fn part1(inp: &Matrix<char>) -> Option<usize> {
    run_part1(inp, 100)
}

#[aoc(day20, part2)]
pub fn part2(inp: &Matrix<char>) -> Option<usize> {
    run_part2(inp, 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "###############\n\
                              #...#...#.....#\n\
                              #.#.#.#.#.###.#\n\
                              #S#...#.#.#...#\n\
                              #######.#.#.###\n\
                              #######.#.#...#\n\
                              #######.#.###.#\n\
                              ###..E#...#...#\n\
                              ###.#######.###\n\
                              #...###...#...#\n\
                              #.#####.#.###.#\n\
                              #.#...#.#.#...#\n\
                              #.#.#.#.#.#.###\n\
                              #...#...#...###\n\
                              ###############";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT).expect("valid input");
        let res = run_part1(&gen, 1);
        assert_eq!(res, Some(44));
    }

    #[test]
    fn test_p2() {
        let gen = generate(TEST_INPUT).expect("valid input");
        let res = run_part2(&gen, 50);
        assert_eq!(res, Some(285));
    }
}
