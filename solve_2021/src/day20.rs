use std::collections::HashMap;

use aoc_lib::DoubleLineSplit;

struct Image {
    pixels: HashMap<(i32, i32), bool>,
}

impl Image {
    fn read_window(&self, x: i32, y: i32, bck: bool) -> u32 {
        let mut output = 0;
        let mut index = 8;

        for dy in -1..=1 {
            for dx in -1..=1 {
                let pixel = self.pixels.get(&(x + dx, y + dy));
                let val = *pixel.unwrap_or(&bck);
                if val {
                    output |= 1 << index;
                }
                index -= 1;
            }
        }

        output
    }

    fn convolve(&self, bin_map: &[bool], bck: bool) -> Image {
        let mut pixels = HashMap::with_capacity(self.pixels.len());

        for dy in -1..=1 {
            for dx in -1..=1 {
                for key in self.pixels.keys() {
                    let pos = (key.0 + dx, key.1 + dy);

                    pixels.entry(pos).or_insert_with(|| {
                        let window = self.read_window(pos.0, pos.1, bck);
                        bin_map[window as usize]
                    });
                }
            }
        }

        Image { pixels }
    }
}

pub fn calc(input: &str) -> (usize, usize) {
    let mut parts = input.split_at_doubleblank();

    let remaps: Vec<bool> = parts.next().unwrap().chars().map(|c| c == '#').collect();

    let image_str = parts.next().unwrap();
    let width = image_str.lines().next().unwrap().len();
    let pixels = image_str
        .lines()
        .flat_map(|l| l.chars())
        .enumerate()
        .map(|(i, c)| {
            let pos = ((i % width) as i32, (i / width) as i32);
            (pos, c == '#')
        })
        .collect();

    let mut image = Image { pixels };

    let mut bck = false;

    for _ in 0..2 {
        image = image.convolve(&remaps, bck);
        if remaps[0] {
            bck = !bck;
        }
    }

    let p1 = image.pixels.values().filter(|&&v| v).count();

    for _ in 2..50 {
        image = image.convolve(&remaps, bck);
        if remaps[0] {
            bck = !bck;
        }
    }

    let p2 = image.pixels.values().filter(|&&v| v).count();
    (p1, p2)
}

#[test]
fn test() {
    let (p1, p2) = calc(&aoc_lib::read_file(2021, 20, true));
    assert_eq!(p1, 35);
    assert_eq!(p2, 3351);
}
