use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use pathfinding::prelude::Matrix;

#[derive(Clone, Debug)]
pub struct Input {
    grid: Matrix<char>,
    insts: Vec<char>,
}

#[aoc_generator(day15, part1)]
pub fn generate_p1(s: &str) -> Option<Input> {
    let (map, movements) = s.split_once("\n\n")?;
    let v = map.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let grid = Matrix::from_rows(v).ok()?;

    let insts = movements.lines().join("").chars().collect_vec();

    Some(Input { grid, insts })
}

#[aoc_generator(day15, part2)]
pub fn generate_p2(s: &str) -> Option<Input> {
    let (map, movements) = s.split_once("\n\n")?;
    let v = map
        .lines()
        .map(|l| {
            l.chars()
                .flat_map(|c| match c {
                    'O' => ['[', ']'],
                    '@' => ['@', '.'],
                    '#' => ['#', '#'],
                    '.' => ['.', '.'],
                    _ => panic!("unexpected char: {c}"),
                })
                .collect_vec()
        })
        .collect_vec();
    let grid = Matrix::from_rows(v).ok()?;

    let insts = movements.lines().join("").chars().collect_vec();

    Some(Input { grid, insts })
}

fn find_start_pos(grid: &Matrix<char>) -> Option<(usize, usize)> {
    grid.keys().find(|&(r, c)| grid[(r, c)] == '@')
}

#[allow(clippy::cast_possible_wrap)]
fn move_towards((dr, dc): (isize, isize), (rr, rc): &mut (usize, usize), grid: &mut Matrix<char>) {
    let (nr, nc) = (*rr as isize + dr, *rc as isize + dc);
    assert!(nr >= 0 && nc >= 0);
    let (nr, nc) = (nr as usize, nc as usize);

    // simple case: wall
    if grid[(nr, nc)] == '#' {
        return;
    }

    // simple case: empty space
    if grid[(nr, nc)] == '.' {
        *rr = nr;
        *rc = nc;
        return;
    }

    // simple case: all taken
    if grid
        .in_direction((*rr, *rc), (dr, dc))
        .all(|p| grid[p] != '.')
    {
        return;
    }

    assert_eq!(grid[(nr, nc)], 'O');

    // move boxes first
    let boxes = grid
        .in_direction((*rr, *rc), (dr, dc))
        .take_while(|&p| grid[p] == 'O')
        .collect_vec();

    for &(br, bc) in boxes.iter().rev() {
        let after_box = (br as isize + dr, bc as isize + dc);
        let after_box = (after_box.0 as usize, after_box.1 as usize);

        if grid[after_box] == '#' {
            return;
        }

        assert_eq!(grid[after_box], '.');

        grid.swap(after_box, (br, bc));
    }

    *rr = nr;
    *rc = nc;
}

#[allow(clippy::cast_possible_wrap)]
fn move_towards_p2(
    (dr, dc): (isize, isize),
    (rr, rc): &mut (usize, usize),
    grid: &mut Matrix<char>,
) {
    let (nr, nc) = (*rr as isize + dr, *rc as isize + dc);
    assert!(nr >= 0 && nc >= 0);
    let (nr, nc) = (nr as usize, nc as usize);

    // simple case: wall
    if grid[(nr, nc)] == '#' {
        return;
    }

    // simple case: empty space
    if grid[(nr, nc)] == '.' {
        *rr = nr;
        *rc = nc;
        return;
    }

    // simple case: all taken
    if grid
        .in_direction((*rr, *rc), (dr, dc))
        .all(|p| grid[p] != '.')
    {
        return;
    }

    if dr != 0 {
        assert_eq!(dc, 0);

        // moving up we need to check the neighbouring column as well
        let is_left_edge = grid[(nr, nc)] == '[';
        assert!(is_left_edge || grid[(nr, nc)] == ']');

        let left_block_edge = if is_left_edge { (nr, nc) } else { (nr, nc - 1) };

        if !can_move_vertically(left_block_edge, dr, grid) {
            return;
        }

        do_move_vertically(left_block_edge, dr, grid);
    } else {
        // move boxes first
        let boxes = grid
            .in_direction((*rr, *rc), (dr, dc))
            .take_while(|&p| grid[p] == '[' || grid[p] == ']')
            .collect_vec();

        for &(br, bc) in boxes.iter().rev() {
            let after_box = (br as isize + dr, bc as isize + dc);
            let after_box = (after_box.0 as usize, after_box.1 as usize);

            if grid[after_box] == '#' {
                return;
            }

            assert_eq!(grid[after_box], '.');

            grid.swap(after_box, (br, bc));
        }
    }

    assert_eq!(grid[(nr, nc)], '.');

    *rr = nr;
    *rc = nc;
}

