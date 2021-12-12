use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy)]
enum Command {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

fn offset(pos: (i32, i32), command: Command) -> (i32, i32) {
    match command {
        Command::East => (pos.0 + 1, pos.1),
        Command::SouthEast => (pos.0 + 1, pos.1 - 1),
        Command::SouthWest => (pos.0, pos.1 - 1),
        Command::West => (pos.0 - 1, pos.1),
        Command::NorthWest => (pos.0 - 1, pos.1 + 1),
        Command::NorthEast => (pos.0, pos.1 + 1),
    }
}

pub fn calc(input: &str) -> (usize, usize) {
    let instructions = input
        .lines()
        .map(|l| {
            let mut ret = Vec::new();
            let mut left = l;
            while !left.is_empty() {
                if let Some(pre) = left.strip_prefix('e') {
                    ret.push(Command::East);
                    left = pre;
                }
                if let Some(pre) = left.strip_prefix("se") {
                    ret.push(Command::SouthEast);
                    left = pre;
                }
                if let Some(pre) = left.strip_prefix("sw") {
                    ret.push(Command::SouthWest);
                    left = pre;
                }
                if let Some(pre) = left.strip_prefix('w') {
                    ret.push(Command::West);
                    left = pre;
                }
                if let Some(pre) = left.strip_prefix("nw") {
                    ret.push(Command::NorthWest);
                    left = pre;
                }
                if let Some(pre) = left.strip_prefix("ne") {
                    ret.push(Command::NorthEast);
                    left = pre;
                }
            }
            ret
        })
        .collect::<Vec<Vec<_>>>();

    let mut tiles: HashMap<(i32, i32), bool> = HashMap::new();

    for instruction in instructions {
        let pos = instruction
            .iter()
            .fold((0, 0), |pos, &command| offset(pos, command));
        tiles.insert(pos, !tiles.get(&pos).unwrap_or(&true));
    }

    let p1 = tiles.values().filter(|&&tile| !tile).count();

    for _ in 0..100 {
        let cur_tiles = tiles.clone();

        let count_black = |pos: (i32, i32)| {
            if let Some(false) = cur_tiles.get(&(pos.0, pos.1)) {
                1
            } else {
                0
            }
        };

        let check_keys = cur_tiles
            .keys()
            .flat_map(|&pos| {
                vec![
                    offset(pos, Command::East),
                    offset(pos, Command::SouthEast),
                    offset(pos, Command::SouthWest),
                    offset(pos, Command::West),
                    offset(pos, Command::NorthWest),
                    offset(pos, Command::NorthEast),
                    pos,
                ]
            })
            .collect::<HashSet<_>>();

        for pos in check_keys {
            let neighbours = count_black(offset(pos, Command::East))
                + count_black(offset(pos, Command::SouthEast))
                + count_black(offset(pos, Command::SouthWest))
                + count_black(offset(pos, Command::West))
                + count_black(offset(pos, Command::NorthWest))
                + count_black(offset(pos, Command::NorthEast));

            if *cur_tiles.get(&pos).unwrap_or(&true) {
                if neighbours == 2 {
                    tiles.insert(pos, false);
                }
            } else if neighbours == 0 || neighbours > 2 {
                tiles.insert(pos, true);
            }
        }
    }

    let p2 = tiles.values().filter(|&&tile| !tile).count();
    (p1, p2)
}
