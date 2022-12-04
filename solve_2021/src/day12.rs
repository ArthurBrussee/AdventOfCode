use std::{collections::HashMap, iter};

use aoc_lib::AocSolution;

struct Path<'a> {
    caves: Vec<Cave>,
    name_to_node: HashMap<&'a str, u8>,
}

struct Cave {
    connections: Vec<u8>,
    big: bool,
}

impl<'a> Path<'a> {
    fn get_or_add(&mut self, name: &'a str) -> u8 {
        if let Some(index) = self.name_to_node.get(name) {
            return *index;
        }

        let node = Cave {
            connections: Vec::new(),
            big: name.chars().next().unwrap().is_uppercase(),
        };

        let index = self.caves.len() as u8;
        self.name_to_node.insert(name, index);
        self.caves.push(node);
        index
    }

    fn add_path(
        &self,
        start: u8,
        end: u8,
        prev_path: &[u8],
        double_budget: &mut u32,
    ) -> Vec<Vec<u8>> {
        if start == end {
            return vec![prev_path.to_vec()];
        }

        // Can't revisit the start of the path.
        if !prev_path.is_empty() && start == prev_path[0] {
            return Vec::new();
        }

        // Can't visit small caves twice.
        let mut budget = *double_budget;
        if !self.caves[start as usize].big && prev_path.contains(&start) {
            if budget == 0 {
                return Vec::new();
            }
            budget -= 1;
        }

        let mut results = Vec::new();
        let connected = &self.caves[start as usize].connections;

        let from_path: Vec<u8> = prev_path.iter().copied().chain(iter::once(start)).collect();

        for &c in connected {
            let all_new_paths = self.add_path(c, end, &from_path, &mut budget);
            results.extend(all_new_paths);
        }

        results
    }

    fn pathfind(&self, start: &'a str, end: &'a str, allow_double: bool) -> Vec<Vec<u8>> {
        let start_node = self.name_to_node[&start];
        let end_node = self.name_to_node[&end];
        let mut budget = u32::from(allow_double);
        self.add_path(start_node, end_node, &Vec::new(), &mut budget)
    }

    fn connect(&mut self, name1: &'a str, name2: &'a str) {
        let node1 = self.get_or_add(name1);
        let node2 = self.get_or_add(name2);
        self.caves[node1 as usize].connections.push(node2);
        self.caves[node2 as usize].connections.push(node1);
    }
}

pub struct Solution;

impl AocSolution<usize, usize> for Solution {
    const DATE: (u32, u32) = (2021, 12);

    fn calc(input: &str) -> (usize, usize) {
        let mut path = Path {
            caves: Vec::new(),
            name_to_node: HashMap::new(),
        };

        for l in input.lines() {
            let (name1, name2) = l.split_once('-').unwrap();
            path.connect(name1, name2);
        }

        let p1 = path.pathfind("start", "end", false);
        let p2 = path.pathfind("start", "end", true);
        (p1.len(), p2.len())
    }
}

#[test]
fn test() {
    Solution::test(226, 3509);
}
