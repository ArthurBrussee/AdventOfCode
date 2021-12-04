use std::fs;

pub fn calc() -> (u32, u32) {
    fs::read_to_string("./solve_2020/inputs/day6.txt")
        .expect("Can't find input file.")
        .split("\n\n")
        .map(|group| {
            group
                .split_whitespace()
                .map(|passenger| passenger.bytes().fold(0, |mask, c| mask | 1 << (c - b'a')))
                .fold((0, std::u32::MAX), |(count_or, count_and), c| {
                    (count_or | c, count_and & c)
                })
        })
        .fold((0, 0), |(count_or, count_and), (mask_or, mask_and)| {
            (
                count_or + mask_or.count_ones(),
                count_and + mask_and.count_ones(),
            )
        })
}
