use std::collections::{HashSet, VecDeque};

use aoc_lib::AocSolution;
use itertools::Itertools;

pub struct Solution;

struct HeightMap {
    vals: Vec<u8>,
    width: u32,
    height: u32,
}

impl HeightMap {
    fn get(&self, pos: (u32, u32)) -> u8 {
        self.vals[(pos.0 + pos.1 * self.width) as usize]
    }

    fn shortest_path(&self, start: (u32, u32), goal: (u32, u32)) -> Option<Vec<(u32, u32)>> {
        let mut visited = HashSet::new();
        visited.insert(start);

        let mut q = VecDeque::new();
        q.push_back(vec![start]);

        while let Some(path) = q.pop_front() {
            let node = *path.last().unwrap();

            if node == goal {
                return Some(path);
            }

            let edges = [(1, 0), (-1, 0), (0, 1), (0, -1)];

            for e in edges {
                let to: (i32, i32) = (node.0 as i32 + e.0, node.1 as i32 + e.1);

                if (0..self.width as i32).contains(&to.0) && (0..self.height as i32).contains(&to.1)
                {
                    let to = (to.0.try_into().unwrap(), to.1.try_into().unwrap());

                    if self.get(to) <= self.get(node) + 1 && visited.insert(to) {
                        let new_path = path.iter().copied().chain([to]).collect();
                        q.push_back(new_path);
                    }
                }
            }
        }

        None
    }
}

impl AocSolution<usize, usize> for Solution {
    const DATE: (u32, u32) = (2022, 12);

    fn calc(input: &str) -> (usize, usize) {
        let vals = input
            .lines()
            .flat_map(|l| l.chars())
            .map(|c| match c {
                'S' => b'a',
                'E' => b'z',
                c => c as u8,
            })
            .collect();

        let width = input.lines().next().unwrap().len() as u32;
        let height = input.lines().count() as u32;

        let heightmap = HeightMap {
            vals,
            width,
            height,
        };

        let find = |tile| {
            let pos = input
                .lines()
                .flat_map(|l| l.chars())
                .position(|c| c == tile)
                .unwrap() as u32;
            (pos % width, pos / width)
        };

        let start = find('S');
        let goal = find('E');

        let p1 = heightmap.shortest_path(start, goal).unwrap().len() - 1;

        let p2 = (0..width)
            .cartesian_product(0..height)
            .filter(|&(x, y)| heightmap.get((x, y)) == b'a')
            .filter_map(|start| {
                heightmap
                    .shortest_path(start, goal)
                    .map(|path| path.len() - 1)
            })
            .min()
            .unwrap();

        (p1, p2)
    }
}

#[test]
fn test() {
    Solution::test(31, 29);
}
