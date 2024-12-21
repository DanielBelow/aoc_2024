use aoc_runner_derive::{aoc, aoc_generator};
use memoize::memoize;

#[aoc_generator(day21)]
pub fn generate(s: &str) -> Vec<String> {
    s.lines().map(ToString::to_string).collect()
}

fn append_dir(dir: char, dirs: &str) -> String {
    dirs.to_string() + &dir.to_string()
}

#[allow(clippy::too_many_lines, clippy::needless_pass_by_value)]
#[memoize]
fn directional_keypad_successors(state: KeypadState) -> Vec<(KeypadState, usize)> {
    let succs = match state.current {
        'A' => vec![
            KeypadState {
                current: '>',
                dir: append_dir('v', &state.dir),
            },
            KeypadState {
                current: '^',
                dir: append_dir('<', &state.dir),
            },
        ],
        '^' => vec![
            KeypadState {
                current: 'A',
                dir: append_dir('>', &state.dir),
            },
            KeypadState {
                current: 'v',
                dir: append_dir('v', &state.dir),
            },
        ],
        '<' => vec![KeypadState {
            current: 'v',
            dir: append_dir('>', &state.dir),
        }],
        'v' => vec![
            KeypadState {
                current: '>',
                dir: append_dir('>', &state.dir),
            },
            KeypadState {
                current: '<',
                dir: append_dir('<', &state.dir),
            },
            KeypadState {
                current: '^',
                dir: append_dir('^', &state.dir),
            },
        ],
        '>' => vec![
            KeypadState {
                current: 'v',
                dir: append_dir('<', &state.dir),
            },
            KeypadState {
                current: 'A',
                dir: append_dir('^', &state.dir),
            },
        ],
        _ => unreachable!("invalid directional keypad state"),
    };

    succs.iter().map(|it| (it.clone(), 1)).collect()
}

