use aoc_lib::{AocSolution, DoubleLineSplit};
use itertools::Itertools;

pub struct Solution;

impl AocSolution<String, String> for Solution {
    const DATE: (u32, u32) = (2022, 5);

    fn calc(input: &str) -> (String, String) {
        let (part1, part2) = input.split_at_empty_line().collect_tuple().unwrap();

        let mut stacks_9000: Vec<Vec<_>> = part1
            .lines()
            .map(|line| line.split(' ').map(|x| x.chars().next().unwrap()).collect())
            .collect();

        let mut stacks_9001 = stacks_9000.clone();

        for line in part2.lines() {
            let (num, from, to) = line
                .split(' ')
                .map(|x| x.parse().unwrap())
                .collect_tuple()
                .unwrap();

            let items: Vec<_> = (0..num)
                .map(|_| stacks_9000[from - 1].pop().unwrap())
                .collect();
            stacks_9000[to - 1].extend(items);

            let items: Vec<_> = (0..num)
                .map(|_| stacks_9001[from - 1].pop().unwrap())
                .collect();
            stacks_9001[to - 1].extend(items.iter().rev());
        }

        fn collect_last_crates(stacks: Vec<Vec<char>>) -> String {
            stacks.iter().map(|x| x.last().unwrap()).collect()
        }

        (
            collect_last_crates(stacks_9000),
            collect_last_crates(stacks_9001),
        )
    }
}

#[test]
fn test() {
    Solution::test("CMZ".to_string(), "MCD".to_string());
}
