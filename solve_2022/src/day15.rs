use std::str::FromStr;

use aoc_lib::AocSolution;
use itertools::Itertools;

pub struct Solution;

struct Sensor {
    pos: (i32, i32),
    beacon: (i32, i32),
    radius: i32,
}

fn parse_xy(input: &str, prefix: &str) -> (i32, i32) {
    let (x, y) = input
        .strip_prefix(prefix)
        .map(|x| x.split_once(", ").unwrap())
        .unwrap();

    (
        x.strip_prefix("x=").map(|x| x.parse().unwrap()).unwrap(),
        y.strip_prefix("y=").map(|x| x.parse().unwrap()).unwrap(),
    )
}

impl FromStr for Sensor {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (sensor, beacon) = input.split_once(": ").ok_or(())?;
        let pos = parse_xy(sensor, "Sensor at ");
        let beacon = parse_xy(beacon, "closest beacon is at ");
        Ok(Self {
            pos,
            beacon,
            radius: manhattan_distance(pos, beacon),
        })
    }
}

fn manhattan_distance(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn count_unique(sensors: &[Sensor], y: i32, start: i32, end: i32) -> usize {
    let mut count = 0;
    let mut i = start;

    let ranges = sensors
        .iter()
        .filter_map(|sensor| {
            let check_dist = (y - sensor.pos.1).abs();

            if check_dist <= sensor.radius {
                let leftover_radius = sensor.radius - check_dist;
                Some((sensor.pos.0 - leftover_radius)..=(sensor.pos.0 + leftover_radius))
            } else {
                None
            }
        })
        .sorted_unstable_by_key(|r| *r.start());

    for r in ranges {
        if i < *r.start() {
            i = *r.start();
        }
        if i <= *r.end() {
            let i_start = i;
            i = (*r.end() + 1).min(end);

            count += (i - i_start) as usize;

            if i >= end {
                break;
            }
        }
    }
    count
}

impl AocSolution<usize, usize> for Solution {
    const DATE: (u32, u32) = (2022, 15);

    fn calc(input: &str) -> (usize, usize) {
        let mut lines = input.lines();
        let check: i32 = lines.next().and_then(|x| x.parse().ok()).unwrap();
        let max: usize = lines.next().and_then(|x| x.parse().ok()).unwrap();
        let sensors: Vec<Sensor> = lines.map(|l| l.parse().unwrap()).collect();

        let p1 = count_unique(&sensors, check, i32::MIN, i32::MAX)
            - sensors
                .iter()
                .map(|s| s.beacon)
                .unique()
                .filter(|b| b.1 == check)
                .count();

        let mut y = 0;

        let p2 = loop {
            let count = count_unique(&sensors, y, 0, max as i32);

            if count < max {
                let x = (0..max as i32)
                    .find(|&x| {
                        sensors
                            .iter()
                            .all(|s| manhattan_distance((x, y), s.pos) > s.radius)
                    })
                    .unwrap();
                break (x as usize) * 4000000 + y as usize;
            }
            y += 1;
        };

        (p1, p2)
    }
}

#[test]
fn test() {
    Solution::test(26, 56000011);
}
