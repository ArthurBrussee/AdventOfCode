use std::{env, fmt::Display, fs, str::FromStr, time::Instant};

pub fn run_solution<T, V>(day: u32, calc: fn() -> (T, V))
where
    T: Display,
    V: Display,
{
    let now = Instant::now();
    let (p1, p2) = calc();
    let ms = now.elapsed().as_secs_f64() * 1000.0;
    println!("Day {}, {:.2}ms: ({}, {})", day, ms, p1, p2);
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

pub fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("Failed to find file!")
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

pub fn parse_file_lines<T>(path: &str) -> Vec<T>
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    parse_lines(&read_file(path))
}

pub fn parse_lines<T>(str: &str) -> Vec<T>
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    map_lines(str, |it| it.parse().expect("Failed to parse!"))
}
