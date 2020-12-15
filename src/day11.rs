use std::fs;

#[derive(Clone, PartialEq, Copy)]
enum Tile {
    EmptySeat,
    Occupied,
    Floor,
}

#[derive(Clone)]
struct Board {
    tiles: Vec<Tile>,
    width: i32,
    height: i32,
}

impl Board {
    fn new(str: &String) -> Board {
        Board {
            tiles: str
                .chars()
                .filter_map(|c| match c {
                    'L' => Some(Tile::EmptySeat),
                    '#' => Some(Tile::Occupied),
                    '.' => Some(Tile::Floor),
                    _ => None,
                })
                .collect::<Vec<_>>(),
            width: str.lines().next().unwrap().len() as i32,
            height: str.lines().count() as i32,
        }
    }

    fn get_tile(&self, x: i32, y: i32) -> Option<&Tile> {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            Some(&self.tiles[(x + y * self.width) as usize])
        } else {
            None
        }
    }
}

fn simulate_to_equal(board: &Board, overcrowd: usize, cast: bool) -> usize {
    let mut board = board.clone();
    let mut new_board = board.clone();

    loop {
        let mut change = false;
        for (i, tile) in board.tiles.iter().enumerate() {
            let i = i as i32;
            if tile == &Tile::Floor {
                continue;
            }
            let x = i % board.width;
            let y = i / board.width;

            let mut count = 0;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let occ_neighbour = if !cast {
                        board.get_tile(x + dx, y + dy) == Some(&Tile::Occupied)
                    } else {
                        let mut l = 1;
                        loop {
                            match board.get_tile(x + dx * l, y + dy * l) {
                                Some(Tile::Occupied) => break true,
                                Some(Tile::EmptySeat) | None => break false,
                                Some(_) => {}
                            }
                            l += 1;
                        }
                    };
                    if occ_neighbour {
                        count += 1;
                    }
                }
            }

            match (tile, count) {
                (Tile::EmptySeat, 0) => {
                    new_board.tiles[i as usize] = Tile::Occupied;
                    change = true;
                }
                (Tile::Occupied, c) if c >= overcrowd => {
                    new_board.tiles[i as usize] = Tile::EmptySeat;
                    change = true;
                }
                _ => new_board.tiles[i as usize] = *tile,
            }
        }
        if !change {
            break board.tiles.iter().filter(|&x| x == &Tile::Occupied).count();
        }
        std::mem::swap(&mut board, &mut new_board);
    }
}

pub fn calc() -> (usize, usize) {
    let board =
        Board::new(&fs::read_to_string("./inputs/day11.txt").expect("Can't find input file."));
    let p1 = simulate_to_equal(&board, 4, false);
    let p2 = simulate_to_equal(&board, 5, true);
    (p1, p2)
}
