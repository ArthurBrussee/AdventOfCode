use aoc_lib::AocSolution;
use fxhash::FxHashSet;
use itertools::Itertools;

pub struct Solution;

fn points_between(start: (i32, i32), end: (i32, i32)) -> Vec<(i32, i32)> {
    let mut points = Vec::new();
    let (dx, dy) = (end.0 - start.0, end.1 - start.1);
    let (dx, dy) = (dx.signum(), dy.signum());
    let mut cur = start;
    while cur != end {
        points.push(cur);
        cur.0 += dx;
        cur.1 += dy;
    }
    points.push(end);
    points
}

impl AocSolution<usize, usize> for Solution {
    const DATE: (u32, u32) = (2022, 14);

    fn calc(input: &str) -> (usize, usize) {
        let obstacles: FxHashSet<(i32, i32)> = input
            .lines()
            .flat_map(|l| {
                let vertices: Vec<(i32, i32)> = l
                    .split(" -> ")
                    .map(|p| {
                        p.split(',')
                            .map(|x| x.parse::<i32>().unwrap())
                            .collect_tuple()
                            .unwrap()
                    })
                    .collect();

                vertices
                    .windows(2)
                    .map(|w| w.iter().collect_tuple().unwrap())
                    .flat_map(|(&v1, &v2)| points_between(v1, v2))
                    .collect_vec()
            })
            .collect();

        let p1 = simulate_sand(&obstacles, true);
        let p2 = simulate_sand(&obstacles, false);

        (p1, p2)
    }
}

fn simulate_sand(obstacles: &FxHashSet<(i32, i32)>, floor_is_end: bool) -> usize {
    let mut obstacles = obstacles.clone();

    let highest_y = *obstacles.iter().map(|(_, y)| y).max().unwrap();
    let start_count = obstacles.len();

    'outer: loop {
        let mut sand_pos = (500, 0);

        loop {
            if !obstacles.contains(&(sand_pos.0, sand_pos.1 + 1)) {
                sand_pos.1 += 1;
            } else if !obstacles.contains(&(sand_pos.0 - 1, sand_pos.1 + 1)) {
                sand_pos.0 -= 1;
                sand_pos.1 += 1;
            } else if !obstacles.contains(&(sand_pos.0 + 1, sand_pos.1 + 1)) {
                sand_pos.0 += 1;
                sand_pos.1 += 1;
            } else {
                obstacles.insert(sand_pos);

                if sand_pos == (500, 0) {
                    break 'outer;
                } else {
                    break;
                }
            }

            if sand_pos.1 > highest_y {
                if floor_is_end {
                    break 'outer;
                } else {
                    obstacles.insert(sand_pos);
                    break;
                }
            }
        }
    }
    obstacles.len() - start_count
}

#[test]
fn test() {
    Solution::test(24, 93);
}
