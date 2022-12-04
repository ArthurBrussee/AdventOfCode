use aoc_lib::AocSolution;

pub struct Solution;

fn crack(nums: &[i64], preamble: usize) -> Option<i64> {
    for i in preamble..nums.len() {
        let mut found = false;
        'outer: for j in i - preamble..i {
            for k in j + 1..i {
                if nums[j] + nums[k] == nums[i] {
                    found = true;
                    break 'outer;
                }
            }
        }
        if !found {
            return Some(nums[i]);
        }
    }

    None
}

impl AocSolution<i64, i64> for Solution {
    const YEAR: u32 = 2020;
    const DAY: u32 = 9;

    fn calc(input: &str) -> (i64, i64) {
        let nums = input
            .lines()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i64>>();

        let p1 = crack(&nums, 25).unwrap();

        let sat = nums
            .iter()
            .scan(0, |sum, i| Some(*sum + i))
            .collect::<Vec<_>>();

        for i in 0..sat.len() {
            for j in i..sat.len() {
                if sat[j] - sat[i] == p1 {
                    let slice = &nums[i + 1..j + 1];
                    let p2 = slice.iter().min().unwrap() + slice.iter().max().unwrap();
                    return (p1, p2);
                }
            }
        }
        unreachable!("No solution");
    }
}
