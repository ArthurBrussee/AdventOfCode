use std::cmp;
use std::fmt::{Debug, Display};
use std::{fs, str::FromStr};

pub trait AocSolution<P1 = u32, P2 = u32>
where
    P1: cmp::PartialEq + Debug + Display + Send + Sync,
    P2: cmp::PartialEq + Debug + Display + Send + Sync,
{
    const DATE: (u32, u32);

    fn year() -> u32 {
        Self::DATE.0
    }

    fn day() -> u32 {
        Self::DATE.1
    }

    fn calc(input: &str) -> (P1, P2);

    fn read_file(test: bool) -> String {
        let path = if test {
            format!("./inputs/day{}_test.txt", Self::day())
        } else {
            format!("./solve_{}/inputs/day{}.txt", Self::year(), Self::day())
        };
        let file = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Please include input file for {}!", path));
        file.replace("\r\n", "\n")
    }

    fn test(solution1: P1, solution2: P2) {
        let (p1, p2) = Self::calc(&Self::read_file(true));
        assert_eq!(p1, solution1);
        assert_eq!(p2, solution2);
    }
}

pub struct SolutionExec {
    pub f: fn() -> String,
    pub year: u32,
    pub day: u32,
}

pub trait Execute<P1, P2> {
    fn get_exec() -> SolutionExec;
}

impl<T, P1, P2> Execute<P1, P2> for T
where
    T: AocSolution<P1, P2>,
    P1: cmp::PartialEq + Debug + Display + Send + Sync,
    P2: cmp::PartialEq + Debug + Display + Send + Sync,
{
    fn get_exec() -> SolutionExec {
        let f = || {
            let (p1, p2) = T::calc(&T::read_file(false));
            format!("{}, {}", p1, p2)
        };

        SolutionExec {
            f,
            year: T::year(),
            day: T::day(),
        }
    }
}

pub fn map_file_lines<T>(path: &str, func: fn(&str) -> T) -> Vec<T> {
    map_lines(
        &fs::read_to_string(path).expect("Failed to find file!"),
        func,
    )
}

pub fn map_lines<T>(str: &str, func: fn(&str) -> T) -> Vec<T> {
    str.lines().map(func).collect()
}

pub fn parse_lines<T>(str: &str) -> Vec<T>
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    map_lines(str, |it| it.parse().expect("Failed to parse!"))
}

pub trait DoubleLineSplit {
    type Iterator;

    fn split_at_empty_line(&self) -> Self::Iterator;
}

impl<'a> DoubleLineSplit for &'a str {
    type Iterator = std::str::Split<'a, &'a str>;

    fn split_at_empty_line(&self) -> std::str::Split<'a, &'a str> {
        if self.contains("\r\n") {
            self.split("\r\n\r\n")
        } else {
            self.split("\n\n")
        }
    }
}
