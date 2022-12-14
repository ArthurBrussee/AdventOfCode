use aoc_lib::SolutionExec;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{env, time::Instant};

fn main() {
    let days = env::args()
        .filter_map(|arg| arg.strip_prefix("--day").map(|f| f.to_string()))
        .map(|r| r.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let years = env::args()
        .filter_map(|arg| arg.strip_prefix("--year").map(|f| f.to_string()))
        .map(|r| r.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut solutions: Vec<SolutionExec> = Vec::new();
    solutions.extend(solve_2020::get_executions());
    solutions.extend(solve_2021::get_executions());
    solutions.extend(solve_2022::get_executions());

    let solutions: Vec<&SolutionExec> = solutions
        .iter()
        .filter(|x| {
            (years.is_empty() || years.contains(&x.year))
                && (days.is_empty() || days.contains(&x.day))
        })
        .collect();

    let start_run = Instant::now();

    let strings: Vec<String> = solutions
        .par_iter()
        .map(|&exec| {
            let now = Instant::now();
            let result = (exec.f)();
            let ms = now.elapsed().as_secs_f64() * 1000.0;
            format!(
                "[{}, day {}]: {}  ({:.2}ms)",
                exec.year, exec.day, result, ms
            )
        })
        .collect();

    for s in strings {
        println!("{}", s);
    }

    println!(
        "Done with AOC! Took {:.2}ms in total.",
        start_run.elapsed().as_secs_f64() * 1000.0
    );
}