#[allow(clippy::cast_possible_wrap)]
fn do_move_vertically((br, bc): (usize, usize), row_dir: isize, grid: &mut Matrix<char>) {
    assert_ne!(row_dir, 0);
    assert_eq!(grid[(br, bc)], '[');
    assert_eq!(grid[(br, bc + 1)], ']');

    let next_br = (br as isize + row_dir) as usize;

    // []
    // []
    if grid[(next_br, bc)] == '[' && grid[(next_br, bc + 1)] == ']' {
        do_move_vertically((next_br, bc), row_dir, grid);
    }

    // []
    // .[
    if grid[(next_br, bc)] == ']' {
        do_move_vertically((next_br, bc - 1), row_dir, grid);
    }

    // .[
    // []
    if grid[(next_br, bc + 1)] == '[' {
        do_move_vertically((next_br, bc + 1), row_dir, grid);
    }

    // ..
    // []
    if grid[(next_br, bc)] == '.' && grid[(next_br, bc + 1)] == '.' {
        grid.swap((br, bc), (next_br, bc));
        grid.swap((br, bc + 1), (next_br, bc + 1));
    }
}

#[allow(clippy::cast_possible_wrap)]
fn can_move_vertically((br, bc): (usize, usize), row_dir: isize, grid: &Matrix<char>) -> bool {
    assert_ne!(row_dir, 0);
    assert_eq!(grid[(br, bc)], '[');
    assert_eq!(grid[(br, bc + 1)], ']');

    let next_br = (br as isize + row_dir) as usize;
    if grid[(next_br, bc)] == '.' && grid[(next_br, bc + 1)] == '.' {
        return true;
    }

    if grid[(next_br, bc)] == '#' || grid[(next_br, bc + 1)] == '#' {
        return false;
    }

    let above_bc_left = grid[(next_br, bc)];
    let above_bc_right = grid[(next_br, bc + 1)];

    if above_bc_left == '[' && above_bc_right == ']' {
        return can_move_vertically((next_br, bc), row_dir, grid);
    }

    if above_bc_left == ']' && !can_move_vertically((next_br, bc - 1), row_dir, grid) {
        return false;
    }

    if above_bc_right == '[' && !can_move_vertically((next_br, bc + 1), row_dir, grid) {
        return false;
    }

    true
}

#[aoc(day15, part1)]
pub fn part1(inp: &Input) -> Option<usize> {
    let mut grid = inp.grid.clone();

    let mut robot_pos = find_start_pos(&grid)?;
    grid[robot_pos] = '.';

    for inst in &inp.insts {
        match *inst {
            '>' => move_towards((0, 1), &mut robot_pos, &mut grid),
            '<' => move_towards((0, -1), &mut robot_pos, &mut grid),
            '^' => move_towards((-1, 0), &mut robot_pos, &mut grid),
            'v' => move_towards((1, 0), &mut robot_pos, &mut grid),
            _ => panic!("invalid instruction"),
        };
    }

    let sum_of_boxes = grid
        .keys()
        .filter(|&(r, c)| grid[(r, c)] == 'O')
        .map(|(r, c)| r * 100 + c)
        .sum::<usize>();

    Some(sum_of_boxes)
}

