use aoc_lib::AocSolution;

fn calc_increasing(nums: &[usize]) -> usize {
    nums.windows(2).filter(|w| w[0] < w[1]).count()
}

fn calc_window_sum(nums: &[usize]) -> usize {
    let windowed: Vec<usize> = nums.windows(3).map(|w| w.iter().sum()).collect();
    calc_increasing(&windowed)
}

pub struct Solution;

impl AocSolution<usize, usize> for Solution {
    const DATE: (u32, u32) = (2021, 1);

    fn calc(input: &str) -> (usize, usize) {
        let inputs: Vec<usize> = aoc_lib::parse_lines(input);
        (calc_increasing(&inputs), calc_window_sum(&inputs))
    }
}

#[test]
fn test_p1() {
    let inputs = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(calc_increasing(&inputs), 7)
}

#[test]
fn test_p2() {
    let inputs = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let sum = calc_window_sum(&inputs);
    assert_eq!(sum, 5)
}
