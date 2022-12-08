use aoc_lib::AocSolution;
use itertools::Itertools;

pub struct Solution;

struct Grid {
    vals: Vec<u8>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(input: &str) -> Grid {
        Grid {
            vals: input
                .lines()
                .flat_map(|line| line.chars().map(|x| x.to_digit(10).unwrap() as u8))
                .collect(),
            width: input.lines().next().unwrap().chars().count(),
            height: input.lines().count(),
        }
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        self.vals[x + y * self.width]
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        let tree_height = self.get(x, y);

        (0..x).all(|xp| self.get(xp, y) < tree_height)
            || (x + 1..self.width).all(|xp| self.get(xp, y) < tree_height)
            || (0..y).all(|yp| self.get(x, yp) < tree_height)
            || (y + 1..self.height).all(|yp| self.get(x, yp) < tree_height)
    }

    fn scenic_score(&self, x: usize, y: usize) -> usize {
        let tree_height = self.get(x, y);

        let left = (1..x)
            .rev()
            .take_while(|&xp| self.get(xp, y) < tree_height)
            .count();
        let right = (x + 1..self.width - 1)
            .take_while(|&xp| self.get(xp, y) < tree_height)
            .count();
        let top = (1..y)
            .rev()
            .take_while(|&yp| self.get(x, yp) < tree_height)
            .count();
        let down = (y + 1..self.height - 1)
            .take_while(|&yp| self.get(x, yp) < tree_height)
            .count();
        (left + 1) * (right + 1) * (top + 1) * (down + 1)
    }
}

impl AocSolution for Solution {
    const DATE: (u32, u32) = (2022, 8);

    fn calc(input: &str) -> (u32, u32) {
        let grid = Grid::new(input);

        let count_visible = (0..grid.width)
            .cartesian_product(0..grid.height)
            .filter(|(x, y)| grid.is_visible(*x, *y))
            .count();

        let max_scenic = (1..grid.width - 1)
            .cartesian_product(1..grid.height - 1)
            .map(|(x, y)| grid.scenic_score(x, y))
            .max()
            .unwrap();

        (count_visible as u32, max_scenic as u32)
    }
}

#[test]
fn test() {
    Solution::test(21, 8);
}
