use aoc_lib::run_solution;

mod day1;
mod day2;
mod day3;

pub fn run() {
    let funcs = [
        || run_solution(2022, 1, day1::calc),
        || run_solution(2022, 2, day2::calc),
        || run_solution(2022, 3, day3::calc),
    ];

    aoc_lib::run_solutions(&funcs);
}
