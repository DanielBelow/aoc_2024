use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use pathfinding::prelude::Matrix;

#[aoc_generator(day16)]
pub fn generate(s: &str) -> Option<Matrix<char>> {
    let v = s.lines().map(|l| l.chars().collect_vec()).collect_vec();
    Matrix::from_rows(v).ok()
}

fn find_node(pos: char, grid: &Matrix<char>) -> Option<(usize, usize)> {
    grid.keys().find(|&(r, c)| grid[(r, c)] == pos)
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct State {
    pos: (usize, usize),
    dir: Direction,
}

impl State {
    const fn with_dir(self, dir: Direction) -> Self {
        Self { dir, ..self }
    }

    fn move_forward(self, grid: &Matrix<char>) -> Option<Self> {
        let new_pos = match self.dir {
            Direction::North => {
                let new_r = self.pos.0.checked_add_signed(-1)?;
                (new_r, self.pos.1)
            }
            Direction::East => (self.pos.0, self.pos.1 + 1),
            Direction::South => (self.pos.0 + 1, self.pos.1),
            Direction::West => {
                let new_c = self.pos.1.checked_add_signed(-1)?;
                (self.pos.0, new_c)
            }
        };

        if grid.get(new_pos).is_some_and(|pos| *pos != '#') {
            Some(Self {
                pos: new_pos,
                ..self
            })
        } else {
            None
        }
    }
}

fn add_turns(state: State, states: &mut Vec<(State, usize)>) {
    const TURN_COST: usize = 1000;

    if state.dir == Direction::North || state.dir == Direction::South {
        states.push((state.with_dir(Direction::East), TURN_COST));
        states.push((state.with_dir(Direction::West), TURN_COST));
    } else if state.dir == Direction::East || state.dir == Direction::West {
        states.push((state.with_dir(Direction::North), TURN_COST));
        states.push((state.with_dir(Direction::South), TURN_COST));
    }
}

fn successors(state: State, grid: &Matrix<char>) -> Vec<(State, usize)> {
    const MOVE_COST: usize = 1;

    let mut succs = vec![];

    if let Some(fwd) = state.move_forward(grid) {
        succs.push((fwd, MOVE_COST));
    }
    add_turns(state, &mut succs);

    succs
}

#[aoc(day16, part1)]
#[allow(clippy::too_many_lines)]
pub fn part1(grid: &Matrix<char>) -> Option<usize> {
    let start_pos = find_node('S', grid)?;
    let goal_pos = find_node('E', grid)?;

    let state = State {
        pos: start_pos,
        dir: Direction::East,
    };

    let (_, cost) =
        pathfinding::prelude::dijkstra(&state, |s| successors(*s, grid), |s| s.pos == goal_pos)?;
    Some(cost)
}

#[aoc(day16, part2)]
pub fn part2(grid: &Matrix<char>) -> Option<usize> {
    let start_pos = find_node('S', grid)?;
    let goal_pos = find_node('E', grid)?;
    let start_dir = Direction::East;

    let state = State {
        pos: start_pos,
        dir: start_dir,
    };

    let (paths, _) = pathfinding::prelude::astar_bag_collect(
        &state,
        |s| successors(*s, grid),
        |_| 1, // dummy heuristic
        |s| s.pos == goal_pos,
    )?;

    let num_unique_positions = paths
        .iter()
        .flat_map(|p| p.iter().map(|s| s.pos))
        .unique()
        .count();
    Some(num_unique_positions)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "###############\n\
                              #.......#....E#\n\
                              #.#.###.#.###.#\n\
                              #.....#.#...#.#\n\
                              #.###.#####.#.#\n\
                              #.#.#.......#.#\n\
                              #.#.#####.###.#\n\
                              #...........#.#\n\
                              ###.#.#####.#.#\n\
                              #...#.....#.#.#\n\
                              #.#.#.###.#.#.#\n\
                              #.....#...#.#.#\n\
                              #.###.#.#.#.#.#\n\
                              #S..#.....#...#\n\
                              ###############";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT).expect("valid input");
        let res = part1(&gen);
        assert_eq!(res, Some(7036));
    }

    #[test]
    fn test_p2() {
        let gen = generate(TEST_INPUT).expect("valid input");
        let res = part2(&gen);
        assert_eq!(res, Some(45));
    }
}
