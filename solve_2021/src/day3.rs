use aoc_lib::AocSolution;

fn parse_bin(str: &str) -> u16 {
    u16::from_str_radix(str, 2).expect("Failed to parse binary.")
}

fn bit_set(num: u16, bit: u8) -> bool {
    num & (1 << bit) > 0
}

fn calc_gamma_epsilon(nums: &[u16]) -> (u16, u16) {
    let mut freq_1 = [0_usize; 16];
    let mut high_bit = 1;

    for num in nums {
        for (i, freq) in freq_1.iter_mut().enumerate() {
            if num & (1 << i) > 0 {
                *freq += 1;
                high_bit = high_bit.max(i + 1);
            }
        }
    }

    let mut gamma = 0;

    for (i, &freq_1) in freq_1[0..high_bit].iter().enumerate() {
        let freq_0 = nums.len() - freq_1;

        let mask = 1 << i;
        if freq_1 >= freq_0 {
            gamma |= mask;
        }
    }

    let epsilon = !gamma & ((1 << high_bit) - 1);
    (gamma, epsilon)
}

fn calc_power(nums: &[u16]) -> u32 {
    let (gamma, epsilon) = calc_gamma_epsilon(nums);
    gamma as u32 * epsilon as u32
}

fn find(nums: &[u16], flip: bool) -> Option<u16> {
    let mut remaining: Vec<u16> = Vec::from(nums);

    for i in (0..16).rev() {
        let (gamma, epsilon) = calc_gamma_epsilon(&remaining);
        let mask = if !flip { gamma } else { epsilon };

        remaining.retain(|&r| bit_set(mask, i) == bit_set(r, i));

        if remaining.len() == 1 {
            return Some(remaining[0]);
        }
    }
    None
}

fn calc_diagnostic(nums: &[u16]) -> u32 {
    let co2_rating = find(nums, false).expect("Couldn't find C02 rating.");
    let o2_rating = find(nums, true).expect("Couldn't find 02 rating.");
    co2_rating as u32 * o2_rating as u32
}
pub struct Solution;

impl AocSolution for Solution {
    const YEAR: u32 = 2021;
    const DAY: u32 = 3;

    fn calc(input: &str) -> (u32, u32) {
        let input = aoc_lib::map_lines(input, parse_bin);
        let p1 = calc_power(&input);
        let p2 = calc_diagnostic(&input);
        (p1, p2)
    }
}

#[test]
fn test() {
    Solution::test(198, 230);
}
