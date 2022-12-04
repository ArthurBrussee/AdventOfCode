use aoc_lib::AocSolution;
use itertools::Itertools;
use std::ops::RangeInclusive;

pub struct Solution;

impl AocSolution<usize, usize> for Solution {
    const DATE: (u32, u32) = (2022, 4);

    fn calc(input: &str) -> (usize, usize) {
        input
            .lines()
            .map(|line| {
                fn to_range(l: &str) -> RangeInclusive<usize> {
                    let (min, max) = l.split_once('-').unwrap();
                    min.parse().unwrap()..=max.parse().unwrap()
                }

                let (r1, r2) = line.split(',').map(to_range).collect_tuple().unwrap();

                let overlap_full = r1.contains(r2.start()) && r1.contains(r2.end())
                    || r2.contains(r1.start()) && r2.contains(r1.end());
                let overlap_part = overlap_full || r1.contains(r2.start()) || r1.contains(r2.end());
                (usize::from(overlap_full), usize::from(overlap_part))
            })
            .fold((0, 0), |(acc1, acc2), (p1, p2)| (acc1 + p1, acc2 + p2))
    }
}

#[test]
fn test() {
    Solution::test(2, 4);
}
