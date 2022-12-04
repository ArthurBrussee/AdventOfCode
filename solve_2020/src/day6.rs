use aoc_lib::AocSolution;

pub struct Solution;

use aoc_lib::DoubleLineSplit;

impl AocSolution for Solution {
    const DATE: (u32, u32) = (2020, 6);

    fn calc(input: &str) -> (u32, u32) {
        input
            .split_at_doubleblank()
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
}
