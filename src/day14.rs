use std::collections::HashMap;
use std::fs;

fn parse_addr(left: &str, right: &str) -> (usize, u64) {
    let addr: usize = left
        .strip_prefix("mem[")
        .unwrap()
        .replace("]", "")
        .parse()
        .unwrap();
    let val: u64 = right.parse().unwrap();
    (addr, val)
}

pub fn calc() -> (u64, u64) {
    let file = fs::read_to_string("./inputs/day14.txt").unwrap();

    let mut mems: HashMap<usize, u64> = HashMap::new();
    let mut mask_or = 0;
    let mut mask_and = 0;

    for line in file.lines() {
        let parts = line.split(" = ").collect::<Vec<_>>();
        match parts[0] {
            "mask" => {
                let bin_str_or = &parts[1].replace("X", "0");
                let bin_str_and = &parts[1].replace("X", "1");
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

    let mut cur_mask: String = String::new();

    for line in file.lines() {
        let parts = line.split(" = ").collect::<Vec<_>>();

        match parts[0] {
            "mask" => {
                cur_mask = parts[1].to_string();
            }
            _ => {
                let (addr, val) = parse_addr(parts[0], parts[1]);
                let addr_bits = (0..36)
                    .rev()
                    .map(|i| if addr & (1 << i) == 0 { 0 } else { 1 });

                let write = addr_bits
                    .zip(cur_mask.chars())
                    .map(|s| match s {
                        (0, '0') => Bit::Zero,
                        (1, '0') => Bit::One,
                        (_, '1') => Bit::One,
                        (_, 'X') => Bit::Floating,
                        _ => unreachable!("Oh crap. {} {}", s.0, s.1),
                    })
                    .collect::<Vec<_>>();

                let mut adresses: Vec<usize> = vec![];

                let mut potentials_iter = write.iter().rev().enumerate();
                match potentials_iter.next().unwrap().1 {
                    Bit::Zero => adresses.push(0),
                    Bit::One => adresses.push(1),
                    Bit::Floating => {
                        adresses.push(0);
                        adresses.push(1);
                    }
                }
                for (i, bit_list) in potentials_iter {
                    let mask = 1 << i;

                    match bit_list {
                        Bit::Zero => {}
                        Bit::One => {
                            for addr in adresses.iter_mut() {
                                *addr |= mask;
                            }
                        }
                        Bit::Floating => {
                            adresses.append(&mut adresses.iter().map(|r| r | mask).collect())
                        }
                    }
                }

                for addr in adresses {
                    mems.insert(addr as usize, val);
                }
            }
        }
    }

    let p2 = mems.values().sum();
    (p1, p2)
}
