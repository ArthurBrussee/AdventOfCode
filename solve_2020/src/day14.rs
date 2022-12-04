use aoc_lib::AocSolution;

pub struct Solution;

use std::collections::HashMap;

fn parse_addr(left: &str, right: &str) -> (usize, u64) {
    let addr: usize = left
        .strip_prefix("mem[")
        .unwrap()
        .replace(']', "")
        .parse()
        .unwrap();
    let val: u64 = right.parse().unwrap();
    (addr, val)
}

impl AocSolution<u64, u64> for Solution {
    const YEAR: u32 = 2020;
    const DAY: u32 = 14;

    fn calc(input: &str) -> (u64, u64) {
        let mut mems: HashMap<usize, u64> = HashMap::new();
        let mut mask_or = 0;
        let mut mask_and = 0;

        for line in input.lines() {
            let parts = line.split(" = ").collect::<Vec<_>>();
            match parts[0] {
                "mask" => {
                    let bin_str_or = &parts[1].replace('X', "0");
                    let bin_str_and = &parts[1].replace('X', "1");
                    mask_or = u64::from_str_radix(bin_str_or, 2).unwrap();
                    mask_and = u64::from_str_radix(bin_str_and, 2).unwrap();
                }
                _ => {
                    let (addr, val) = parse_addr(parts[0], parts[1]);
                    mems.insert(addr, (val | mask_or) & mask_and);
                }
            }
        }

        let p1 = mems.values().sum();

        enum Bit {
            Zero,
            One,
            Floating,
        }

        let mut mems: HashMap<usize, u64> = HashMap::new();
        let mut bitmasks: Vec<usize> = vec![];
        let mut mask_and = 0;

        for line in input.lines() {
            let parts = line.split(" = ").collect::<Vec<_>>();

            match parts[0] {
                "mask" => {
                    mask_and = 0;

                    let write = parts[1]
                        .chars()
                        .enumerate()
                        .map(|(i, s)| match s {
                            '0' => {
                                mask_and |= 1 << (35 - i);
                                Bit::Zero
                            }
                            '1' => Bit::One,
                            'X' => Bit::Floating,
                            _ => unreachable!("Oh crap."),
                        })
                        .collect::<Vec<_>>();

                    let start_mask = write.iter().rev().enumerate().fold(0, |acc, (i, bit)| {
                        let bit = usize::from(matches!(bit, Bit::One));
                        acc | (bit << i)
                    });

                    bitmasks.clear();
                    bitmasks.push(start_mask);

                    for (i, _) in write
                        .iter()
                        .rev()
                        .enumerate()
                        .filter(|(_, bit_list)| matches!(bit_list, Bit::Floating))
                    {
                        bitmasks.extend(bitmasks.clone().iter().map(|r| r | (1 << i)))
                    }
                }
                _ => {
                    let (addr, val) = parse_addr(parts[0], parts[1]);

                    for mask in &bitmasks {
                        mems.insert(addr & mask_and | mask, val);
                    }
                }
            }
        }

        let p2 = mems.values().sum();
        (p1, p2)
    }
}
