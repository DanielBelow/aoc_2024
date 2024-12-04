use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{iproduct, Itertools};
use pathfinding::matrix::Matrix;
use std::iter::Iterator;

#[aoc_generator(day04)]
pub fn generate(s: &str) -> Option<Matrix<char>> {
    let v = s.lines().map(|l| l.chars().collect_vec()).collect_vec();
    Matrix::from_rows(v).ok()
}

fn get_n_chrs_in_dir(
    matrix: &Matrix<char>,
    n: usize,
    (x, y): (usize, usize),
    (dx, dy): (isize, isize),
) -> Vec<char> {
    matrix
        .in_direction((x, y), (dx, dy))
        .take(n)
        .filter_map(|(x, y)| matrix.get((x, y)))
        .copied()
        .collect_vec()
}

const MAS: [char; 3] = ['M', 'A', 'S'];

#[aoc(day04, part1)]
pub fn part1(inp: &Matrix<char>) -> usize {
    let mut result = 0;

    for (r, c) in inp.keys().filter(|&(x, y)| inp[(x, y)] == 'X') {
        for (dx, dy) in [
            (0, 1),
            (1, 0),
            (0, -1),
            (-1, 0),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ] {
            let n_chrs = get_n_chrs_in_dir(inp, 3, (r, c), (dx, dy));
            result += usize::from(n_chrs == MAS);
        }
    }

    result
}

#[aoc(day04, part2)]
pub fn part2(inp: &Matrix<char>) -> usize {
    let mut result = 0;

    for (r, c) in
        iproduct!(0..inp.rows - 2, 0..inp.columns - 2).filter(|&(x, y)| inp[(x + 1, y + 1)] == 'A')
    {
        let diag_ok = (inp[(r, c)] == 'M' && inp[(r + 2, c + 2)] == 'S')
            || (inp[(r, c)] == 'S' && inp[(r + 2, c + 2)] == 'M');

        let anti_diag_ok = (inp[(r, c + 2)] == 'M' && inp[(r + 2, c)] == 'S')
            || (inp[(r, c + 2)] == 'S' && inp[(r + 2, c)] == 'M');

        result += usize::from(diag_ok && anti_diag_ok);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "MMMSXXMASM\n\
                              MSAMXMSMSA\n\
                              AMXSXMAAMM\n\
                              MSAMASMSMX\n\
                              XMASAMXAMM\n\
                              XXAMMXXAMA\n\
                              SMSMSASXSS\n\
                              SAXAMASAAA\n\
                              MAMMMXMMMM\n\
                              MXMXAXMASX";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT).expect("valid input");
        let res = part1(&gen);
        assert_eq!(res, 18);
    }

    #[test]
    fn test_p2() {
        let gen = generate(TEST_INPUT).expect("valid input");
        let res = part2(&gen);
        assert_eq!(res, 9);
    }
}
