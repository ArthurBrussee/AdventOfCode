use std::{collections::HashMap, hash::Hash};

use aoc_lib::AocSolution;

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
enum PodType {
    Amber,
    Bronze,
    Copper,
    Desert,
}

type Pos = (usize, usize);

#[derive(Clone, Hash, PartialEq, Eq)]
struct PodGame {
    pods: Vec<Option<PodType>>,
    width: usize,
    height: usize,
}

fn get_distance(src: Pos, dst: Pos) -> u32 {
    fn manhattan(src: Pos, dst: Pos) -> u32 {
        (dst.0 as isize - src.0 as isize).unsigned_abs() as u32
            + (dst.1 as isize - src.1 as isize).unsigned_abs() as u32
    }

    let mut distance = 0;
    let mut last_pos = src;

    if src.0 != dst.0 {
        if src.1 > 1 {
            last_pos = (src.0, 1);
            distance += manhattan(src, (src.0, 1));
        }
        if dst.1 > 1 {
            distance += manhattan(last_pos, (dst.0, 1));
            last_pos = (dst.0, 1);
        }
    }

    distance += manhattan(last_pos, dst);
    distance
}

impl From<&str> for PodGame {
    fn from(src: &str) -> Self {
        let width = src.lines().next().unwrap().len();
        let height = src.lines().count();

        let mut game = PodGame {
            pods: vec![None; width * height],
            width,
            height,
        };

        for (pos, typ) in src.lines().enumerate().flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                'A' => Some(((x, y), PodType::Amber)),
                'B' => Some(((x, y), PodType::Bronze)),
                'C' => Some(((x, y), PodType::Copper)),
                'D' => Some(((x, y), PodType::Desert)),
                _ => None,
            })
        }) {
            game.set(pos, Some(typ));
        }

        game
    }
}

impl PodGame {
    fn set(&mut self, pos: Pos, val: Option<PodType>) {
        self.pods[pos.0 + pos.1 * self.width] = val;
    }

    fn get(&self, pos: Pos) -> Option<PodType> {
        self.pods[pos.0 + pos.1 * self.width]
    }

    fn rooms(&self) -> usize {
        self.height - 3
    }

    fn has_straight_path(&self, start: Pos, dst: Pos) -> bool {
        let mut pos = start;
        let dir = (
            (dst.0 as isize - start.0 as isize).signum(),
            (dst.1 as isize - start.1 as isize).signum(),
        );
        while pos != dst {
            pos.0 = (pos.0 as isize + dir.0) as usize;
            pos.1 = (pos.1 as isize + dir.1) as usize;
            if self.get(pos).is_some() {
                return false;
            }
        }
        true
    }

    fn has_valid_path(&self, src: Pos, dst: Pos) -> bool {
        let mut last_pos = src;

        if src.0 != dst.0 {
            if src.1 > 1 {
                if !self.has_straight_path(last_pos, (src.0, 1)) {
                    return false;
                }
                last_pos = (src.0, 1);
            }
            if dst.1 > 1 {
                if !self.has_straight_path(last_pos, (dst.0, 1)) {
                    return false;
                }
                last_pos = (dst.0, 1);
            }
        }
        if !self.has_straight_path(last_pos, dst) {
            return false;
        }
        true
    }

    fn is_finished(&self) -> bool {
        for y in (0..self.rooms()).rev().map(|y| 2 + y) {
            if self.get((PodType::Amber.room_x(), y)) != Some(PodType::Amber)
                || self.get((PodType::Bronze.room_x(), y)) != Some(PodType::Bronze)
                || self.get((PodType::Copper.room_x(), y)) != Some(PodType::Copper)
                || self.get((PodType::Desert.room_x(), y)) != Some(PodType::Desert)
            {
                return false;
            }
        }
        true
    }

    fn try_move(
        &self,
        pos: Pos,
        next_pos: Pos,
        cost: u32,
        cache: &mut HashMap<PodGame, u32>,
    ) -> Option<u32> {
        let typ = self.get(pos).unwrap();

        let mut next_game = self.clone();
        next_game.set(pos, None);
        next_game.set(next_pos, Some(typ));
        let new_cost = cost + get_distance(pos, next_pos) * typ.step_cost();

        let entry = cache.get_mut(&next_game);

        if let Some(prev_cost) = entry {
            if *prev_cost <= new_cost {
                return None;
            }
            *prev_cost = new_cost;
        } else {
            cache.insert(next_game.clone(), new_cost);
        }

        Some(next_game.best_cost(new_cost, cache))
    }

    fn best_cost(&self, cost: u32, cache: &mut HashMap<PodGame, u32>) -> u32 {
        if self.is_finished() {
            return cost;
        }

        let mut min_cost = u32::MAX;

        'outer: for (i, typ) in self
            .pods
            .iter()
            .enumerate()
            .filter_map(|(i, typ)| typ.map(|t| (i, t)))
        {
            let pos = ((i % self.width), (i / self.width));

            let mut goal_pos = (0, 0);
            let x = typ.room_x();

            for dst in (0..self.rooms()).rev().map(|y| (x, 2 + y)) {
                if pos == dst {
                    continue 'outer;
                }

                match self.get(dst) {
                    None => {
                        goal_pos = dst;
                        break;
                    }
                    Some(new_type) => {
                        if new_type != typ {
                            goal_pos = dst;
                            break;
                        }
                    }
                }
            }

            if self.has_valid_path(pos, goal_pos) {
                if let Some(cost) = self.try_move(pos, goal_pos, cost, cache) {
                    min_cost = min_cost.min(cost);
                }
                continue;
            }

            if pos.1 == 1 && pos.0 >= 1 && pos.0 <= 11 {
                continue;
            }

            // Pod can only move into the hallway.
            for dst in [1, 2, 4, 6, 8, 10, 11] {
                if self.has_valid_path(pos, (dst, 1)) {
                    if let Some(cost) = self.try_move(pos, (dst, 1), cost, cache) {
                        min_cost = min_cost.min(cost);
                    }
                }
            }
        }

        min_cost
    }
}

impl PodType {
    fn room_x(&self) -> usize {
        match self {
            PodType::Amber => 3,
            PodType::Bronze => 5,
            PodType::Copper => 7,
            PodType::Desert => 9,
        }
    }

    fn step_cost(&self) -> u32 {
        match self {
            PodType::Amber => 1,
            PodType::Bronze => 10,
            PodType::Copper => 100,
            PodType::Desert => 1000,
        }
    }
}

pub struct Solution;

impl AocSolution<u32, u32> for Solution {
    const YEAR: u32 = 2021;
    const DAY: u32 = 23;

    fn calc(input: &str) -> (u32, u32) {
        let mut lines = input.lines().collect::<Vec<_>>();
        lines.insert(3, "  #D#C#B#A#  ");
        lines.insert(4, "  #D#B#A#C#  ");

        let input_p2 = lines.join("\n");

        let game1 = PodGame::from(input);
        let game2 = PodGame::from(input_p2.as_str());

        let p1 = game1.best_cost(0, &mut HashMap::new());
        let p2 = game2.best_cost(0, &mut HashMap::new());

        (p1, p2)
    }
}

#[test]
fn test() {
    Solution::test(12521, 44169);
}
