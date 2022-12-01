use aoc_lib::DoubleLineSplit;

pub fn calc(input: &str) -> (usize, usize) {
    let mut cals = input
        .split_at_doubleblank()
        .map(|food_list| food_list.lines().map(|x| x.parse::<usize>().unwrap()).sum())
        .collect::<Vec<usize>>();

    cals.sort_by(|a, b| b.cmp(a));

    (cals[0], cals.iter().take(3).sum())
}

#[test]
fn test() {
    let (p1, p2) = calc(&aoc_lib::read_file(2021, 1, true));
    assert_eq!(p1, 24000);
    assert_eq!(p2, 45000);
}
