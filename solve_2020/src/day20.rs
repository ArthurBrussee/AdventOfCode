use std::cmp::max;
use std::collections::HashMap;

#[derive(Clone, Copy)]
enum Transform {
    Identity,
    FlipX,
    FlipY,
    FlipXY,
    Rotate90,
}
const FLIPS: [Transform; 4] = [
    Transform::Identity,
    Transform::FlipX,
    Transform::FlipY,
    Transform::FlipXY,
];

const ROTATIONS: [Transform; 2] = [Transform::Identity, Transform::Rotate90];

#[derive(Clone, Copy)]
enum Side {
    Right,
    Up,
    Left,
    Down,
}
const SIDES: [Side; 4] = [Side::Up, Side::Left, Side::Right, Side::Down];

impl Side {
    fn dir(&self) -> (i32, i32) {
        match self {
            Side::Right => (1, 0),
            Side::Up => (0, -1),
            Side::Left => (-1, 0),
            Side::Down => (0, 1),
        }
    }
}

#[derive(Clone)]
struct Tile {
    id: u64,
    size: i32,
    pixels: Vec<bool>,
}

impl Tile {
    fn from_str(id: u64, s: &str) -> Tile {
        Tile {
            id,
            size: s.lines().next().unwrap().len() as i32,
            pixels: s
                .lines()
                .flat_map(|line| line.chars().map(|c| c == '#'))
                .collect::<Vec<_>>(),
        }
    }

    fn transform(&self, trans: Transform) -> Tile {
        Tile {
            id: self.id,
            size: self.size,
            pixels: (0..self.pixels.len())
                .map(|i| {
                    let x = i as i32 % self.size;
                    let y = i as i32 / self.size;
                    let (xp, yp) = match trans {
                        Transform::Identity => (x, y),
                        Transform::FlipX => (self.size - 1 - x, y),
                        Transform::FlipY => (x, self.size - 1 - y),
                        Transform::FlipXY => (self.size - 1 - x, self.size - 1 - y),
                        Transform::Rotate90 => (y, self.size - 1 - x),
                    };
                    self.get(xp, yp)
                })
                .collect(),
        }
    }

    fn get(&self, x: i32, y: i32) -> bool {
        self.pixels[(x + y * self.size) as usize]
    }
}

fn fits(tile: &Tile, (x, y): (i32, i32), grid: &HashMap<(i32, i32), Tile>) -> bool {
    if grid.get(&(x, y)).is_some() {
        return false;
    }
    for &s in &SIDES {
        let dir = s.dir();
        if let Some(neighbour) = grid.get(&(x + dir.0, y + dir.1)) {
            if (0..tile.size).any(|i| match s {
                Side::Right => tile.get(tile.size - 1, i) != neighbour.get(0, i),
                Side::Up => tile.get(i, 0) != neighbour.get(i, neighbour.size - 1),
                Side::Left => tile.get(0, i) != neighbour.get(neighbour.size - 1, i),
                Side::Down => tile.get(i, tile.size - 1) != neighbour.get(i, 0),
            }) {
                return false;
            }
        }
    }
    true
}

pub fn calc(input: &str) -> (u64, usize) {
    let tiles = input
        .split("\n\n")
        .map(|tile| {
            let mut board_iter = tile.splitn(2, "\r\n");
            let id = board_iter
                .next()
                .and_then(|f| f.strip_prefix("Tile "))
                .and_then(|f| f.strip_suffix(':'))
                .and_then(|f| f.parse().ok())
                .unwrap();
            let rest_string = board_iter.next().unwrap();
            (id, Tile::from_str(id, rest_string))
        })
        .collect::<HashMap<_, _>>();

    let mut grid = HashMap::new();

    let mut remaining = tiles.keys().collect::<Vec<_>>();
    remaining.sort();

    grid.insert((0, 0), tiles[remaining[0]].clone());
    remaining.remove(0);

    for _ in 0..tiles.len() {
        'find_tile: for (i, id) in remaining.iter().enumerate() {
            for &f in &FLIPS {
                let tile_flipped = tiles[id].transform(f);
                for &r in &ROTATIONS {
                    let tilep = tile_flipped.transform(r);

                    for (x, y) in grid.keys() {
                        for &s in &SIDES {
                            let dir = s.dir();
                            let pos = (x + dir.0, y + dir.1);

                            if fits(&tilep, pos, &grid) {
                                grid.insert(pos, tilep);
                                remaining.remove(i);
                                break 'find_tile;
                            }
                        }
                    }
                }
            }
        }
    }

    let (min_x, min_y) = *grid.keys().min().unwrap();
    let (max_x, max_y) = *grid.keys().max().unwrap();

    let tile_size = tiles.values().next().unwrap().size - 2;
    let full_pic_size = (tiles.len() as f64).sqrt() as i32 * tile_size;

    let mut full_pixels = vec![false; (full_pic_size * full_pic_size) as usize];

    for ((bx, by), tile) in grid.iter() {
        for x in 0..tile_size {
            for y in 0..tile_size {
                let xx = x + (bx - min_x) * tile_size;
                let yy = y + (by - min_y) * tile_size;
                full_pixels[(xx + yy * full_pic_size) as usize] = tile.get(x + 1, y + 1);
            }
        }
    }

    let full_pic = Tile {
        id: 0,
        size: full_pic_size,
        pixels: full_pixels,
    };

    let monster_tile = Tile::from_str(
        0,
        "                  # 
#    ##    ##    ###
 #  #  #  #  #  #   ",
    );

    let mut monster_count = 0;

    for &f in &FLIPS {
        for &r in &ROTATIONS {
            let pic = full_pic.transform(f).transform(r);
            let mut monster_count_cur = 0;

            for x in 0..pic.size {
                for y in 0..pic.size {
                    let mut matches = true;

                    'monster_loop: for j in 0..3 {
                        for i in 0..monster_tile.size {
                            let xp = x + i;
                            let yp = y + j;

                            let monster_active = monster_tile.get(i, j);
                            let pic_active = xp < pic.size && yp < pic.size && pic.get(xp, yp);

                            if monster_active && !pic_active {
                                matches = false;
                                break 'monster_loop;
                            }
                        }
                    }

                    if matches {
                        monster_count_cur += 1;
                    }
                }
            }

            monster_count = max(monster_count, monster_count_cur);
        }
    }

    let total_count = full_pic.pixels.iter().filter(|b| **b).count()
        - monster_count * monster_tile.pixels.iter().filter(|b| **b).count();

    let corners = vec![
        grid[&(min_x, min_y)].id,
        grid[&(max_x, min_y)].id,
        grid[&(min_x, max_y)].id,
        grid[&(max_x, max_y)].id,
    ];
    (corners.iter().product(), total_count)
}
