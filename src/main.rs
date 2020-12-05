use std::fmt::Display;
use std::time::Instant;

mod day1;
mod day2;
mod day3;
mod day4;

fn print_solution(day: impl Display, solution: (impl Display, impl Display), start: Instant) {
    let ms = start.elapsed().as_secs_f64() * 1000.0;
    println!("Day {}, {:.2}ms: ({}, {})", day, ms, solution.0, solution.1);
}

fn main() {
    println!("AOC 2020!");

    let start_run = Instant::now();

    let now = Instant::now();
    print_solution(1, day1::calc(), now);

    let now = Instant::now();
    print_solution(2, day2::calc(), now);

    let now = Instant::now();
    print_solution(3, day3::calc(), now);

    let now = Instant::now();
    print_solution(4, day4::calc(), now);

    println!(
        "Done with AOC! Took {:.2}ms",
        start_run.elapsed().as_secs_f64() * 1000.0
    );
}
