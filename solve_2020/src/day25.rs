use aoc_lib::AocSolution;

fn transform_once(cur: usize, subject: usize) -> usize {
    (cur * subject) % 20201227
}

fn transform(subject: usize, loop_size: usize) -> usize {
    let mut cur = 1;
    for _ in 0..loop_size {
        cur = transform_once(cur, subject);
    }
    cur
}

pub struct Solution;

impl AocSolution<usize, usize> for Solution {
    const YEAR: u32 = 2020;
    const DAY: u32 = 25;

    fn calc(_: &str) -> (usize, usize) {
        let card_key_public = 2959251;
        let door_key_public = 4542595;

        let card_loop_size = {
            let mut attempt = 0;
            let mut cur = 1;
            loop {
                if cur == card_key_public {
                    break attempt;
                }
                cur = transform_once(cur, 7);
                attempt += 1;
            }
        };
        (transform(door_key_public, card_loop_size), 0)
    }
}
