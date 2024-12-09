use aoc_runner_derive::aoc;
use itertools::Itertools;

fn checksum(s: &[Option<usize>]) -> usize {
    s.iter()
        .enumerate()
        .filter_map(|(idx, num)| num.map(|num| (idx, num)))
        .fold(0, |acc, (idx, num)| acc + idx * num)
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

    let counts_map = res.iter().filter_map(|it| *it).counts();

    for id_to_move in (0..=highest_id).rev() {
        let space_needed = counts_map[&id_to_move];

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

fn expand_format(s: &str) -> Vec<Option<usize>> {
    let mut res = vec![];

    for (id, mut chnk) in s.chars().chunks(2).into_iter().enumerate() {
        if let Some(file) = chnk.next().and_then(|it| it.to_digit(10)) {
            res.extend_from_slice(&vec![Some(id); file as usize]);
        }

        if let Some(free) = chnk.next().and_then(|it| it.to_digit(10)) {
            res.extend_from_slice(&vec![None; free as usize]);
        }
    }

    res
}

#[aoc(day09, part1)]
pub fn part1(inp: &str) -> usize {
    let vec = expand_format(inp);
    let res = reorder_blocks(&vec);

    checksum(&res)
}

#[aoc(day09, part2)]
pub fn part2(inp: &str) -> usize {
    let vec = expand_format(inp);
    let res = reorder_whole_file(&vec);

    checksum(&res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2333133121414131402";

    #[test]
    fn test_p1() {
        let res = part1(TEST_INPUT);
        assert_eq!(res, 1928);
    }

    #[test]
    fn test_p2() {
        let res = part2(TEST_INPUT);
        assert_eq!(res, 2858);
    }
}
