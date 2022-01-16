use std::{collections::HashMap, hash::Hash};

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
enum PodType {
    Amber,
    Bronze,
    Copper,
    Desert,
}

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Pod {
    typ: PodType,
    pos: (u8, u8),
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct PodGame {
    pods: Vec<Pod>,
    rooms: u8,
}

fn path_corners(src: (u8, u8), dst: (u8, u8)) -> Vec<(u8, u8)> {
    let mut res = vec![src];
    if src.0 != dst.0 {
        if src.1 > 1 {
            res.push((src.0, 1));
        }
        if dst.1 > 1 {
            res.push((dst.0, 1));
        }
    }
    res.push(dst);
    res
}

fn get_distance(src: (u8, u8), dst: (u8, u8)) -> u32 {
    fn manhattan(src: (u8, u8), dst: (u8, u8)) -> u32 {
        (dst.0 as i8 - src.0 as i8).abs() as u32 + (dst.1 as i8 - src.1 as i8).abs() as u32
    }

    let corners = path_corners(src, dst);
    corners.windows(2).map(|w| manhattan(w[0], w[1])).sum()
}

impl<'a> From<&str> for PodGame {
    fn from(src: &str) -> Self {
        let mut pods: Vec<_> = src
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| match c {
                    'A' => Some(Pod {
                        typ: PodType::Amber,
                        pos: (x as u8, y as u8),
                    }),
                    'B' => Some(Pod {
                        typ: PodType::Bronze,
                        pos: (x as u8, y as u8),
                    }),
                    'C' => Some(Pod {
                        typ: PodType::Copper,
                        pos: (x as u8, y as u8),
                    }),
                    'D' => Some(Pod {
                        typ: PodType::Desert,
                        pos: (x as u8, y as u8),
                    }),
                    _ => None,
                })
            })
            .collect();

        pods.sort();
        PodGame {
            pods,
            rooms: (src.lines().count() - 3) as u8,
        }
    }
}

impl PodGame {
    fn get(&self, pos: (u8, u8)) -> Option<&Pod> {
        self.pods.iter().find(|p| p.pos == pos)
    }

    fn goal_pos(&self, pod: &Pod) -> (u8, u8) {
        let x = pod.typ.room_x();

        for dst in (0u8..self.rooms as u8).rev().map(|y| (x, 2 + y)) {
            if pod.pos == dst {
                return dst;
            }

            match self.get(dst) {
                None => return dst,
                Some(other) => {
                    if other.typ != pod.typ {
                        return dst;
                    }
                }
            }
        }

        unreachable!()
    }

    fn has_valid_path(&self, src: (u8, u8), dst: (u8, u8)) -> bool {
        let corners = path_corners(src, dst);
        corners.windows(2).all(|wp| {
            let mut pos = wp[0];
            let dir = (
                (wp[1].0 as i8 - wp[0].0 as i8).signum(),
                (wp[1].1 as i8 - wp[0].1 as i8).signum(),
            );
            while pos != wp[1] {
                pos.0 = (pos.0 as i8 + dir.0) as u8;
                pos.1 = (pos.1 as i8 + dir.1) as u8;
                if self.get(pos).is_some() {
                    return false;
                }
            }
            true
        })
    }

    fn is_finished(&self) -> bool {
        self.pods
            .iter()
            .all(|a| a.pos.0 == a.typ.room_x() && a.pos.1 > 1)
    }

    fn best_cost(&self, cost: u32, cache: &mut HashMap<PodGame, u32>) -> u32 {
        if self.is_finished() {
            return cost;
        }

        let mut try_move = |id: usize, next_pos: (u8, u8)| -> Option<u32> {
            let pod = self.pods[id];
            let mut next_game = self.clone();
            next_game.pods[id].pos = next_pos;
            next_game.pods.sort();
            let new_cost = cost + get_distance(pod.pos, next_pos) * pod.typ.step_cost();

            if let Some(&prev_cost) = cache.get(&next_game) {
                if prev_cost <= new_cost {
                    return None;
                }
            }

            cache.insert(next_game.clone(), new_cost);
            Some(next_game.best_cost(new_cost, cache))
        };

        let mut min_cost = u32::MAX;

        for (i, pod) in self.pods.iter().enumerate() {
            let goal_pos = self.goal_pos(pod);
            if pod.pos == goal_pos {
                continue;
            }

            if self.has_valid_path(pod.pos, goal_pos) {
                if let Some(cost) = try_move(i, goal_pos) {
                    min_cost = min_cost.min(cost);
                }

                continue;
            }

            if pod.pos.1 == 1 && (1..=11).contains(&pod.pos.0) {
                continue;
            }

            // Pod can only move into the hallway.
            let chk_hallway_tiles = [1, 2, 4, 6, 8, 10, 11];

            for dst in chk_hallway_tiles {
                if !self.has_valid_path(pod.pos, (dst, 1)) {
                    continue;
                }
                if let Some(cost) = try_move(i, (dst, 1)) {
                    min_cost = min_cost.min(cost);
                }
            }
        }

        min_cost
    }
}

impl PodType {
    fn room_x(&self) -> u8 {
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

pub fn calc(input: &str) -> (u32, u32) {
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

#[test]
fn test() {
    let (p1, p2) = calc(&aoc_lib::read_file(2021, 23, true));
    assert_eq!(p2, 44169);
    assert_eq!(p1, 12521);
}
