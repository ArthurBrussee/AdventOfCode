use std::{env, fmt::Display, fs, str::FromStr, time::Instant};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub fn read_file(year: u32, day: u32, test: bool) -> String {
    let path = if test {
        format!("./inputs/day{}_test.txt", day)
    } else {
        format!("./solve_{}/inputs/day{}.txt", year, day)
    };
    let file = fs::read_to_string(&path).expect("Please include input file!");
    file.replace("\r\n", "\n")
}

pub fn run_solution<T, V>(year: u32, day: u32, calc: fn(&str) -> (T, V)) -> String
where
    T: Display,
    V: Display,
{
    let now = Instant::now();

    let (p1, p2) = calc(&read_file(year, day, false));
    let ms = now.elapsed().as_secs_f64() * 1000.0;
    format!("Day {}, {:.2}ms: ({}, {})", day, ms, p1, p2)
}

pub fn run_solutions(funcs: &[fn() -> String]) {
    let start_run = Instant::now();

    let funcs_to_run = get_days_to_run()
        .filter_map(|d| funcs.get(d))
        .collect::<Vec<_>>();

    let strings: Vec<_> = funcs_to_run.par_iter().map(|f| f()).collect();

    for s in strings {
        println!("{}", s);
    }

    println!(
        "Done with AOC! Took {:.2}ms",
        start_run.elapsed().as_secs_f64() * 1000.0
    );
}

pub fn get_days_to_run() -> impl Iterator<Item = usize> {
    let mut days = env::args()
        .filter_map(|arg| arg.strip_prefix("--day").map(|f| f.to_string()))
        .map(|r| r.parse::<usize>().unwrap() - 1)
        .collect::<Vec<_>>();

    if days.is_empty() {
        days.extend(0..25);
    }

    days.into_iter()
}

pub fn get_years_to_run() -> impl Iterator<Item = usize> {
    let mut years = env::args()
        .filter_map(|arg| arg.strip_prefix("--year").map(|f| f.to_string()))
        .map(|r| r.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    // Let's see if I can do this for the next 10 years haha
    if years.is_empty() {
        years.extend(2020..=2030);
    }

    years.into_iter()
}

pub fn map_file_lines<T>(path: &str, func: fn(&str) -> T) -> Vec<T> {
    map_lines(
        &fs::read_to_string(path).expect("Failed to find file!"),
        func,
    )
}

pub fn map_lines<T>(str: &str, func: fn(&str) -> T) -> Vec<T> {
    str.lines().map(func).collect()
}

pub fn parse_lines<T>(str: &str) -> Vec<T>
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    map_lines(str, |it| it.parse().expect("Failed to parse!"))
}

pub trait DoubleLineSplit {
    type Iterator;

    fn split_at_doubleblank(&self) -> Self::Iterator;
}

impl<'a> DoubleLineSplit for &'a str {
    type Iterator = std::str::Split<'a, &'a str>;

    fn split_at_doubleblank(&self) -> std::str::Split<'a, &'a str> {
        if self.contains("\r\n") {
            self.split("\r\n\r\n")
        } else {
            self.split("\n\n")
        }
    }
}
