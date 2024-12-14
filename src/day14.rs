use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display_derive::{Display, FromStr};
use pathfinding::prelude::Matrix;

#[derive(Display, FromStr, Debug, Copy, Clone)]
#[display("p={px},{py} v={vx},{vy}")]
pub struct Entry {
    px: isize,
    py: isize,
    vx: isize,
    vy: isize,
}

#[aoc_generator(day14)]
pub fn generate(s: &str) -> Vec<Entry> {
    s.lines().filter_map(|line| line.parse().ok()).collect_vec()
}

fn simulate(entries: &mut [Entry], width: isize, height: isize) {
    for entry in entries {
        entry.px = (entry.px + entry.vx).rem_euclid(width);
        entry.py = (entry.py + entry.vy).rem_euclid(height);
    }
}

fn entries_to_grid(entries: &[Entry], width: usize, height: usize) -> Matrix<usize> {
    let mut grid = Matrix::new(height, width, 0);

    for entry in entries {
        grid[(entry.py as usize, entry.px as usize)] += 1;
    }

    grid
}

fn run_part1(entries: &mut [Entry], width: isize, height: isize) -> usize {
    for _ in 0..100 {
        simulate(entries, width, height);
    }

    let rows = height as usize / 2;
    let cols = width as usize / 2;

    let mut top_left = 0;
    let mut top_right = 0;
    let mut bot_left = 0;
    let mut bot_right = 0;

    let grid = entries_to_grid(entries, width as usize, height as usize);
    for (r, c) in grid.keys() {
        if r < rows && c < cols {
            top_left += grid[(r, c)];
        } else if r > rows && c < cols {
            top_right += grid[(r, c)];
        } else if r < rows && c > cols {
            bot_left += grid[(r, c)];
        } else if r > rows && c > cols {
            bot_right += grid[(r, c)];
        }
    }

    top_left * top_right * bot_left * bot_right
}

#[aoc(day14, part1)]
pub fn part1(inp: &[Entry]) -> usize {
    const WIDTH: isize = 101;
    const HEIGHT: isize = 103;

    let mut entries = inp.to_vec();
    run_part1(&mut entries, WIDTH, HEIGHT)
}

// stupid heuristic checking >7 values > 0 in a row/column
fn has_tree(grid: &Matrix<usize>) -> bool {
    for (r, c) in grid.keys() {
        if grid[(r, c)] > 0 {
            let has_enough_x = grid
                .in_direction((r, c), (0, 1))
                .take_while(|&(r, c)| grid[(r, c)] > 0)
                .count()
                > 7;
            let has_enough_y = grid
                .in_direction((r, c), (1, 0))
                .take_while(|&(r, c)| grid[(r, c)] > 0)
                .count()
                > 7;

            if has_enough_x && has_enough_y {
                return true;
            }
        }
    }

    false
}

#[aoc(day14, part2)]
pub fn part2(inp: &[Entry]) -> usize {
    const WIDTH: isize = 101;
    const HEIGHT: isize = 103;

    let mut entries = inp.to_vec();

    let mut idx = 0;
    loop {
        idx += 1;
        simulate(&mut entries, WIDTH, HEIGHT);

        let grid = entries_to_grid(&entries, WIDTH as usize, HEIGHT as usize);
        if has_tree(&grid) {
            return idx;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "p=0,4 v=3,-3\n\
                              p=6,3 v=-1,-3\n\
                              p=10,3 v=-1,2\n\
                              p=2,0 v=2,-1\n\
                              p=0,0 v=1,3\n\
                              p=3,0 v=-2,-2\n\
                              p=7,6 v=-1,-3\n\
                              p=3,0 v=-1,-2\n\
                              p=9,3 v=2,3\n\
                              p=7,3 v=-1,2\n\
                              p=2,4 v=2,-3\n\
                              p=9,5 v=-3,-3";

    #[test]
    fn test_p1() {
        let mut gen = generate(TEST_INPUT);
        let res = run_part1(&mut gen, 11, 7);
        assert_eq!(res, 12);
    }
}
