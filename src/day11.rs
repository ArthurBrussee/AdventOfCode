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
            Some(&self.tiles[self.index(x, y)])
        } else {
            None
        }
    }

    fn index(&self, x: i32, y: i32) -> usize {
        return (x + y * self.width) as usize;
    }
}

fn simulate_to_equal(board: &Board, overcrowd: usize, cast: bool) -> usize {
    let mut seats = Vec::new();

    for (i, tile) in board.tiles.iter().enumerate() {
        if tile == &Tile::Floor {
            continue;
        }
        let x = (i as i32) % board.width;
        let y = (i as i32) / board.width;

        let mut neighbours: Vec<u32> = Vec::new();

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                if !cast {
                    if matches!(
                        board.get_tile(x + dx, y + dy),
                        Some(Tile::EmptySeat) | Some(Tile::Occupied)
                    ) {
                        neighbours.push(board.index(x + dx, y + dy) as u32);
                    }
                } else {
                    let mut l = 1;
                    loop {
                        let xx = x + dx * l;
                        let yy = y + dy * l;
                        match board.get_tile(xx, yy) {
                            Some(Tile::Occupied) | Some(Tile::EmptySeat) => {
                                neighbours.push(board.index(xx, yy) as u32);
                                break;
                            }
                            Some(Tile::Floor) => {}
                            None => break,
                        }
                        l += 1;
                    }
                }
            }
        }
        seats.push((i as u32, neighbours));
    }

    let mut board = board.clone();
    let mut new_board = board.clone();
    loop {
        let mut change = false;
        for (i, neighbours) in &seats {
            let tile = board.tiles[*i as usize];

            let new_board_tile = &mut new_board.tiles[*i as usize];

            if tile == Tile::EmptySeat
                && !neighbours
                    .iter()
                    .any(|&n| board.tiles[n as usize] == Tile::Occupied)
            {
                *new_board_tile = Tile::Occupied;
                change = true;
            } else if tile == Tile::Occupied
                && neighbours.len() >= overcrowd
                && neighbours
                    .iter()
                    .filter(|&n| board.tiles[*n as usize] == Tile::Occupied)
                    .count()
                    >= overcrowd
            {
                *new_board_tile = Tile::EmptySeat;
                change = true;
            } else {
                *new_board_tile = tile;
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
