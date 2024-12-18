use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use pathfinding::matrix::Matrix;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Coord {
    row: usize,
    col: usize,
}

fn parse_line(line: &str) -> Coord {
    let (lhs, rhs) = line.split_once(',').expect("valid input");
    let lhs = lhs.parse().expect("number");
    let rhs = rhs.parse().expect("number");

    Coord { row: rhs, col: lhs }
}

#[aoc_generator(day18)]
pub fn generate(s: &str) -> Vec<Coord> {
    s.lines().map(parse_line).collect_vec()
}

fn build_grid(rows: usize, cols: usize) -> Matrix<char> {
    Matrix::new(rows, cols, '.')
}

fn bytes_fall(num_bytes: usize, coords: &[Coord], grid: &mut Matrix<char>) {
    for &coord in coords.iter().take(num_bytes) {
        grid[(coord.row, coord.col)] = '#';
    }
}

fn find_path(grid: &Matrix<char>, start: Coord, end: Coord) -> Option<(Vec<Coord>, usize)> {
    pathfinding::prelude::dijkstra(
        &start,
        |&coord| {
            let mut succs = vec![];

            for (dr, dc) in [(1isize, 0isize), (0, 1), (-1, 0), (0, -1)] {
                if let (Some(nr), Some(nc)) = (
                    coord.row.checked_add_signed(dr),
                    coord.col.checked_add_signed(dc),
                ) {
                    if grid.get((nr, nc)).is_some_and(|c| *c != '#') {
                        succs.push((Coord { row: nr, col: nc }, 1));
                    }
                }
            }

            succs
        },
        |&pos| pos == end,
    )
}

fn find_first_blocking(
    start: Coord,
    end: Coord,
    to_skip: usize,
    coords: &[Coord],
    grid: &mut Matrix<char>,
) -> Option<Coord> {
    // fill initial state
    bytes_fall(to_skip, coords, grid);

    let (mut path, _) = find_path(grid, start, end).expect("path exists");

    for &coord in &coords[to_skip..] {
        grid[(coord.row, coord.col)] = '#';

        if !path.contains(&coord) {
            // definitely not blocking our path
            continue;
        }

        match find_path(grid, start, end) {
            None => return Some(coord),
            Some((new_path, _)) => path = new_path,
        };
    }

    None
}

#[aoc(day18, part1)]
pub fn part1(inp: &[Coord]) -> usize {
    const START: Coord = Coord { row: 0, col: 0 };
    const END: Coord = Coord { row: 70, col: 70 };

    let mut grid = build_grid(71, 71);
    bytes_fall(1024, inp, &mut grid);

    let (_, cost) = find_path(&grid, START, END).expect("path exists");
    cost
}

#[aoc(day18, part2)]
pub fn part2(inp: &[Coord]) -> String {
    const START: Coord = Coord { row: 0, col: 0 };
    const END: Coord = Coord { row: 70, col: 70 };
    const TO_SKIP: usize = 1024; // known from p1 to still have a valid path

    let mut grid = build_grid(71, 71);

    let coord = find_first_blocking(START, END, TO_SKIP, inp, &mut grid).expect("path was blocked");
    format!("{},{}", coord.col, coord.row)
}

#[cfg(test)]
mod tests {
    use super::*;

    const START: Coord = Coord { row: 0, col: 0 };
    const END: Coord = Coord { row: 6, col: 6 };

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

        let (_, cost) = find_path(&grid, START, END).expect("path exists");
        assert_eq!(cost, 22);
    }

    #[test]
    fn test_p2() {
        const TO_SKIP: usize = 12;

        let gen = generate(TEST_INPUT);
        let mut grid = build_grid(7, 7);
        let coord =
            find_first_blocking(START, END, TO_SKIP, &gen, &mut grid).expect("path was blocked");
        assert_eq!(coord, Coord { row: 1, col: 6 });
    }
}
