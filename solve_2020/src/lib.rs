use std::time::Instant;

use aoc_lib::run_solution;

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
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub fn run() {
    println!("AOC 2020!");

    let start_run = Instant::now();

    let funcs = [
        || run_solution(2020, 1, day1::calc),
        || run_solution(2020, 2, day2::calc),
        || run_solution(2020, 3, day3::calc),
        || run_solution(2020, 4, day4::calc),
        || run_solution(2020, 5, day5::calc),
        || run_solution(2020, 6, day6::calc),
        || run_solution(2020, 7, day7::calc),
        || run_solution(2020, 8, day8::calc),
        || run_solution(2020, 9, day9::calc),
        || run_solution(2020, 10, day10::calc),
        || run_solution(2020, 11, day11::calc),
        || run_solution(2020, 12, day12::calc),
        || run_solution(2020, 13, day13::calc),
        || run_solution(2020, 14, day14::calc),
        || run_solution(2020, 15, day15::calc),
        || run_solution(2020, 16, day16::calc),
        || run_solution(2020, 17, day17::calc),
        || run_solution(2020, 18, day18::calc),
        || run_solution(2020, 19, day19::calc),
        || run_solution(2020, 20, day20::calc),
        || run_solution(2020, 21, day21::calc),
        || run_solution(2020, 22, day22::calc),
        || run_solution(2020, 23, day23::calc),
        || run_solution(2020, 24, day24::calc),
        || run_solution(2020, 25, day25::calc),
    ];

    for f in aoc_lib::get_days_to_run().filter_map(|d| funcs.get(d)) {
        f()
    }

    println!(
        "Done with AOC! Took {:.2}ms",
        start_run.elapsed().as_secs_f64() * 1000.0
    );
}
