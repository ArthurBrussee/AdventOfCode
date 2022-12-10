use aoc_lib::AocSolution;
use itertools::Itertools;

pub struct Solution;

enum Dir {
    Right,
    Up,
    Left,
    Down,
}

impl Dir {
    fn delta(&self) -> (i32, i32) {
        match self {
            Dir::Right => (1, 0),
            Dir::Up => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Down => (0, -1),
        }
    }
}

fn sim_rope<const N: usize>(commands: &[Dir]) -> u32 {
    let mut positions = [(0, 0); N];

    commands
        .iter()
        .map(|command| {
            let delta = command.delta();
            positions[0] = (positions[0].0 + delta.0, positions[0].1 + delta.1);

            for i in 1..N {
                let pos = positions[i];
                let prev_pos = positions[i - 1];
                let dif = ((prev_pos.0 - pos.0), (prev_pos.1 - pos.1));

                if dif.0.abs() == 2 || dif.1.abs() == 2 {
                    positions[i] = (pos.0 + dif.0.signum(), pos.1 + dif.1.signum());
                }
            }
            positions[N - 1]
        })
        .unique()
        .count() as u32
}

impl AocSolution for Solution {
    const DATE: (u32, u32) = (2022, 9);

    fn calc(input: &str) -> (u32, u32) {
        let directions: Vec<Dir> = input
            .lines()
            .flat_map(|s| {
                let (c, n) = s.split_once(' ').unwrap();
                let n = n.parse().unwrap();
                (0..n).map(move |_| match c {
                    "R" => Dir::Right,
                    "U" => Dir::Up,
                    "L" => Dir::Left,
                    "D" => Dir::Down,
                    _ => unreachable!(),
                })
            })
            .collect();

        (sim_rope::<2>(&directions), sim_rope::<10>(&directions))
    }
}

#[test]
fn test() {
    Solution::test(13, 1);
}
