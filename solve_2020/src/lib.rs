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
        || run_solution(22, day22::calc),
        || run_solution(23, day23::calc),
        || run_solution(24, day24::calc),
        || run_solution(25, day25::calc),
    ];

    for f in aoc_lib::get_days_to_run().filter_map(|d| funcs.get(d)) {
        f()
    }

    println!(
        "Done with AOC! Took {:.2}ms",
        start_run.elapsed().as_secs_f64() * 1000.0
    );
}
