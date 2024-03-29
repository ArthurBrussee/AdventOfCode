use std::collections::HashSet;

use aoc_lib::AocSolution;

enum Fold {
    Left(u32),
    Up(u32),
}

type Grid = HashSet<(u32, u32)>;

fn reflect(a: u32, b: u32) -> u32 {
    if a > b {
        b - (a - b)
    } else {
        a
    }
}

fn fold(pts: &Grid, fold: Fold) -> Grid {
    match fold {
        Fold::Left(x) => pts.iter().map(|pt| (reflect(pt.0, x), pt.1)).collect(),
        Fold::Up(y) => pts.iter().map(|pt| (pt.0, reflect(pt.1, y))).collect(),
    }
}

pub struct Solution;

impl AocSolution<usize, String> for Solution {
    const DATE: (u32, u32) = (2021, 13);

    fn calc(input: &str) -> (usize, String) {
        let (points_str, folds_str) = input.split_once("\n\n").unwrap();

        let points: Grid = points_str
            .lines()
            .map(|p| {
                let (x, y) = p.split_once(',').unwrap();
                (x.parse().unwrap(), y.parse().unwrap())
            })
            .collect();

        let mut folds = folds_str.lines().map(|f| {
            let (xy, val) = f.split(' ').last().unwrap().split_once('=').unwrap();
            let val = val.parse().unwrap();
            match xy {
                "x" => Fold::Left(val),
                "y" => Fold::Up(val),
                _ => unreachable!(),
            }
        });

        let mut cur_points = points;

        cur_points = fold(&cur_points, folds.next().unwrap());
        let p1 = cur_points.len();

        for f in folds {
            cur_points = fold(&cur_points, f);
        }

        let max_x = cur_points.iter().map(|p| p.0).max().unwrap();
        let max_y = cur_points.iter().map(|p| p.1).max().unwrap();

        let output = (0..=max_y)
            .map(|y| {
                (0..=max_x)
                    .map(|x| {
                        if cur_points.contains(&(x, y)) {
                            '#'
                        } else {
                            '.'
                        }
                    })
                    .collect::<String>()
            })
            .fold("\n".to_owned(), |a, b| a + &b + "\n");

        (p1, output)
    }
}

#[test]
fn test() {
    let (p1, _) = Solution::calc(&Solution::read_file(true));
    assert_eq!(p1, 17);
}
