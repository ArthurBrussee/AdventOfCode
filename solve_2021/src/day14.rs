use std::collections::HashMap;
use std::collections::LinkedList;

struct Polymer {
    state: LinkedList<u8>,
    rules: HashMap<(u8, u8), u8>,
}

impl Polymer {
    fn step(&mut self) {
        let len = self.state.len();
        let mut cursor = self.state.cursor_front_mut();

        for _ in 0..len - 1 {
            let cur = *cursor.current().unwrap();
            let next = *cursor.peek_next().unwrap();
            cursor.move_next();

            if let Some(&replace) = self.rules.get(&(cur, next)) {
                cursor.insert_before(replace);
            }
        }
    }

    fn score(&self) -> usize {
        let mut counts = HashMap::new();
        for c in &self.state {
            *counts.entry(c).or_insert(0) += 1;
        }
        counts.values().max().unwrap() - counts.values().min().unwrap()
    }
}

pub fn calc(input: &str) -> (usize, usize) {
    let mut lines = input.lines();

    let state = lines.next().unwrap().chars().map(|c| c as u8).collect();

    lines.next();

    let mut rules = HashMap::new();

    for l in lines {
        let (template, result) = l.split_once(" -> ").unwrap();
        let mut chars = template.chars();
        rules.insert(
            (chars.next().unwrap() as u8, chars.next().unwrap() as u8),
            result.chars().next().unwrap() as u8,
        );
    }

    let mut poly = Polymer { state, rules };

    for _ in 0..10 {
        poly.step();

        let mut counts = HashMap::new();
        for c in &poly.state {
            *counts.entry(c).or_insert(0) += 1;
        }
        let max = counts.values().max().unwrap();
        let max_char = counts.iter().max_by_key(|(c, ct)| *ct).unwrap().0;

        println!("{} {}", max, max_char);
    }

    let p1 = poly.score();

    for i in 10..20 {
        poly.step();
        println!("Did step {}", i);
    }

    let p2 = poly.score();

    (p1, p2)
}

#[test]
fn test() {
    let (p1, p2) = calc(&aoc_lib::read_file(2021, 14, true));
    assert_eq!(p1, 1588);
    assert_eq!(p2, 2188189693529);
}
