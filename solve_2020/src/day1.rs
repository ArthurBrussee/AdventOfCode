use aoc_lib::AocSolution;
use std::collections::HashSet;

pub struct Solution;

impl AocSolution<i32, i32> for Solution {
    const DATE: (u32, u32) = (2020, 1);

    fn calc(input: &str) -> (i32, i32) {
        let map: HashSet<i32> = input.lines().map(|it| it.parse().unwrap()).collect();

        fn part1(map: &HashSet<i32>) -> i32 {
            for num in map {
                let remainder = 2020 - num;
                if map.get(&remainder).is_some() {
                    return num * remainder;
                }
            }
            panic!("Puzzle has no solution!");
        }

        fn part2(map: &HashSet<i32>) -> i32 {
            for num1 in map {
                for num2 in map {
                    let remainder = 2020 - num1 - num2;
                    if map.get(&remainder).is_some() {
                        return num1 * num2 * remainder;
                    }
                }
            }
            panic!("Puzzle has no solution!");
        }

        (part1(&map), part2(&map))
    }
}
