use std::collections::HashMap;

use aoc_lib::AocSolution;

pub struct Solution;

impl AocSolution<i64, i64> for Solution {
    const YEAR: u32 = 2021;
    const DAY: u32 = 24;

    fn calc(_: &str) -> (i64, i64) {
        // div z 1     div z 1     div z 1     div z 1     div z 1    div z 26   div z 1    div z 26   div z 26  div z 26  div z 1    div z 26   div z 26  div z 26
        let a_vals = [1, 1, 1, 1, 1, 26, 1, 26, 26, 26, 1, 26, 26, 26];

        // add x 11    add x 13    add x 12    add x 15    add x 10   add x -1   add x 14   add x -8   add x -7  add x -8  add x 11   add x -2   add x -2  add x -13
        let b_vals = [11, 13, 12, 15, 10, -1, 14, -8, -7, -8, 11, -2, -2, -13];

        // add y 5     add y 5     add y 1     add y 15    add y 2    add y 2    add y 5    add y 8    add y 14  add y 12  add y 7    add y 14   add y 13  add y 6
        let c_vals = [5, 5, 1, 15, 2, 2, 5, 8, 14, 12, 7, 14, 13, 6];

        let mut prev_z = &HashMap::from([(0, Vec::<(i64, i64)>::new())]);
        let mut all_results = Vec::new();

        for ((a, b), c) in a_vals.iter().copied().zip(b_vals).zip(c_vals) {
            let mut results = HashMap::new();

            for &z in prev_z.keys() {
                for w in 1..=9 {
                    // Each program just runs this.
                    let cond = (z % 26) + b != w;

                    // In the 14 numbers, 7 have a a == 26, and 7 have a == 1.
                    // Each block with a == 1 adds a digit to z, each block a == 26 should remove a digit from z.
                    if a == 1 && !cond || a == 26 && cond {
                        continue;
                    }

                    let comp = if cond { (z / a) * 26 + w + c } else { z / a };
                    results.entry(comp).or_insert_with(Vec::new).push((z, w));
                }
            }
            all_results.push(results);
            prev_z = all_results.last().unwrap();
        }

        type DigitFunc = fn(&Vec<(i64, i64)>) -> Option<&(i64, i64)>;

        let extract_digit = |f: DigitFunc| {
            let mut z_key = 0;
            let mut num = 0;
            for (i, res) in all_results.iter().rev().enumerate() {
                let (z, digit) = res.get(&z_key).and_then(f).copied().unwrap();

                z_key = z;
                num += digit * 10i64.pow(i as u32);
            }
            num
        };

        let p1 = extract_digit(|x| x.iter().max_by_key(|(_, w)| w));
        let p2 = extract_digit(|x| x.iter().min_by_key(|(_, w)| w));

        (p1, p2)
    }
}

#[test]
fn test() {
    let (p1, p2) = Solution::calc("");
    assert_eq!(p1, 96918996924991);
    assert_eq!(p2, 91811241911641);
}