#[allow(clippy::too_many_lines, clippy::needless_pass_by_value)]
#[memoize]
fn keypad_successors(state: KeypadState) -> Vec<(KeypadState, usize)> {
    let succs = match state.current {
        'A' => vec![
            KeypadState {
                current: '0',
                dir: append_dir('<', &state.dir),
            },
            KeypadState {
                current: '3',
                dir: append_dir('^', &state.dir),
            },
        ],
        '0' => vec![
            KeypadState {
                current: '2',
                dir: append_dir('^', &state.dir),
            },
            KeypadState {
                current: 'A',
                dir: append_dir('>', &state.dir),
            },
        ],
        '1' => vec![
            KeypadState {
                current: '4',
                dir: append_dir('^', &state.dir),
            },
            KeypadState {
                current: '2',
                dir: append_dir('>', &state.dir),
            },
        ],
        '2' => vec![
            KeypadState {
                current: '1',
                dir: append_dir('<', &state.dir),
            },
            KeypadState {
                current: '5',
                dir: append_dir('^', &state.dir),
            },
            KeypadState {
                current: '3',
                dir: append_dir('>', &state.dir),
            },
            KeypadState {
                current: '0',
                dir: append_dir('v', &state.dir),
            },
        ],
        '3' => vec![
            KeypadState {
                current: '2',
                dir: append_dir('<', &state.dir),
            },
            KeypadState {
                current: '6',
                dir: append_dir('^', &state.dir),
            },
            KeypadState {
                current: 'A',
                dir: append_dir('v', &state.dir),
            },
        ],
        '4' => vec![
            KeypadState {
                current: '7',
                dir: append_dir('^', &state.dir),
            },
            KeypadState {
                current: '5',
                dir: append_dir('>', &state.dir),
            },
            KeypadState {
                current: '1',
                dir: append_dir('v', &state.dir),
            },
        ],
        '5' => vec![
            KeypadState {
                current: '8',
                dir: append_dir('^', &state.dir),
            },
            KeypadState {
                current: '6',
                dir: append_dir('>', &state.dir),
            },
            KeypadState {
                current: '2',
                dir: append_dir('v', &state.dir),
            },
            KeypadState {
                current: '4',
                dir: append_dir('<', &state.dir),
            },
        ],
        '6' => vec![
            KeypadState {
                current: '9',
                dir: append_dir('^', &state.dir),
            },
            KeypadState {
                current: '5',
                dir: append_dir('<', &state.dir),
            },
            KeypadState {
                current: '3',
                dir: append_dir('v', &state.dir),
            },
        ],
        '7' => vec![
            KeypadState {
                current: '8',
                dir: append_dir('>', &state.dir),
            },
            KeypadState {
                current: '4',
                dir: append_dir('v', &state.dir),
            },
        ],
        '8' => vec![
            KeypadState {
                current: '9',
                dir: append_dir('>', &state.dir),
            },
            KeypadState {
                current: '5',
                dir: append_dir('v', &state.dir),
            },
            KeypadState {
                current: '7',
                dir: append_dir('<', &state.dir),
            },
        ],
        '9' => vec![
            KeypadState {
                current: '6',
                dir: append_dir('v', &state.dir),
            },
            KeypadState {
                current: '8',
                dir: append_dir('<', &state.dir),
            },
        ],

        _ => unreachable!("unexpect state"),
    };

    succs.iter().map(|it| (it.clone(), 1)).collect()
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct KeypadState {
    current: char,
    dir: String,
}

#[memoize]
fn all_keypad_paths(from: char, to: char) -> Vec<String> {
    let (path, _) = pathfinding::prelude::astar_bag_collect(
        &KeypadState {
            current: from,
            dir: String::new(),
        },
        |c| keypad_successors(c.clone()),
        |_| 0,
        |cur| cur.current == to,
    )
    .expect("path exists");

    path.iter()
        .map(|p| p.last().expect("non-empty").dir.clone() + "A")
        .collect()
}

#[memoize]
fn all_dpad_paths(from: char, to: char) -> Vec<String> {
    let (path, _) = pathfinding::prelude::astar_bag_collect(
        &KeypadState {
            current: from,
            dir: String::new(),
        },
        |c| directional_keypad_successors(c.clone()),
        |_| 0,
        |cur| cur.current == to,
    )
    .expect("path exists");

    path.iter()
        .map(|p| p.last().expect("non-empty").dir.clone() + "A")
        .collect()
}

#[memoize]
#[allow(clippy::needless_pass_by_value)]
fn find_min_length(s: String, robots: usize) -> usize {
    if robots == 0 {
        return s.len();
    }

    let mut res = 0;
    let mut current = 'A';

    for c in s.chars() {
        let paths = all_dpad_paths(current, c);

        let min_length = paths
            .iter()
            .map(|it| find_min_length(it.clone(), robots - 1))
            .min()
            .expect("exists");

        res += min_length;

        current = c;
    }

    res
}

fn complexity(s: &str, robots: usize) -> usize {
    let mut current = 'A';
    let mut res = 0;

    for c in s.chars() {
        let paths = all_keypad_paths(current, c);

        let min_sub_path = paths
            .iter()
            .map(|p| find_min_length(p.clone(), robots))
            .min()
            .expect("exists");

        res += min_sub_path;
        current = c;
    }

    let num = s
        .strip_suffix("A")
        .map(|n| n.parse::<usize>().expect("number"))
        .expect("number");

    res * num
}

#[aoc(day21, part1)]
pub fn part1(inp: &[String]) -> usize {
    let mut res = 0;

    for s in inp {
        res += complexity(s, 2);
    }

    res
}

#[aoc(day21, part2)]
pub fn part2(inp: &[String]) -> usize {
    let mut res = 0;

    for s in inp {
        res += complexity(s, 25);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "029A\n\
                              980A\n\
                              179A\n\
                              456A\n\
                              379A";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT);
        let res = part1(&gen);
        assert_eq!(res, 126_384);
    }
}
