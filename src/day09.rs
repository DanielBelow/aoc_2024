use aoc_runner_derive::{aoc, aoc_generator};

fn checksum(s: &[Option<usize>]) -> usize {
    let mut res = 0;
    for (idx, num) in s.iter().enumerate() {
        if let Some(num) = num {
            res += idx * num;
        }
    }

    res
}

fn reorder_blocks(v: &[Option<usize>]) -> Vec<Option<usize>> {
    let mut res = v.to_vec();

    loop {
        let last_non_empty = res.iter().rposition(Option::is_some).expect("non empty");
        let first_empty_idx = res.iter().position(Option::is_none).expect("has empty");

        if first_empty_idx > last_non_empty {
            break;
        }

        res.swap(first_empty_idx, last_non_empty);
    }

    res
}

fn reorder_whole_file(v: &[Option<usize>]) -> Vec<Option<usize>> {
    let mut res = v.to_vec();

    let highest_id = res
        .iter()
        .rfind(|it| it.is_some())
        .expect("non empty")
        .expect("digit");

    for id_to_move in (0..=highest_id).rev() {
        let space_needed = res.iter().filter(|it| **it == Some(id_to_move)).count();

        let first_id_idx = res
            .iter()
            .position(|it| *it == Some(id_to_move))
            .expect("found");

        let to_find = vec![None; space_needed];

        if let Some(insertion_idx) = res.windows(space_needed).position(|it| it == to_find) {
            if insertion_idx < first_id_idx {
                for i in 0..space_needed {
                    assert_eq!(res[insertion_idx + i], None);
                    res.swap(insertion_idx + i, first_id_idx + i);
                }
            }
        }
    }

    res
}

#[aoc_generator(day09)]
pub fn generate(s: &str) -> String {
    s.to_string()
}

#[aoc(day09, part1)]
pub fn part1(inp: &str) -> usize {
    let mut vec = vec![];

    let mut id = 0;
    let mut is_empty_space = false;
    for c in inp.chars() {
        let dgt = c.to_digit(10).expect("digit") as usize;

        if is_empty_space {
            for _ in 0..dgt {
                vec.push(None);
            }
        } else {
            for _ in 0..dgt {
                vec.push(Some(id));
            }
            id += 1;
        }

        is_empty_space = !is_empty_space;
    }

    let res = reorder_blocks(&vec);

    checksum(&res)
}

#[aoc(day09, part2)]
pub fn part2(inp: &str) -> usize {
    let mut vec = vec![];

    let mut id = 0;
    let mut is_empty_space = false;
    for c in inp.chars() {
        let dgt = c.to_digit(10).expect("digit") as usize;

        if is_empty_space {
            for _ in 0..dgt {
                vec.push(None);
            }
        } else {
            for _ in 0..dgt {
                vec.push(Some(id));
            }
            id += 1;
        }

        is_empty_space = !is_empty_space;
    }

    let res = reorder_whole_file(&vec);

    checksum(&res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2333133121414131402";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT);
        let res = part1(&gen);
        assert_eq!(res, 1928);
    }

    #[test]
    fn test_p2() {
        let gen = generate(TEST_INPUT);
        let res = part2(&gen);
        assert_eq!(res, 2858);
    }
}
