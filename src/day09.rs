use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::collections::HashMap;

fn checksum(s: &[Option<usize>]) -> usize {
    s.iter()
        .enumerate()
        .filter_map(|(idx, num)| num.map(|num| (idx, num)))
        .fold(0, |acc, (idx, num)| acc + idx * num)
}

fn reorder_blocks(v: &[Option<usize>]) -> Vec<Option<usize>> {
    let mut res = v.to_vec();

    let mut last_non_empty = res.iter().rposition(Option::is_some).expect("non empty");
    let mut first_empty = res.iter().position(Option::is_none).expect("has empty");

    loop {
        if first_empty > last_non_empty {
            break;
        }

        res.swap(first_empty, last_non_empty);

        first_empty += 1;
        last_non_empty -= 1;

        while res[first_empty].is_some() {
            first_empty += 1;
        }

        while res[last_non_empty].is_none() {
            last_non_empty -= 1;
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
    let mut files = HashMap::new();
    let mut empty_space = Vec::new();

    let mut idx = 0;
    for (id, mut chnk) in inp.chars().chunks(2).into_iter().enumerate() {
        if let Some(file) = chnk.next().and_then(|it| it.to_digit(10)) {
            files.insert(id, (idx, file));
            idx += file;
        }

        if let Some(free) = chnk.next().and_then(|it| it.to_digit(10)) {
            empty_space.push((idx, free));
            idx += free;
        }
    }

    for id in (1..files.len()).rev() {
        let (pos, len) = files[&id];
        if let Some(empty_idx) = empty_space.iter().position(|(_, space)| *space >= len) {
            let (start, space) = &mut empty_space[empty_idx];
            if *start >= pos {
                empty_space = empty_space[..empty_idx].to_vec();
                continue;
            }

            files.entry(id).and_modify(|(idx, _)| *idx = *start);

            *start += len;
            *space -= len;
            if *space == 0 {
                empty_space.remove(empty_idx);
            }
        }
    }

    files.iter().fold(0, |acc, (k, &(idx, len))| {
        (idx..idx + len).fold(acc, |acc, n| acc + k * n as usize)
    })
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
