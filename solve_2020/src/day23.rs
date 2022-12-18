use aoc_lib::AocSolution;

fn cup_game(start: &[u32], steps: u32) -> Vec<u32> {
    let len = start.len();

    let mut cups = vec![0; len + 1];
    for i in 0..start.len() {
        let label = start[i];
        let next = start[(i + 1) % start.len()];
        cups[label as usize] = next;
    }

    let mut cur_cup = start[0];

    let cmin = 1u32;
    let cmax = start.len() as u32;

    for _ in 0..steps {
        let mut cur_val = cur_cup;

        let pick1 = cups[cur_cup as usize];
        let pick2 = cups[pick1 as usize];
        let pick3 = cups[pick2 as usize];

        cups[cur_cup as usize] = cups[pick3 as usize];
        cur_cup = cups[cur_cup as usize];

        let destination_cup = loop {
            cur_val = if cur_val <= cmin { cmax } else { cur_val - 1 };
            if cur_val != pick1 && cur_val != pick2 && cur_val != pick3 {
                break cur_val;
            }
        };

        let end_dest = cups[destination_cup as usize];
        cups[destination_cup as usize] = pick1;
        cups[pick3 as usize] = end_dest;
    }
    cups
}

pub struct Solution;
impl AocSolution<String, u64> for Solution {
    const DATE: (u32, u32) = (2020, 23);

    fn calc(_: &str) -> (String, u64) {
        let base_cups = vec![2, 5, 3, 1, 4, 9, 8, 6, 7];
        let cups = cup_game(&base_cups, 100);

        let mut p1 = "".to_owned();
        let mut cur = 1;
        for _ in 0..base_cups.len() - 1 {
            cur = cups[cur] as usize;
            p1 += &cur.to_string()
        }

        let mut large_cups = base_cups;
        large_cups.extend(10u32..=1_000_000);

        let cups_big = cup_game(&large_cups, 10_000_000);
        let next1 = cups_big[1];
        let next2 = cups_big[next1 as usize];
        (p1, next1 as u64 * next2 as u64)
    }
}
