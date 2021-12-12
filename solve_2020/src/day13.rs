fn modinv(time: u64, freq: u64) -> u64 {
    (freq - time % freq) % freq
}

pub fn calc(input: &str) -> (u64, u64) {
    let mut lines = input.lines();
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
