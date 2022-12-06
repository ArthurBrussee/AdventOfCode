use aoc_lib::AocSolution;
use itertools::Itertools;

pub struct Solution;

impl AocSolution<usize, usize> for Solution {
    const DATE: (u32, u32) = (2022, 6);

    fn calc(input: &str) -> (usize, usize) {
        let chars: Vec<_> = input.chars().collect();
        let find_unique = |n: usize| {
            let (first, _) = chars
                .windows(n)
                .find_position(|window| window.iter().unique().count() == n)
                .unwrap();
            first + n
        };
        (find_unique(4), find_unique(14))
    }
}

#[test]
fn test() {
    Solution::test(7, 19);
}
