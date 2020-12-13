use std::fs;

fn modinv(time: u64, freq: u64) -> u64 {
    (freq - time % freq) % freq
}

pub fn calc() -> (u64, u64) {
    let file = fs::read_to_string("./inputs/day13.txt").unwrap();
    let mut lines = file.lines();
    let earliest = lines.next().and_then(|x| x.parse().ok()).unwrap();
    let busses = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(id, x)| Some((id as u64, x.parse().ok()?)))
        .collect::<Vec<_>>();
    let (_, best_bus) = busses
        .iter()
        .min_by_key(|(_, n)| modinv(earliest, *n))
        .unwrap();

    let remainder = modinv(earliest, *best_bus);

    let mut offset = 0;
    let mut modulus = 1;
    for bus in &busses {
        loop {
            let rem = modinv(offset, bus.1);
            if rem == (bus.0 % bus.1) {
                break;
            }
            offset += modulus;
        }
        modulus *= bus.1;
    }
    (remainder * best_bus, offset)
}
