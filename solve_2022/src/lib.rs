use aoc_lib::{Execute, SolutionExec};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub fn get_executions() -> [SolutionExec; 9] {
    [
        day1::Solution::get_exec(),
        day2::Solution::get_exec(),
        day3::Solution::get_exec(),
        day4::Solution::get_exec(),
        day5::Solution::get_exec(),
        day6::Solution::get_exec(),
        day7::Solution::get_exec(),
        day8::Solution::get_exec(),
        day9::Solution::get_exec(),
    ]
}
