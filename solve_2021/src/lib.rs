#![feature(destructuring_assignment)]
#![feature(label_break_value)]

use std::time::Instant;

use aoc_lib::run_solution;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

pub fn run() {
    println!("AOC 2021!");

    let start_run = Instant::now();

    let funcs = [
        || run_solution(1, day1::calc),
        || run_solution(2, day2::calc),
        || run_solution(3, day3::calc),
        || run_solution(4, day4::calc),
        || run_solution(5, day5::calc),
    ];

    for f in aoc_lib::get_days_to_run().filter_map(|d| funcs.get(d)) {
        f()
    }

    println!(
        "Done with AOC! Took {:.2}ms",
        start_run.elapsed().as_secs_f64() * 1000.0
    );
}
