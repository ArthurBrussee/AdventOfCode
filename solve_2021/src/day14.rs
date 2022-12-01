use std::collections::HashMap;

struct Polymer {
    pair_counts: HashMap<[char; 2], usize>,
    rules: HashMap<[char; 2], char>,
    last_char: char,
}

impl Polymer {
    fn step(&mut self) {
        let mut new_counts = self.pair_counts.clone();

        for (&pair, &count) in &self.pair_counts {
            if let Some(&insertion) = self.rules.get(&pair) {
                *new_counts.get_mut(&pair).unwrap() -= count;
                *new_counts.entry([pair[0], insertion]).or_default() += count;
                *new_counts.entry([insertion, pair[1]]).or_default() += count;
            }
        }

        self.pair_counts = new_counts;
    }

    fn score(&self) -> usize {
        let mut char_counts: HashMap<char, usize> = HashMap::new();
        *char_counts.entry(self.last_char).or_default() += 1;
        for (&[c1, _], &n) in &self.pair_counts {
            *char_counts.entry(c1).or_default() += n;
        }

        char_counts.values().max().unwrap() - char_counts.values().min().unwrap()
    }
}

pub fn calc(input: &str) -> (usize, usize) {
    let mut lines = input.lines();
    let start_templ: Vec<char> = lines.next().unwrap().chars().collect();

    lines.next();

    let mut rules = HashMap::new();
    for l in lines {
        let (template, replace) = l.split_once(" -> ").unwrap();
        let mut templ_chars = template.chars();
        rules.insert(
            [templ_chars.next().unwrap(), templ_chars.next().unwrap()],
            replace.chars().next().unwrap(),
        );
    }

    let mut pair_counts = HashMap::new();
    for window in start_templ.windows(2) {
        let window_arr = window.try_into().unwrap();
        *pair_counts.entry(window_arr).or_default() += 1;
    }

    let mut poly = Polymer {
        pair_counts,
        rules,
        last_char: *start_templ.last().unwrap(),
    };

    for _ in 0..10 {
        poly.step();
    }

    let p1 = poly.score();

    for _ in 10..40 {
        poly.step();
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
