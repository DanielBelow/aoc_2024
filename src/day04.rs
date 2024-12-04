use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{iproduct, EitherOrBoth, Itertools};
use pathfinding::matrix::Matrix;
use std::iter::Iterator;

const MAS: [char; 3] = ['M', 'A', 'S'];

#[aoc_generator(day04)]
pub fn generate(s: &str) -> Option<Matrix<char>> {
    let v = s.lines().map(|l| l.chars().collect_vec()).collect_vec();
    Matrix::from_rows(v).ok()
}

fn check_chars_in_dir(matrix: &Matrix<char>, start: (usize, usize), dir: (isize, isize)) -> bool {
    matrix
        .in_direction(start, dir)
        .take(MAS.len())
        .zip_longest(MAS)
        .all(|it| match it {
            EitherOrBoth::Both((r, c), expected) => matrix[(r, c)] == expected,
            _ => false,
        })
}

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
            let n_chrs = check_chars_in_dir(inp, (r, c), (dx, dy));
            result += usize::from(n_chrs);
        }
    }

    result
}

#[aoc(day04, part2)]
pub fn part2(inp: &Matrix<char>) -> usize {
    iproduct!(1..inp.rows - 1, 1..inp.columns - 1)
        .filter(|&(x, y)| inp[(x, y)] == 'A')
        .fold(0, |acc, (r, c)| {
            let top_left = inp[(r - 1, c - 1)];
            let bot_right = inp[(r + 1, c + 1)];

            let top_right = inp[(r - 1, c + 1)];
            let bot_left = inp[(r + 1, c - 1)];

            let diag_ok =
                (top_left == 'M' && bot_right == 'S') || (top_left == 'S' && bot_right == 'M');

            let anti_diag_ok =
                (top_right == 'M' && bot_left == 'S') || (top_right == 'S' && bot_left == 'M');

            acc + usize::from(diag_ok && anti_diag_ok)
        })
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
