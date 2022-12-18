use aoc_lib::{AocSolution, DoubleLineSplit};
use itertools::Itertools;
use std::iter::Peekable;

pub struct Solution;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Expr {
    List(Vec<Expr>),
    Number(usize),
}

fn parse_package_peekable<I: Iterator<Item = char>>(chars: &mut Peekable<I>) -> Expr {
    match chars.peek().unwrap() {
        '[' => {
            chars.next();
            let mut exprs = Vec::new();

            loop {
                let next = *chars.peek().unwrap();
                if next == ']' {
                    chars.next();
                    return Expr::List(exprs);
                } else if next == ',' {
                    chars.next();
                }
                let expr = parse_package_peekable(chars);
                exprs.push(expr);
            }
        }
        _ => Expr::Number(
            chars
                .peeking_take_while(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse()
                .unwrap(),
        ),
    }
}

fn parse_package(input: &str) -> Expr {
    parse_package_peekable(&mut input.chars().peekable())
}

fn compare(left: &Expr, right: &Expr) -> std::cmp::Ordering {
    match (left, right) {
        (Expr::List(a), Expr::List(b)) => {
            for (a, b) in a.iter().zip(b) {
                let cmp = compare(a, b);
                if cmp == std::cmp::Ordering::Less || cmp == std::cmp::Ordering::Greater {
                    return cmp;
                }
            }
            a.len().cmp(&b.len())
        }
        (Expr::List(_), Expr::Number(b)) => compare(left, &Expr::List(vec![Expr::Number(*b)])),
        (Expr::Number(a), Expr::List(_)) => compare(&Expr::List(vec![Expr::Number(*a)]), right),
        (Expr::Number(a), Expr::Number(b)) => a.cmp(b),
    }
}

impl AocSolution<usize, usize> for Solution {
    const DATE: (u32, u32) = (2022, 13);

    fn calc(input: &str) -> (usize, usize) {
        let idxs = input
            .split_at_empty_line()
            .enumerate()
            .filter_map(|(i, l)| {
                let (p1, p2) = l.lines().map(parse_package).collect_tuple().unwrap();

                if compare(&p1, &p2) == std::cmp::Ordering::Less {
                    Some(i + 1)
                } else {
                    None
                }
            })
            .sum();

        let sep_1 = parse_package("[[2]]");
        let sep_2 = parse_package("[[6]]");

        let mut all_packages: Vec<Expr> = input
            .lines()
            .filter(|l| !l.is_empty())
            .map(parse_package)
            .chain([sep_1.clone(), sep_2.clone()])
            .collect();

        all_packages.sort_by(compare);

        let idx1 = all_packages.iter().position(|p| p == &sep_1).unwrap();
        let idx2 = all_packages.iter().position(|p| p == &sep_2).unwrap();

        (idxs, (idx1 + 1) * (idx2 + 1))
    }
}

#[test]
fn test() {
    Solution::test(13, 140);
}
