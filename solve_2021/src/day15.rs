use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use aoc_lib::AocSolution;

struct Grid {
    values: Vec<u8>,
    width: i32,
    height: i32,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    pos: (i32, i32),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Grid {
    fn get(&self, x: i32, y: i32) -> Option<u8> {
        let in_bounds = (0..self.width).contains(&x) && (0..self.height).contains(&y);
        in_bounds.then(|| self.values[(x + y * self.width) as usize])
    }

    fn calc_path(&self, start: (i32, i32), goal: (i32, i32)) -> Option<usize> {
        let mut dist = HashMap::new();
        let mut heap = BinaryHeap::new();

        dist.insert(start, 0);
        heap.push(State {
            cost: 0,
            pos: start,
        });

        while let Some(State { cost, pos }) = heap.pop() {
            if pos == goal {
                return Some(cost);
            }

            if cost > dist[&pos] {
                continue;
            }

            let neighbours = [
                (pos.0 - 1, pos.1),
                (pos.0, pos.1 - 1),
                (pos.0 + 1, pos.1),
                (pos.0, pos.1 + 1),
            ];

            for n in neighbours {
                if let Some(val) = self.get(n.0, n.1) {
                    let next = State {
                        cost: cost + val as usize,
                        pos: n,
                    };

                    let neighbour_risk = dist.entry(n).or_insert(usize::MAX);

                    // If so, add it to the frontier and continue
                    if next.cost < *neighbour_risk {
                        heap.push(next);
                        dist.insert(next.pos, next.cost);
                    }
                }
            }
        }
        None
    }
}

pub struct Solution;

impl AocSolution<usize, usize> for Solution {
    const DATE: (u32, u32) = (2021, 15);

    fn calc(input: &str) -> (usize, usize) {
        let width = input.lines().next().unwrap().len() as i32;
        let height = input.lines().count() as i32;
        let values = input
            .lines()
            .flat_map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8))
            .collect();

        let grid = Grid {
            values,
            width,
            height,
        };

        let p1 = grid.calc_path((0, 0), (width - 1, height - 1)).unwrap();

        let big_values = (0..height * 5)
            .flat_map(|y| (0..width * 5).map(move |x| (x, y)))
            .map(|(x, y)| {
                let base_val = grid.get(x % width, y % height).unwrap();
                let xx = x / width;
                let yy = y / height;
                let mut val = base_val + xx as u8 + yy as u8;
                if val > 9 {
                    val -= 9;
                }
                val
            })
            .collect();

        let big_grid = Grid {
            values: big_values,
            width: width * 5,
            height: height * 5,
        };

        let p2 = big_grid
            .calc_path((0, 0), (width * 5 - 1, height * 5 - 1))
            .unwrap();

        (p1, p2)
    }
}

#[test]
fn test() {
    Solution::test(40, 315);
}
