use aoc_lib::{Execute, SolutionExec};

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

pub fn get_executions() -> [SolutionExec; 25] {
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
        day10::Solution::get_exec(),
        day11::Solution::get_exec(),
        day12::Solution::get_exec(),
        day13::Solution::get_exec(),
        day14::Solution::get_exec(),
        day15::Solution::get_exec(),
        day16::Solution::get_exec(),
        day17::Solution::get_exec(),
        day18::Solution::get_exec(),
        day19::Solution::get_exec(),
        day20::Solution::get_exec(),
        day21::Solution::get_exec(),
        day22::Solution::get_exec(),
        day23::Solution::get_exec(),
        day24::Solution::get_exec(),
        day25::Solution::get_exec(),
    ]
}
