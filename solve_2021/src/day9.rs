use std::collections::HashSet;

use aoc_lib::AocSolution;

struct Grid {
    values: Vec<u8>,
    width: i32,
    height: i32,
}

impl Grid {
    fn get(&self, x: i32, y: i32) -> Option<u8> {
        let in_bounds = x >= 0 && x < self.width && y >= 0 && y < self.height;
        in_bounds.then(|| self.values[(x + y * self.width) as usize])
    }
}

fn get_low_points(grid: &Grid) -> HashSet<(i32, i32)> {
    (0..grid.width)
        .flat_map(|x| {
            (0..grid.height).filter_map(move |y| {
                let center = grid.get(x, y).unwrap();
                let ids = [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)];

                let basin = ids
                    .iter()
                    .filter_map(|(xp, yp)| grid.get(*xp, *yp))
                    .all(|x| center < x);

                basin.then_some((x, y))
            })
        })
        .collect()
}

fn flood_recurs<'a>(
    grid: &Grid,
    x: i32,
    y: i32,
    pts: &'a mut HashSet<(i32, i32)>,
) -> &'a mut HashSet<(i32, i32)> {
    if pts.insert((x, y)) {
        for (xp, yp) in [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)] {
            if let Some(val) = grid.get(xp, yp) {
                if val != 9 {
                    flood_recurs(grid, xp, yp, pts);
                }
            }
        }
    }

    pts
}

fn basin_size(grid: &Grid, x: i32, y: i32) -> usize {
    let mut pts = HashSet::new();
    flood_recurs(grid, x, y, &mut pts).len()
}

fn parse_nums(input: &str) -> Grid {
    let width = input.lines().next().unwrap().chars().count() as i32;
    let height = input.lines().count() as i32;

    let values: Vec<u8> = input
        .lines()
        .flat_map(|x| x.chars().map(|c| c.to_digit(10).unwrap() as u8))
        .collect();

    Grid {
        values,
        width,
        height,
    }
}

pub struct Solution;

impl AocSolution<u32, usize> for Solution {
    const YEAR: u32 = 2021;
    const DAY: u32 = 9;

    fn calc(input: &str) -> (u32, usize) {
        let grid = parse_nums(input);

        let low_points = get_low_points(&grid);
        let p1 = low_points
            .iter()
            .map(|(x, y)| grid.get(*x, *y).unwrap() as u32 + 1)
            .sum();

        let mut basin_sizes: Vec<usize> = low_points
            .iter()
            .map(|(x, y)| basin_size(&grid, *x, *y))
            .collect();

        basin_sizes.sort_unstable();

        let p2 = basin_sizes.iter().rev().take(3).product();

        (p1, p2)
    }
}

#[test]
fn test() {
    Solution::test(15, 1134);
}
