use std::fs;

fn tree_count(slope_x: usize, slope_y: usize, map: &Vec<&str>) -> usize {
    map.iter()
        .step_by(slope_y)
        .enumerate()
        .filter(|(y, row)| row.chars().nth(y * slope_x % row.len()).unwrap() == '#')
        .count()
}

pub fn calc() -> (usize, usize) {
    let map = fs::read_to_string("./inputs/day1.txt")
        .unwrap()
        .lines()
        .collect::<Vec<_>>();

    let p1 = tree_count(3, 1, &map);
    let p2 = tree_count(1, 1, &map)
        * tree_count(3, 1, &map)
        * tree_count(5, 1, &map)
        * tree_count(7, 1, &map)
        * tree_count(1, 2, &map);

    (p1, p2)
}
