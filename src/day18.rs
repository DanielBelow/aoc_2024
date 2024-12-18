use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use pathfinding::matrix::Matrix;

fn parse_line(line: &str) -> (usize, usize) {
    let (lhs, rhs) = line.split_once(',').expect("valid input");
    (lhs.parse().expect("number"), rhs.parse().expect("number"))
}

#[aoc_generator(day18)]
pub fn generate(s: &str) -> Vec<(usize, usize)> {
    s.lines().map(parse_line).collect_vec()
}

fn build_grid(rows: usize, cols: usize) -> Matrix<char> {
    Matrix::new(rows, cols, '.')
}

fn bytes_fall(num_bytes: usize, coords: &[(usize, usize)], grid: &mut Matrix<char>) {
    for &(c, r) in coords.iter().take(num_bytes) {
        grid[(r, c)] = '#';
    }
}

fn find_path(
    grid: &Matrix<char>,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<(Vec<(usize, usize)>, usize)> {
    pathfinding::prelude::dijkstra(
        &start,
        |&(r, c)| {
            let mut succs = vec![];

            for (dr, dc) in [(1isize, 0isize), (0, 1), (-1, 0), (0, -1)] {
                if let (Some(nr), Some(nc)) = (r.checked_add_signed(dr), c.checked_add_signed(dc)) {
                    if grid.get((nr, nc)).is_some_and(|c| *c != '#') {
                        succs.push(((nr, nc), 1));
                    }
                }
            }

            succs
        },
        |&pos| pos == end,
    )
}

fn find_first_blocking(
    start: (usize, usize),
    end: (usize, usize),
    coords: &[(usize, usize)],
    grid: &Matrix<char>,
) -> Option<(usize, usize)> {
    for num_bytes in (0..coords.len()).rev() {
        let mut grid = grid.clone();
        bytes_fall(num_bytes, coords, &mut grid);

        if find_path(&grid, start, end).is_some() {
            return Some(coords[num_bytes]);
        }
    }

    None
}

#[aoc(day18, part1)]
pub fn part1(inp: &[(usize, usize)]) -> usize {
    let mut grid = build_grid(71, 71);
    bytes_fall(1024, inp, &mut grid);

    let (_, cost) = find_path(&grid, (0, 0), (70, 70)).expect("path exists");
    cost
}

#[aoc(day18, part2)]
pub fn part2(inp: &[(usize, usize)]) -> String {
    let grid = build_grid(71, 71);

    let (r, c) = find_first_blocking((0, 0), (70, 70), inp, &grid).expect("path was blocked");
    format!("{r},{c}")
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "5,4\n\
                              4,2\n\
                              4,5\n\
                              3,0\n\
                              2,1\n\
                              6,3\n\
                              2,4\n\
                              1,5\n\
                              0,6\n\
                              3,3\n\
                              2,6\n\
                              5,1\n\
                              1,2\n\
                              5,5\n\
                              2,5\n\
                              6,5\n\
                              1,4\n\
                              0,4\n\
                              6,4\n\
                              1,1\n\
                              6,1\n\
                              1,0\n\
                              0,5\n\
                              1,6\n\
                              2,0";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT);
        let mut grid = build_grid(7, 7);
        bytes_fall(12, &gen, &mut grid);
        let (_, cost) = find_path(&grid, (0, 0), (6, 6)).expect("path exists");
        assert_eq!(cost, 22);
    }

    #[test]
    fn test_p2() {
        let gen = generate(TEST_INPUT);
        let grid = build_grid(7, 7);
        let (r, c) = find_first_blocking((0, 0), (6, 6), &gen, &grid).expect("path was blocked");
        assert_eq!(r, 6);
        assert_eq!(c, 1);
    }
}
