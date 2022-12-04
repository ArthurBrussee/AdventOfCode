use aoc_lib::AocSolution;

#[derive(Debug)]
struct Cube {
    on: bool,
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
}

impl Cube {
    fn volume(&self) -> usize {
        (self.x.1 + 1 - self.x.0) as usize
            * (self.y.1 + 1 - self.y.0) as usize
            * (self.z.1 + 1 - self.z.0) as usize
    }
}

fn intersect_range(range_a: (i32, i32), range_b: (i32, i32)) -> Option<(i32, i32)> {
    let range = (range_a.0.max(range_b.0), range_a.1.min(range_b.1));

    if range.1 >= range.0 {
        Some(range)
    } else {
        None
    }
}

fn intersect(cube: &Cube, region: &Cube) -> Option<Cube> {
    let x = intersect_range(cube.x, region.x);
    let y = intersect_range(cube.y, region.y);
    let z = intersect_range(cube.z, region.z);

    match (x, y, z) {
        (Some(x), Some(y), Some(z)) => Some(Cube {
            on: cube.on,
            x,
            y,
            z,
        }),
        _ => None,
    }
}

fn count_active(cubes: &[Cube], region: &Cube) -> usize {
    let mut count = 0;

    for (i, c) in cubes.iter().enumerate() {
        if let Some(c) = intersect(c, region) {
            let active_under_me = count_active(&cubes[0..i], &c);

            if c.on {
                let volume = c.volume();
                count += volume - active_under_me;
            } else {
                count -= active_under_me;
            };
        }
    }

    count
}

pub struct Solution;

impl AocSolution<usize, usize> for Solution {
    const DATE: (u32, u32) = (2021, 22);

    fn calc(input: &str) -> (usize, usize) {
        let cubes = input
            .lines()
            .map(|l| {
                let (on, coords) = l.split_once(' ').unwrap();
                let ranges = coords
                    .split(',')
                    .map(|e| {
                        let min_max = e[2..].split_once("..").unwrap();

                        let min_max: (i32, i32) =
                            (min_max.0.parse().unwrap(), min_max.1.parse().unwrap());
                        (min_max.0.min(min_max.1), min_max.0.max(min_max.1))
                    })
                    .collect::<Vec<_>>();

                Cube {
                    on: on == "on",
                    x: ranges[0],
                    y: ranges[1],
                    z: ranges[2],
                }
            })
            .collect::<Vec<_>>();

        let p1 = count_active(
            &cubes,
            &Cube {
                on: false,
                x: (-50, 50),
                y: (-50, 50),
                z: (-50, 50),
            },
        );

        let range = 500_000;
        let p2 = count_active(
            &cubes,
            &Cube {
                on: false,
                x: (-range, range),
                y: (-range, range),
                z: (-range, range),
            },
        );

        (p1, p2)
    }
}

#[test]
fn test() {
    Solution::test(474140, 2758514936282235);
}
