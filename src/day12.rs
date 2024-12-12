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
    pathfinding::prelude::strongly_connected_components(&nodes, |&(r, c)| {
        succs(grid, (r, c))
            .iter()
            .map(|(r, c)| (*r, *c))
            .collect_vec()
    })
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
        // outside corners

        // above is empty and left is empty
        let top_left_corner = !comp.contains(&(r - 1, c)) && !comp.contains(&(r, c - 1));

        // above is empty and right is empty
        let top_right_corner = !comp.contains(&(r - 1, c)) && !comp.contains(&(r, c + 1));

        // left is empty and below is empty
        let bot_left_corner = !comp.contains(&(r, c - 1)) && !comp.contains(&(r + 1, c));

        // right is empty and below is empty
        let bot_right_corner = !comp.contains(&(r, c + 1)) && !comp.contains(&(r + 1, c));

        result += usize::from(top_left_corner)
            + usize::from(top_right_corner)
            + usize::from(bot_left_corner)
            + usize::from(bot_right_corner);

        // inside corners

        // below contained, right contained, bot-right diag not contained
        let top_left_inside = comp.contains(&(r + 1, c))
            && comp.contains(&(r, c + 1))
            && !comp.contains(&(r + 1, c + 1));

        // below contained, left contained, bot-left diag not contained
        let top_right_inside = comp.contains(&(r + 1, c))
            && comp.contains(&(r, c - 1))
            && !comp.contains(&(r + 1, c - 1));

        // above contained, right contained, top-right diag not contained
        let bot_left_inside = comp.contains(&(r - 1, c))
            && comp.contains(&(r, c + 1))
            && !comp.contains(&(r - 1, c + 1));

        // above contained, left contained, top-left diag not contained
        let bot_right_inside = comp.contains(&(r - 1, c))
            && comp.contains(&(r, c - 1))
            && !comp.contains(&(r - 1, c - 1));

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
