use std::{collections::HashMap, error::Error, ops::Sub, str::FromStr};

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

struct Line {
    start: Point,
    end: Point,
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Line {
    fn is_diagonal(&self) -> bool {
        self.start.x != self.end.x && self.start.y != self.end.y
    }

    fn points(&self) -> Vec<Point> {
        let mut results = Vec::new();

        let mut delta = self.end - self.start;
        let len = delta.x.abs().max(delta.y.abs());
        delta.x /= len;
        delta.y /= len;

        for i in 0..=len {
            results.push(Point {
                x: self.start.x + delta.x * i,
                y: self.start.y + delta.y * i,
            })
        }
        results
    }
}

impl FromStr for Line {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" -> ");

        let xy_start: Vec<i32> = parts
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse().expect("Failed to parse"))
            .collect();

        let xy_end: Vec<i32> = parts
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse().expect("Failed to parse"))
            .collect();

        Ok(Line {
            start: Point {
                x: xy_start[0],
                y: xy_start[1],
            },
            end: Point {
                x: xy_end[0],
                y: xy_end[1],
            },
        })
    }
}

struct Vent {
    count: HashMap<Point, u32>,
}

impl Vent {
    fn add_line(&mut self, line: &Line) {
        for p in line.points() {
            *self.count.entry(p).or_default() += 1;
        }
    }
}

fn count_double_vents(lines: &[Line], ignore_diaganol: bool) -> usize {
    let mut vent = Vent {
        count: HashMap::new(),
    };

    for l in lines {
        if ignore_diaganol && l.is_diagonal() {
            continue;
        }

        vent.add_line(l);
    }

    vent.count.values().filter(|&&x| x >= 2).count()
}

pub fn calc(input: &str) -> (usize, usize) {
    let lines = aoc_lib::parse_lines(input);
    (
        count_double_vents(&lines, true),
        count_double_vents(&lines, false),
    )
}

#[test]
fn test() {
    let (p1, p2) = calc(&aoc_lib::read_file(2021, 5, true));
    assert_eq!(p1, 5);
    assert_eq!(p2, 12);
}
