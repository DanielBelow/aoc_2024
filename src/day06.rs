use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{iproduct, Itertools};
use num::Complex;
use pathfinding::prelude::Matrix;
use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub struct Guard {
    pos: Complex<i64>,
    direction: Complex<i64>,
}

impl Guard {
    fn turn_right(&mut self) {
        self.direction *= -Complex::i();
    }

    fn move_forward(&mut self) {
        self.pos += self.direction;
    }

    fn next_coord(&self) -> (usize, usize) {
        let next = self.pos + self.direction;
        (next.re as usize, next.im as usize)
    }
}

#[aoc_generator(day06)]
pub fn generate(s: &str) -> Option<Matrix<char>> {
    let v = s.lines().map(|l| l.chars().collect_vec()).collect_vec();
    Matrix::from_rows(v).ok()
}

fn walk_path(mut guard: Guard, grid: &Matrix<char>) -> Vec<Guard> {
    let mut seen = vec![guard];

    while let Some(c) = grid.get(guard.next_coord()) {
        if *c == '.' {
            guard.move_forward();
            seen.push(guard);
            continue;
        }

        if *c == '#' {
            guard.turn_right();
        }
    }

    seen
}

#[aoc(day06, part1)]
#[allow(clippy::cast_possible_wrap)]
pub fn part1(inp: &Matrix<char>) -> usize {
    let mut grid = inp.clone();

    let (start_row, start_col) = grid
        .keys()
        .find(|&(r, c)| grid[(r, c)] == '^')
        .expect("start_row, start_col");

    grid[(start_row, start_col)] = '.';

    let guard = Guard {
        pos: Complex::new(start_row as i64, start_col as i64),
        direction: Complex::new(-1, 0),
    };

    let path = walk_path(guard, &grid);
    path.iter().map(|g| g.pos).unique().count()
}

#[aoc(day06, part2)]
#[allow(clippy::cast_possible_wrap)]
pub fn part2(inp: &Matrix<char>) -> usize {
    let mut grid = inp.clone();

    let (start_row, start_col) = grid
        .keys()
        .find(|&(r, c)| grid[(r, c)] == '^')
        .expect("start_row, start_col");

    grid[(start_row, start_col)] = '.';

    let mut num_loops = 0;

    let start_guard = Guard {
        pos: Complex::new(start_row as i64, start_col as i64),
        direction: Complex::new(-1, 0),
    };

    let real_path = walk_path(start_guard, &grid)
        .iter()
        .map(|g| (g.pos.re as usize, g.pos.im as usize))
        .unique()
        .collect_vec();

    for (r, c) in iproduct!(0..inp.rows, 0..inp.columns) {
        if grid[(r, c)] == '#' || !real_path.contains(&(r, c)) || (r == start_row && c == start_col)
        {
            continue;
        }

        grid[(r, c)] = '#';

        let mut guard = start_guard;

        let mut seen = HashSet::new();
        while let Some(chr) = grid.get(guard.next_coord()) {
            if !seen.insert(guard) {
                num_loops += 1;
                break;
            }

            match *chr {
                '#' => guard.turn_right(),
                '.' => guard.move_forward(),
                _ => {}
            };
        }

        grid[(r, c)] = '.';
    }

    num_loops
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "....#.....\n\
                              .........#\n\
                              ..........\n\
                              ..#.......\n\
                              .......#..\n\
                              ..........\n\
                              .#..^.....\n\
                              ........#.\n\
                              #.........\n\
                              ......#...";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT).expect("valid input");
        let res = part1(&gen);
        assert_eq!(res, 41);
    }

    #[test]
    fn test_p2() {
        let gen = generate(TEST_INPUT).expect("valid input");
        let res = part2(&gen);
        assert_eq!(res, 6);
    }
}
