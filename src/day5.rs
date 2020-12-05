use std::{collections::HashSet, fs};

fn seat(seat_str: &str) -> (u32, u32) {
    let mut lower_row = 0;
    let mut upper_row = 127;

    seat_str[0..7].chars().for_each(|c| {
        let mid = (lower_row + upper_row) / 2;
        match c {
            'F' => upper_row = mid,
            'B' => lower_row = mid + 1,
            _ => unreachable!(),
        }
    });

    let mut lower_column = 0;
    let mut upper_column = 7;

    seat_str[7..10].chars().for_each(|c| {
        let mid = (lower_column + upper_column) / 2;
        match c {
            'L' => upper_column = mid,
            'R' => lower_column = mid + 1,
            _ => unreachable!(),
        }
    });

    (lower_row, lower_column)
}

fn seat_id(seat: (u32, u32)) -> u32 {
    seat.0 * 8 + seat.1
}

pub fn calc() -> (u32, u32) {
    let seat_ids = fs::read_to_string("./inputs/day5.txt")
        .unwrap()
        .lines()
        .map(|s| seat_id(seat(s)))
        .collect::<HashSet<_>>();

    let max_seat_id = *seat_ids.iter().max().unwrap();

    let my_seat = (0..=max_seat_id)
        .filter(|id| id > &0 && !seat_ids.contains(id))
        .find(|id| seat_ids.contains(&(id - 1)) && seat_ids.contains(&(id + 1)))
        .unwrap();

    (max_seat_id, my_seat)
}

#[test]
fn test_seat() {
    assert_eq!(seat("BFFFBBFRRR"), (70, 7));
    assert_eq!(seat("FFFBBBFRRR"), (14, 7));
}
