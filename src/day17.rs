use std::collections::{HashMap, HashSet};

#[derive(Clone, PartialEq, Copy)]
enum Cube {
    Inactive,
    Active,
}

type EnergyGrid = HashMap<(i32, i32, i32, i32), Cube>;

fn simulate(cubes: &EnergyGrid, w_range: i32) -> usize {
    let mut cur_cubes = cubes.clone();

    for _ in 0..6 {
        let mut check_keys = HashSet::new();
        let mut commands: Vec<((i32, i32, i32, i32), Cube)> = Vec::new();

        for (x, y, z, w) in
            cur_cubes
                .iter()
                .filter_map(|(k, v)| if v == &Cube::Active { Some(k) } else { None })
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
                            if let Some(Cube::Active) =
                                cur_cubes.get(&(x + dx, y + dy, z + dz, w + dw))
                            {
                                active_neighbour_count += 1;
                            }
                        }
                    }
                }
            }

            let index = (x, y, z, w);

            let cube = cur_cubes.get(&index).unwrap_or(&Cube::Inactive);
            if cube == &Cube::Active && !matches!(active_neighbour_count, 2..=3) {
                commands.push((index, Cube::Inactive));
            } else if cube == &Cube::Inactive && active_neighbour_count == 3 {
                commands.push((index, Cube::Active));
            }
        }

        for (index, cube) in commands.drain(0..) {
            cur_cubes.insert(index, cube);
        }
    }

    cur_cubes.values().filter(|&&x| x == Cube::Active).count()
}

pub fn calc() -> (usize, usize) {
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
                    b'.' => Cube::Inactive,
                    b'#' => Cube::Active,
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
