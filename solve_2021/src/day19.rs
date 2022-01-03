use std::{
    collections::HashSet,
    hash::Hash,
    ops::{Add, Sub},
};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Default, Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn sin_cos(angle: u32) -> (i32, i32) {
    match angle {
        0 => (0, 1),
        90 => (1, 0),
        180 => (0, -1),
        270 => (-1, 0),
        _ => unreachable!(),
    }
}

impl Point {
    fn rotate(&self, angle_x: u32, angle_y: u32, angle_z: u32) -> Point {
        let (x, y, z) = (self.x, self.y, self.z);
        // Manual matrix multiplications.
        let (s, c) = sin_cos(angle_x);
        let (x, y, z) = (x, y * c - z * s, y * s + z * c);
        let (s, c) = sin_cos(angle_y);
        let (x, y, z) = (x * c + z * s, y, -x * s + z * c);
        let (s, c) = sin_cos(angle_z);
        let (x, y, z) = (x * c - y * s, x * s + y * c, z);
        Point { x, y, z }
    }

    fn l1(&self) -> u32 {
        (self.x.abs() + self.y.abs() + self.z.abs()) as u32
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

struct Readings {
    points: Vec<Point>,
}

impl Readings {
    fn rotated(&self, angle_x: u32, angle_y: u32, angle_z: u32) -> Readings {
        Readings {
            points: self
                .points
                .iter()
                .map(move |&p| p.rotate(angle_x, angle_y, angle_z))
                .collect(),
        }
    }
}

impl From<&str> for Readings {
    fn from(input: &str) -> Self {
        let mut lines = input.lines();

        // Header
        lines.next();

        let points = lines
            .map(|l| {
                let mut numbers = l.split(',');

                Point {
                    x: numbers.next().and_then(|x| x.parse().ok()).unwrap(),
                    y: numbers.next().and_then(|y| y.parse().ok()).unwrap(),
                    z: numbers.next().and_then(|z| z.parse().ok()).unwrap(),
                }
            })
            .collect();

        Readings { points }
    }
}

type BitType = u32;

struct BeaconMap {
    bloom_bits: [BitType; (Self::LEN / Self::BITS) as usize],
    pts: HashSet<Point>,
}

impl BeaconMap {
    const BITS: u32 = BitType::BITS;
    const LEN: u32 = 4096 * BeaconMap::BITS;

    fn new() -> Self {
        Self {
            bloom_bits: [0; (Self::LEN / Self::BITS) as usize],
            pts: HashSet::default(),
        }
    }

    fn bloom_bit(pt: Point) -> u32 {
        let num = unsafe {
            // SAFETY: Just a hack to convert bitwise i32 to u32.
            std::mem::transmute::<Point, [u32; 3]>(pt)
        };

        (num[0] + (num[1] << 11) + (num[2] << 22)) % Self::LEN
    }

    fn insert(&mut self, pt: Point) {
        let bit = BeaconMap::bloom_bit(pt);
        self.bloom_bits[(bit / Self::BITS) as usize] |= 1 << (bit % Self::BITS);
        self.pts.insert(pt);
    }

    fn matches(&self, pts: &[Point], offset: Point) -> bool {
        let mut matches = 0usize;

        for (i, &p) in pts.iter().enumerate() {
            let chk = p + offset;

            let bit = BeaconMap::bloom_bit(chk);
            let chunk = self.bloom_bits.get((bit / Self::BITS) as usize).unwrap();
            let res = chunk & (1 << (bit % Self::BITS));

            if res > 0 {
                matches += 1;
            }

            let remaining = pts.len() - i;

            if remaining + matches < 12 {
                return false;
            }
        }

        if matches >= 12 {
            return pts
                .iter()
                .copied()
                .filter(|&p| self.pts.contains(&(p + offset)))
                .count()
                >= 12;
        }

        false
    }
}

pub fn calc(input: &str) -> (usize, u32) {
    let readings: Vec<_> = input.split("\n\n").map(Readings::from).collect();
    let mut beacons = BeaconMap::new();

    for p in readings[0].points.iter().copied() {
        beacons.insert(p);
    }

    let mut remaining_templates: Vec<_> = readings.iter().skip(1).collect();
    let mut offsets = vec![Point::default()];

    while !remaining_templates.is_empty() {
        remaining_templates.retain(|t| {
            for face in 0..6 {
                for rot in [0, 90, 180, 270] {
                    let (ax, ay, az) = match face {
                        0 => (rot, 0, 0),   // +x
                        1 => (rot, 180, 0), // -x
                        2 => (rot, 0, 90),  // +y
                        3 => (rot, 0, 270), // -y
                        4 => (rot, 90, 0),  // +Z
                        5 => (rot, 270, 0), // -Z
                        _ => unreachable!(),
                    };

                    let rotated = t.rotated(ax, ay, az);

                    for &beacon_pos in &beacons.pts {
                        for &base_point in rotated.points.iter() {
                            let offset = beacon_pos - base_point;

                            if beacons.matches(&rotated.points, offset) {
                                offsets.push(offset);

                                for p in rotated.points.into_iter().map(|p| p + offset) {
                                    beacons.insert(p);
                                }

                                return false;
                            }
                        }
                    }
                }
            }
            true
        });
    }

    let mut max_dist = 0;

    for &p1 in &offsets {
        for &p2 in &offsets {
            let mag = (p2 - p1).l1();
            max_dist = max_dist.max(mag);
        }
    }

    (beacons.pts.len(), max_dist)
}

#[test]
fn test() {
    let (p1, p2) = calc(&aoc_lib::read_file(2021, 19, true));
    assert_eq!(p1, 79);
    assert_eq!(p2, 3621);
}
