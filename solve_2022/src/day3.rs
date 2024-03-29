use std::collections::HashSet;

use aoc_lib::AocSolution;
use itertools::Itertools;

fn char_to_priority(c: char) -> u8 {
    if c.is_lowercase() {
        c as u8 - b'a' + 1
    } else {
        c as u8 - b'A' + 27
    }
}

pub struct Solution;

impl AocSolution<usize, usize> for Solution {
    const DATE: (u32, u32) = (2022, 3);

    fn calc(input: &str) -> (usize, usize) {
        let lines: Vec<_> = input.lines().collect();

        let rucksack_overlap = lines
            .iter()
            .map(|line| {
                let (l, r) = line.split_at(line.len() / 2);
                let hash_left: HashSet<_> = l.chars().collect();
                let hash_right: HashSet<_> = r.chars().collect();
                let common = hash_left.intersection(&hash_right).next().unwrap();
                char_to_priority(*common) as usize
            })
            .sum();

        let elves_overlap = lines
            .chunks(3)
            .map(|elves| {
                let (h1, h2, h3) = elves
                    .iter()
                    .map(|s| s.chars().collect::<HashSet<_>>())
                    .collect_tuple()
                    .unwrap();

                let hash1: HashSet<_> = h1.intersection(&h2).copied().collect();
                let common = *hash1.intersection(&h3).next().unwrap();
                char_to_priority(common) as usize
            })
            .sum();

        (rucksack_overlap, elves_overlap)
    }
}

#[test]
fn test() {
    Solution::test(157, 70);
}
