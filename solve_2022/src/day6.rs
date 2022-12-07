use aoc_lib::AocSolution;

pub struct Solution;

impl AocSolution<usize, usize> for Solution {
    const DATE: (u32, u32) = (2022, 6);

    fn calc(input: &str) -> (usize, usize) {
        fn unique_chars(bytes: &[u8]) -> bool {
            bytes
                .iter()
                .try_fold(0u32, |acc, &b| {
                    let mask = 1 << (b - b'a');
                    if acc & mask == mask {
                        None
                    } else {
                        Some(acc | mask)
                    }
                })
                .is_some()
        }

        let find_unique = |n| input.as_bytes().windows(n).position(unique_chars).unwrap() + n;
        (find_unique(4), find_unique(14))
    }
}

#[test]
fn test() {
    Solution::test(7, 19);
}
