use std::collections::{HashMap, HashSet};

type EnergyGrid = HashMap<(i32, i32, i32, i32), bool>;

fn simulate(cubes: &EnergyGrid, w_range: i32) -> usize {
    let mut cur_cubes = cubes.clone();

    for _ in 0..6 {
        let mut check_keys = HashSet::new();
        let mut commands: Vec<((i32, i32, i32, i32), bool)> = Vec::new();

        for (x, y, z, w) in cur_cubes
            .iter()
            .filter_map(|(k, &v)| if v { Some(k) } else { None })
        {
            for dw in -w_range..=w_range {
                for dz in -1..=1 {
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            check_keys.insert((x + dx, y + dy, z + dz, w + dw));
                        }
                    }
                }
            }
        }

        for (x, y, z, w) in check_keys {
            let mut active_neighbour_count = 0;

            for dw in -w_range..=w_range {
                for dz in -1..=1 {
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
                                continue;
                            }
                            if let Some(true) = cur_cubes.get(&(x + dx, y + dy, z + dz, w + dw)) {
                                active_neighbour_count += 1;
                            }
                        }
                    }
                }
            }

            let index = (x, y, z, w);

            let &cube = cur_cubes.get(&index).unwrap_or(&false);
            if cube && !matches!(active_neighbour_count, 2..=3) {
                commands.push((index, false));
            } else if !cube && active_neighbour_count == 3 {
                commands.push((index, true));
            }
        }

        for (index, cube) in commands.drain(0..) {
            cur_cubes.insert(index, cube);
        }
    }

    cur_cubes.values().filter(|&&x| x).count()
}

pub fn calc(_: &str) -> (usize, usize) {
    let board = "###..#..
.#######
#####...
#..##.#.
###..##.
##...#..
..#...#.
.#....##";

    let cubes: EnergyGrid = board
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes().enumerate().map(move |(x, c)| {
                let cube = match c {
                    b'.' => false,
                    b'#' => true,
                    _ => unreachable!(),
                };
                ((x as i32 - 1, y as i32 - 1, 0, 0), cube)
            })
        })
        .collect();

    let p1 = simulate(&cubes, 0);
    let p2 = simulate(&cubes, 1);

    (p1, p2)
}
