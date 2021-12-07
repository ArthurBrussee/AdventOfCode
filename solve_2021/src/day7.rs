fn min_fuel(nums: &[i32], cost_func: fn(i32) -> i32) -> i32 {
    let &min = nums.iter().min().unwrap();
    let &max = nums.iter().max().unwrap();

    (min..max)
        .map(|x| nums.iter().map(|n| cost_func(n - x)).sum())
        .min()
        .unwrap()
}

fn sum_to(n: i32) -> i32 {
    n * (n + 1) / 2
}

pub fn calc() -> (i32, i32) {
    let nums: Vec<i32> = aoc_lib::read_file("./solve_2021/inputs/day7.txt")
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let p1 = min_fuel(&nums, |x| x.abs());
    let p2 = min_fuel(&nums, |x| sum_to(x.abs()));
    (p1, p2)
}

#[test]
fn test() {
    let nums = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    assert_eq!(min_fuel(&nums, |x| x.abs()), 37);
    assert_eq!(min_fuel(&nums, |x| sum_to(x.abs())), 168);
}
