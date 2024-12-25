use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
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

fn is_lock(grid: &Matrix<char>) -> bool {
    grid.get((0, 0)).is_some_and(|r| *r == '#')
        && grid.iter().next().expect("non-empty").iter().all_equal()
}

fn is_key(grid: &Matrix<char>) -> bool {
    grid.get((0, 0)).is_some_and(|r| *r == '.')
        && grid.iter().next().expect("non-empty").iter().all_equal()
}

fn heights(grid: &Matrix<char>, is_lock: bool) -> Vec<usize> {
    let mut result = vec![];

    let chr = if is_lock { '#' } else { '.' };
    for col in grid.column_iter() {
        let cnt = col.iter().take_while(|&&it| *it == chr).count();
        let n = if is_lock {
            cnt - 1
        } else {
            grid.rows - cnt - 1
        };
        result.push(n);
    }

    result
}

#[aoc(day25, part1)]
pub fn part1(conns: &[Matrix<char>]) -> usize {
    let mut result = 0;

    for m1 in conns.iter().filter(|it| is_lock(it)) {
        let height = m1.rows - 1;
        let heights_m1 = heights(m1, true);

        for m2 in conns.iter().filter(|it| is_key(it)) {
            let heights_m2 = heights(m2, false);

            let fits = heights_m1
                .iter()
                .zip(heights_m2.iter())
                .map(|(lhs, rhs)| lhs + rhs)
                .all(|it| it < height);

            if fits {
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
