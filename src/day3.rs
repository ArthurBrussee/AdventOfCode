use crate::lib;

fn tree_count(slope_x: usize, slope_y: usize, map: &Vec<String>) -> usize {
    map.iter()
        .step_by(slope_y)
        .enumerate()
        .filter(|(y, row)| row.chars().nth(y * slope_x % row.len()).unwrap() == '#')
        .count()
}

pub fn part1() -> usize {
    let map: Vec<String> = lib::file_lines("./inputs/day3.txt").collect();
    tree_count(3, 1, &map)
}

pub fn part2() -> usize {
    let map: Vec<String> = lib::file_lines("./inputs/day3.txt").collect();

    tree_count(1, 1, &map)
        * tree_count(3, 1, &map)
        * tree_count(5, 1, &map)
        * tree_count(7, 1, &map)
        * tree_count(1, 2, &map)
}
