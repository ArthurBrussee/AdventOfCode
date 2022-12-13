use aoc_lib::{AocSolution, DoubleLineSplit};
use itertools::Itertools;
use std::str::FromStr;

pub struct Solution;

#[derive(Debug, Clone)]
enum Operation {
    Times(usize),
    Add(usize),
    Sqr,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<usize>,
    op: Operation,
    test: usize,
    monkey_true: usize,
    monkey_false: usize,
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, items, operation, test, monkey_true, monkey_false) =
            s.lines().map(|l| l.trim_start()).collect_tuple().unwrap();

        let items = items
            .strip_prefix("Starting items: ")
            .unwrap()
            .split(", ")
            .filter_map(|l| l.parse().ok())
            .collect();

        let operation = operation.strip_prefix("Operation: new = old ").unwrap();

        let op = if operation.strip_prefix("* old").is_some() {
            Operation::Sqr
        } else if let Some(mul) = operation.strip_prefix("* ") {
            Operation::Times(mul.parse().unwrap())
        } else if let Some(add) = operation.strip_prefix("+ ") {
            Operation::Add(add.parse().unwrap())
        } else {
            panic!("{}", operation);
        };

        let test: usize = test
            .strip_prefix("Test: divisible by ")
            .unwrap()
            .parse()
            .unwrap();

        let monkey_true: usize = monkey_true
            .strip_prefix("If true: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();

        let monkey_false: usize = monkey_false
            .strip_prefix("If false: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();

        Ok(Self {
            items,
            op,
            test,
            monkey_true,
            monkey_false,
        })
    }
}

fn monkey_business(monkeys: &[Monkey], div: usize, rounds: usize) -> usize {
    let mut monkeys = monkeys.to_owned();
    let mut inspections: Vec<usize> = vec![0; monkeys.len()];

    let ring_total: usize = monkeys.iter().map(|m| m.test).product();

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            inspections[i] += monkey.items.len();

            let targets: Vec<(usize, usize)> = monkey
                .items
                .drain(0..)
                .map(|item| {
                    let worry = match monkey.op {
                        Operation::Times(x) => item * x,
                        Operation::Add(x) => item + x,
                        Operation::Sqr => item * item,
                    } / div;

                    let throw_to = if worry % monkey.test == 0 {
                        monkey.monkey_true
                    } else {
                        monkey.monkey_false
                    };

                    (throw_to, worry % ring_total)
                })
                .collect();

            for (t, item) in targets {
                monkeys[t as usize].items.push(item);
            }
        }
    }

    inspections
        .iter()
        .sorted_by(|a, b| b.cmp(a))
        .take(2)
        .product()
}

impl AocSolution<usize, usize> for Solution {
    const DATE: (u32, u32) = (2022, 11);

    fn calc(input: &str) -> (usize, usize) {
        let monkeys: Vec<Monkey> = input
            .split_at_empty_line()
            .map(|l| l.parse().unwrap())
            .collect();

        let inspections_p1 = monkey_business(&monkeys, 3, 20);
        let inspections_p2 = monkey_business(&monkeys, 1, 10000);

        (inspections_p1, inspections_p2)
    }
}

#[test]
fn test() {
    Solution::test(10605, 2713310158);
}
