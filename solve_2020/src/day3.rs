use aoc_lib::AocSolution;

pub struct Solution;

fn tree_count(slope_x: usize, slope_y: usize, map: &[&str]) -> u32 {
    map.iter()
        .step_by(slope_y)
        .enumerate()
        .filter(|(y, row)| row.chars().nth(y * slope_x % row.len()).unwrap() == '#')
        .count() as u32
}

impl AocSolution for Solution {
    const DATE: (u32, u32) = (2020, 3);

    fn calc(input: &str) -> (u32, u32) {
        let map: Vec<_> = input.lines().collect();

        let p1 = tree_count(3, 1, &map);

        let p2 = tree_count(1, 1, &map)
            * tree_count(3, 1, &map)
            * tree_count(5, 1, &map)
            * tree_count(7, 1, &map)
            * tree_count(1, 2, &map);

        (p1, p2)
    }
}
