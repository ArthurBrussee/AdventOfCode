use std::fs;

pub fn calc() -> (u32, u32) {
    fs::read_to_string("./inputs/day6.txt")
        .expect("Can't find input file.")
        .split("\n\n")
        .map(|str| {
            let (mask_or, mask_and) = str
                .split_whitespace()
                .map(|s| {
                    s.chars()
                        .fold(0u32, |acc, c| acc | 1 << (c as u8 - 'a' as u8))
                })
                .fold((0u32, std::u32::MAX), |(count_or, count_and), c| {
                    (count_or | c, count_and & c)
                });

            (mask_or.count_ones(), mask_and.count_ones())
        })
        .fold((0, 0), |(acc_a, acc_b), (a, b)| (acc_a + a, acc_b + b))
}
