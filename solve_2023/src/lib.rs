use aoc_lib::{Execute, SolutionExec};

mod day1;

pub fn get_executions() -> Vec<SolutionExec> {
    vec![
        day1::Solution::get_exec(),
    ]
}
