use aoc_lib::{AocSolution, DoubleLineSplit};

pub struct Solution;

impl AocSolution<usize, usize> for Solution {
    const DATE: (u32, u32) = (2022, 1);

    fn calc(input: &str) -> (usize, usize) {
        let mut cals = input
            .split_at_empty_line()
            .map(|food_list| food_list.lines().map(|x| x.parse::<usize>().unwrap()).sum())
            .collect::<Vec<usize>>();

        cals.sort_by(|a, b| b.cmp(a));

        (cals[0], cals.iter().take(3).sum())
    }
}

#[test]
fn test() {
    Solution::test(24000, 45000);
}
