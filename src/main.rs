use std::env;
use std::fmt::Display;
use std::time::Instant;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn run_solution<T, V>(day: u32, calc: fn() -> (T, V))
where
    T: Display,
    V: Display,
{
    let now = Instant::now();
    let (p1, p2) = calc();
    let ms = now.elapsed().as_secs_f64() * 1000.0;
    println!("Day {}, {:.2}ms: ({}, {})", day, ms, p1, p2);
}

fn main() {
    println!("AOC 2020!");

    let mut days = env::args()
        .filter_map(|arg| arg.strip_prefix("--day").map(|f| f.to_string()))
        .map(|r| r.parse::<usize>().unwrap() - 1)
        .collect::<Vec<_>>();

    if days.len() == 0 {
        days.extend(0..=25);
    }

    let start_run = Instant::now();

    let funcs = [
        || run_solution(1, day1::calc),
        || run_solution(2, day2::calc),
        || run_solution(3, day3::calc),
        || run_solution(4, day4::calc),
        || run_solution(5, day5::calc),
        || run_solution(6, day6::calc),
        || run_solution(7, day7::calc),
        || run_solution(8, day8::calc),
        || run_solution(9, day9::calc),
        || run_solution(10, day10::calc),
        || run_solution(11, day11::calc),
        || run_solution(12, day12::calc),
        || run_solution(13, day13::calc),
        || run_solution(14, day14::calc),
        || run_solution(15, day15::calc),
        || run_solution(16, day16::calc),
        || run_solution(17, day17::calc),
        || run_solution(18, day18::calc),
        || run_solution(19, day19::calc),
        || run_solution(20, day20::calc),
        || run_solution(21, day21::calc),
    ];

    for d in days {
        if let Some(f) = funcs.get(d) {
            f();
        }
    }

    println!(
        "Done with AOC! Took {:.2}ms",
        start_run.elapsed().as_secs_f64() * 1000.0
    );
}
