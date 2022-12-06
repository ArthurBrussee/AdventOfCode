use aoc_lib::{Execute, SolutionExec};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

pub fn get_executions() -> [SolutionExec; 6] {
    [
        day1::Solution::get_exec(),
        day2::Solution::get_exec(),
        day3::Solution::get_exec(),
        day4::Solution::get_exec(),
        day5::Solution::get_exec(),
        day6::Solution::get_exec(),
    ]
}
