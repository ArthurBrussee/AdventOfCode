use crate::lib;
use std::collections::HashSet;

pub fn part1() -> i32 {
    let map: HashSet<i32> = lib::file_lines("./inputs/day1.txt")
        .map(|it| it.parse().unwrap())
        .collect();

    for num in &map {
        let remainder = 2020 - num;
        if map.get(&remainder).is_some() {
            return num * remainder;
        }
    }

    panic!("Puzzle has no solution!");
}

pub fn part2() -> i32 {
    let map: HashSet<i32> = lib::file_lines("./inputs/day1.txt")
        .map(|it| it.parse().unwrap())
        .collect();

    for num1 in &map {
        for num2 in &map {
            let remainder = 2020 - num1 - num2;
            if map.get(&remainder).is_some() {
                return num1 * num2 * remainder;
            }
        }
    }

    panic!("Puzzle has no solution!");
}