#[aoc(day15, part2)]
pub fn part2(inp: &Input) -> Option<usize> {
    let mut grid = inp.grid.clone();

    let mut robot_pos = find_start_pos(&grid)?;
    grid[robot_pos] = '.';

    for inst in &inp.insts {
        match *inst {
            '>' => move_towards_p2((0, 1), &mut robot_pos, &mut grid),
            '<' => move_towards_p2((0, -1), &mut robot_pos, &mut grid),
            '^' => move_towards_p2((-1, 0), &mut robot_pos, &mut grid),
            'v' => move_towards_p2((1, 0), &mut robot_pos, &mut grid),
            _ => panic!("invalid instruction"),
        };
    }

    let sum_of_boxes = grid
        .keys()
        .filter(|&pos| grid[pos] == '[')
        .map(|(r, c)| r * 100 + c)
        .sum::<usize>();

    Some(sum_of_boxes)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "########\n\
                              #..O.O.#\n\
                              ##@.O..#\n\
                              #...O..#\n\
                              #.#.O..#\n\
                              #...O..#\n\
                              #......#\n\
                              ########\n\
                              \n\
                              <^^>>>vv<v>>v<<";

    const TEST_INPUT_P2: &str = "##########\n\
                                 #..O..O.O#\n\
                                 #......O.#\n\
                                 #.OO..O.O#\n\
                                 #..O@..O.#\n\
                                 #O#..O...#\n\
                                 #O..O..O.#\n\
                                 #.OO.O.OO#\n\
                                 #....O...#\n\
                                 ##########\n\
                                 \n\
                                 <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^\n\
                                 vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v\n\
                                 ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<\n\
                                 <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^\n\
                                 ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><\n\
                                 ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^\n\
                                 >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^\n\
                                 <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>\n\
                                 ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>\n\
                                 v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn test_p1() {
        let gen = generate_p1(TEST_INPUT).expect("valid input");
        let res = part1(&gen);
        assert_eq!(res, Some(2028));
    }

    #[test]
    fn test_p2() {
        let gen = generate_p2(TEST_INPUT_P2).expect("valid input");
        let res = part2(&gen);
        assert_eq!(res, Some(9021));
    }

    #[test]
    fn test_can_move() {
        let txt = "..@...\n\
                        ..[]..\n\
                        .[][].\n\
                        [][][]\n\
                        ......";

        let v = txt.lines().map(|l| l.chars().collect_vec()).collect_vec();
        let grid = Matrix::from_rows(v).expect("valid text");

        assert!(can_move_vertically((1, 2), 1, &grid));
    }

    #[test]
    fn test_can_move_2() {
        let txt = "......\n\
                        .[]...\n\
                        ..[]..";

        let v = txt.lines().map(|l| l.chars().collect_vec()).collect_vec();
        let grid = Matrix::from_rows(v).expect("valid text");

        assert!(can_move_vertically((2, 2), -1, &grid));
    }

    #[test]
    fn test_can_move_3() {
        let txt = ".......\n\
                        ...[]..\n\
                        ..[][].\n\
                        .[]#[].\n\
                        .[].[].\n\
                        ..[][].\n\
                        ...[]..\n\
                        .......";

        let v = txt.lines().map(|l| l.chars().collect_vec()).collect_vec();
        let grid = Matrix::from_rows(v).expect("valid text");

        assert!(can_move_vertically((6, 3), -1, &grid));
    }

    #[test]
    fn test_cant_move() {
        let txt = "......\n\
                        ..[]..\n\
                        .[][].\n\
                        [][][]\n\
                        ....#.\n\
                        ......\n\
                        ......";

        let v = txt.lines().map(|l| l.chars().collect_vec()).collect_vec();
        let grid = Matrix::from_rows(v).expect("valid text");

        assert!(!can_move_vertically((1, 2), 1, &grid));
    }

    #[test]
    fn test_cant_move_2() {
        let txt = "##################\n\
                        ##..............##\n\
                        ##..[]..........##\n\
                        ##............####\n\
                        ##....[]##......##\n\
                        ##..[][][]..[][]##\n\
                        ##.....[]...[][]##\n\
                        ##....[]......[]##\n\
                        ##..[][]......####\n\
                        ##..........[][]##\n\
                        ##################";

        let v = txt.lines().map(|l| l.chars().collect_vec()).collect_vec();
        let grid = Matrix::from_rows(v).expect("valid text");

        assert!(!can_move_vertically((8, 6), -1, &grid));
    }
}
