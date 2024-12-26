use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{iproduct, Itertools};
use pathfinding::matrix::Matrix;

#[aoc_generator(day25)]
pub fn generate(s: &str) -> Option<Vec<Matrix<char>>> {
    let mut result = vec![];

    for g in s.split("\n\n") {
        let v = g.lines().map(|l| l.chars().collect_vec()).collect_vec();
        result.push(Matrix::from_rows(v).ok()?);
    }

    Some(result)
}

fn has_overlap(lock: &Matrix<char>, key: &Matrix<char>) -> bool {
    assert_eq!(lock.rows, key.rows);
    assert_eq!(lock.columns, key.columns);

    for (r, c) in iproduct!(0..lock.rows, 0..lock.columns) {
        if lock[(r, c)] == '#' && lock[(r, c)] == key[(r, c)] {
            return true;
        }
    }

    false
}

#[aoc(day25, part1)]
pub fn part1(conns: &[Matrix<char>]) -> usize {
    let mut result = 0;

    let (locks, keys): (Vec<Matrix<char>>, Vec<Matrix<char>>) =
        conns.iter().cloned().partition(|it| it[(0, 0)] == '#');

    for lock in &locks {
        for key in &keys {
            if !has_overlap(lock, key) {
                result += 1;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_P1: &str = "#####\n\
                                 .####\n\
                                 .####\n\
                                 .####\n\
                                 .#.#.\n\
                                 .#...\n\
                                 .....\n\
                                 \n\
                                 #####\n\
                                 ##.##\n\
                                 .#.##\n\
                                 ...##\n\
                                 ...#.\n\
                                 ...#.\n\
                                 .....\n\
                                 \n\
                                 .....\n\
                                 #....\n\
                                 #....\n\
                                 #...#\n\
                                 #.#.#\n\
                                 #.###\n\
                                 #####\n\
                                 \n\
                                 .....\n\
                                 .....\n\
                                 #.#..\n\
                                 ###..\n\
                                 ###.#\n\
                                 ###.#\n\
                                 #####\n\
                                 \n\
                                 .....\n\
                                 .....\n\
                                 .....\n\
                                 #....\n\
                                 #.#..\n\
                                 #.#.#\n\
                                 #####";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT_P1).expect("valid input");
        let res = part1(&gen);
        assert_eq!(res, 3);
    }
}
