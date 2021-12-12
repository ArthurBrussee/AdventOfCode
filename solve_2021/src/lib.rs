#![feature(destructuring_assignment)]
#![feature(bool_to_option)]

use std::time::Instant;

use aoc_lib::run_solution;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub fn run() {
    println!("AOC 2021!");

    let start_run = Instant::now();

    let funcs = [
        || run_solution(2021, 1, day1::calc),
        || run_solution(2021, 2, day2::calc),
        || run_solution(2021, 3, day3::calc),
        || run_solution(2021, 4, day4::calc),
        || run_solution(2021, 5, day5::calc),
        || run_solution(2021, 6, day6::calc),
        || run_solution(2021, 7, day7::calc),
        || run_solution(2021, 8, day8::calc),
        || run_solution(2021, 9, day9::calc),
    ];

    for f in aoc_lib::get_days_to_run().filter_map(|d| funcs.get(d)) {
        f()
    }

    println!(
        "Done with AOC! Took {:.2}ms",
        start_run.elapsed().as_secs_f64() * 1000.0
    );
}
