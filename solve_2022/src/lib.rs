use aoc_lib::run_solution;

mod day1;

pub fn run() {
    aoc_lib::run_solutions(&[|| run_solution(2022, 1, day1::calc)]);
}
