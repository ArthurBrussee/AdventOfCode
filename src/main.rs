use std::fmt::Display;
use std::time::Instant;

mod day1;
mod day2;
mod day3;
mod day4;

mod lib;

fn print_solution(day: impl Display, part: impl Display, solution: impl Display, start: Instant) {
    let ms = start.elapsed().as_secs_f64() * 1000.0;
    println!("Day {}, part {}, {:.2}ms: {}", day, part, ms, solution);
}

fn main() {
    println!("AOC 2020!");

    let start_run = Instant::now();

    let now = Instant::now();
    print_solution(1, 1, day1::part1(), now);

    let now = Instant::now();
    print_solution(1, 2, day1::part2(), now);

    let now = Instant::now();
    print_solution(2, 1, day2::part1(), now);

    let now = Instant::now();
    print_solution(2, 2, day2::part2(), now);

    let now = Instant::now();
    print_solution(3, 1, day3::part1(), now);

    let now = Instant::now();
    print_solution(3, 2, day3::part2(), now);

    let now = Instant::now();
    print_solution(4, 1, day4::part1(), now);

    let now = Instant::now();
    print_solution(4, 2, day4::part2(), now);

    println!(
        "Done with AOC! Took {:.2}ms",
        start_run.elapsed().as_secs_f64() * 1000.0
    );
}
