use std::collections::HashSet;

use aoc_lib::AocSolution;

struct Grid {
    values: Vec<u32>,
    width: i32,
    height: i32,
}

impl Grid {
    fn incr(&mut self, x: i32, y: i32) {
        if (0..self.width).contains(&x) && (0..self.height).contains(&y) {
            self.values[(x + y * self.width) as usize] += 1;
        }
    }

    fn flash(&mut self, x: i32, y: i32) {
        for dx in -1..=1i32 {
            for dy in -1..=1i32 {
                self.incr(x + dx, y + dy);
            }
        }
    }

    fn step(&mut self) -> usize {
        for val in self.values.iter_mut() {
            *val += 1;
        }

        let mut flashed: HashSet<usize> = HashSet::new();

        while let Some(index) = self
            .values
            .iter()
            .enumerate()
            .position(|(index, &x)| !flashed.contains(&index) && x > 9)
        {
            flashed.insert(index);
            let x = (index % (self.width as usize)) as i32;
            let y = (index / self.width as usize) as i32;
            self.flash(x, y);
        }

        for &flash in &flashed {
            self.values[flash] = 0;
        }

        flashed.len()
    }
}

pub struct Solution;

impl AocSolution<usize, usize> for Solution {
    const YEAR: u32 = 2021;
    const DAY: u32 = 11;

    fn calc(input: &str) -> (usize, usize) {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();

        let values: Vec<u32> = input
            .lines()
            .flat_map(|l| l.chars().map(|c| c.to_digit(10).unwrap()))
            .collect();

        let total = values.len();
        let mut grid = Grid {
            values,
            width: width as i32,
            height: height as i32,
        };

        let p1 = (0..100).map(|_| grid.step()).sum();
        let p2 = (101..).find(|_| grid.step() == total).unwrap();
        (p1, p2)
    }
}

#[test]
fn test() {
    Solution::test(1656, 195);
}
