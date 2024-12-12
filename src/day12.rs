use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use pathfinding::prelude::Matrix;

#[aoc_generator(day12)]
pub fn generate(s: &str) -> Option<Matrix<char>> {
    let v = s.lines().map(|l| l.chars().collect_vec()).collect_vec();
    Matrix::from_rows(v).ok()
}

fn succs(grid: &Matrix<char>, (r, c): (usize, usize)) -> Vec<(usize, usize)> {
    let mut succs = vec![];

    for (dr, dc) in [(0, 1), (-1, 0), (0, -1), (1, 0)] {
        if let (Some(new_r), Some(new_c)) = (r.checked_add_signed(dr), c.checked_add_signed(dc)) {
            if grid.get((new_r, new_c)) == grid.get((r, c)) {
                succs.push((new_r, new_c));
            }
        }
    }

    succs
}

fn get_components(grid: &Matrix<char>) -> Vec<Vec<(usize, usize)>> {
    let nodes = grid.keys().collect_vec();
    pathfinding::prelude::strongly_connected_components(&nodes, |&(r, c)| succs(grid, (r, c)))
}

fn perimeter(grid: &Matrix<char>, comp: &[(usize, usize)]) -> usize {
    comp.iter()
        .fold(0, |acc, &(r, c)| acc + 4 - succs(grid, (r, c)).len())
}

#[aoc(day12, part1)]
pub fn part1(inp: &Matrix<char>) -> usize {
    let comps = get_components(inp);
    let mut result = 0;

    for comp in comps {
        result += comp.len() * perimeter(inp, &comp);
    }

    result
}

#[allow(clippy::cast_possible_wrap)]
fn count_corners(comp: &[(usize, usize)]) -> usize {
    let mut result = 0;

    let comp = comp
        .iter()
        .map(|&(r, c)| (r as isize, c as isize))
        .collect_vec();

    for &(r, c) in &comp {
        let above = (r - 1, c);
        let below = (r + 1, c);
        let left = (r, c - 1);
        let right = (r, c + 1);
        let top_right = (r - 1, c + 1);
        let top_left = (r - 1, c - 1);
        let bot_right = (r + 1, c + 1);
        let bot_left = (r + 1, c - 1);

        // outside corners

        // above is empty and left is empty
        let top_left_corner = !comp.contains(&above) && !comp.contains(&left);

        // above is empty and right is empty
        let top_right_corner = !comp.contains(&above) && !comp.contains(&right);

        // left is empty and below is empty
        let bot_left_corner = !comp.contains(&left) && !comp.contains(&below);

        // right is empty and below is empty
        let bot_right_corner = !comp.contains(&right) && !comp.contains(&below);

        result += usize::from(top_left_corner)
            + usize::from(top_right_corner)
            + usize::from(bot_left_corner)
            + usize::from(bot_right_corner);

        // inside corners

        // below contained, right contained, bot-right diag not contained
        let top_left_inside =
            comp.contains(&below) && comp.contains(&right) && !comp.contains(&bot_right);

        // below contained, left contained, bot-left diag not contained
        let top_right_inside =
            comp.contains(&below) && comp.contains(&left) && !comp.contains(&bot_left);

        // above contained, right contained, top-right diag not contained
        let bot_left_inside =
            comp.contains(&above) && comp.contains(&right) && !comp.contains(&top_right);

        // above contained, left contained, top-left diag not contained
        let bot_right_inside =
            comp.contains(&above) && comp.contains(&left) && !comp.contains(&top_left);

        result += usize::from(top_left_inside)
            + usize::from(top_right_inside)
            + usize::from(bot_left_inside)
            + usize::from(bot_right_inside);
    }

    result
}

#[aoc(day12, part2)]
pub fn part2(inp: &Matrix<char>) -> usize {
    let comps = get_components(inp);
    let mut result = 0;

    for comp in comps {
        let num_corners = count_corners(&comp);
        result += comp.len() * num_corners;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "RRRRIICCFF\n\
                              RRRRIICCCF\n\
                              VVRRRCCFFF\n\
                              VVRCCCJFFF\n\
                              VVVVCJJCFE\n\
                              VVIVCCJJEE\n\
                              VVIIICJJEE\n\
                              MIIIIIJJEE\n\
                              MIIISIJEEE\n\
                              MMMISSJEEE";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT).expect("valid input");
        let res = part1(&gen);
        assert_eq!(res, 1930);
    }

    #[test]
    fn test_p2() {
        let gen = generate(TEST_INPUT).expect("valid input");
        let res = part2(&gen);
        assert_eq!(res, 1206);
    }
}
