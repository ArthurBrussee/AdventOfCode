use crate::lib;

struct PasswordProto {
    lower: usize,
    upper: usize,
    letter: char,
    password: String,
}

impl PasswordProto {
    fn new(line: String) -> Self {
        let dash_index = line.find('-').unwrap();
        let colon_index = line.find(':').unwrap();

        PasswordProto {
            lower: line[0..dash_index].parse().unwrap(),
            upper: line[dash_index + 1..colon_index - 2].parse().unwrap(),
            letter: line.chars().nth(colon_index - 1).unwrap(),
            password: line[(colon_index + 2)..].into(),
        }
    }
}

fn pass_p1(pass: &PasswordProto) -> bool {
    let count = pass.password.chars().filter(|&x| x == pass.letter).count();
    count >= pass.lower && count <= pass.upper
}

fn pass_p2(pass: &PasswordProto) -> bool {
    let valid1 = pass.password.chars().nth(pass.lower - 1).unwrap() == pass.letter;
    let valid2 = pass.password.chars().nth(pass.upper - 1).unwrap() == pass.letter;
    valid1 ^ valid2
}

pub fn part1() -> usize {
    lib::file_lines("./inputs/day2.txt")
        .map(PasswordProto::new)
        .filter(pass_p1)
        .count()
}

pub fn part2() -> usize {
    lib::file_lines("./inputs/day2.txt")
        .map(PasswordProto::new)
        .filter(pass_p2)
        .count()
}
