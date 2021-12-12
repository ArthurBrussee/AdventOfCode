pub fn calc(input: &str) -> (usize, usize) {}

#[test]
fn test() {
    let (p1, p2) = calc(&aoc_lib::read_file(2021, 1, true));
    assert_eq!(p1, 1);
    assert_eq!(p2, 1);
}
