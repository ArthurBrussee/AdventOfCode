#![feature(array_windows)]

fn main() {
    let years: Vec<usize> = aoc_lib::get_years_to_run().collect();

    if years.contains(&2020) {
        solve_2020::run()
    }

    if years.contains(&2021) {
        solve_2021::run()
    }
}
