use aoc_lib::AocSolution;

pub struct Solution;

use std::collections::HashSet;

fn seat(seat_str: &str) -> (u32, u32) {
    let row = seat_str[0..7]
        .chars()
        .fold((0u32, 127u32), |(lower_row, upper_row), c| {
            let mid = (lower_row + upper_row) / 2;
            match c {
                'F' => (lower_row, mid),
                'B' => (mid + 1, upper_row),
                _ => unreachable!(),
            }
        })
        .0;

    let column = seat_str[7..10]
        .chars()
        .fold((0u32, 7u32), |(lower_column, upper_column), c| {
            let mid = (lower_column + upper_column) / 2;
            match c {
                'L' => (lower_column, mid),
                'R' => (mid + 1, upper_column),
                _ => unreachable!(),
            }
        })
        .0;
    (row, column)
}

fn seat_id(seat: (u32, u32)) -> u32 {
    seat.0 * 8 + seat.1
}

impl AocSolution for Solution {
    const YEAR: u32 = 2020;
    const DAY: u32 = 5;

    fn calc(input: &str) -> (u32, u32) {
        let seat_ids = input
            .lines()
            .map(|s| seat_id(seat(s)))
            .collect::<HashSet<_>>();

        let max_seat_id = *seat_ids.iter().max().unwrap();

        let my_seat = (0..=max_seat_id)
            .filter(|id| id > &0 && !seat_ids.contains(id))
            .find(|id| [id - 1, id + 1].iter().all(|nid| seat_ids.contains(nid)))
            .unwrap();

        (max_seat_id, my_seat)
    }
}

#[test]
fn test_seat() {
    assert_eq!(seat("BFFFBBFRRR"), (70, 7));
    assert_eq!(seat("FFFBBBFRRR"), (14, 7));
}
