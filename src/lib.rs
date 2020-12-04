use std::fs::File;
use std::io::{self, BufRead};

pub fn file_lines(filename: &str) -> impl Iterator<Item = String> {
    let file = File::open(filename).expect("Can't find input file.");
    io::BufReader::new(file).lines().map(|x| x.unwrap())
}
