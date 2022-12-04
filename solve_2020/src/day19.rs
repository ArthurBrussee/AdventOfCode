use aoc_lib::AocSolution;

pub struct Solution;

use std::collections::HashMap;

use aoc_lib::DoubleLineSplit;

#[derive(Clone)]
enum Rule {
    Leaf(u32),
    Match(Vec<Vec<u32>>),
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Message<'a> {
    val: &'a [u32],
}

fn strip_prexix<'a>(
    message: Message<'a>,
    rule: &Rule,
    rules: &HashMap<u32, Rule>,
) -> Vec<Message<'a>> {
    match rule {
        Rule::Leaf(leaf) => match message.val {
            [start, val @ ..] if start == leaf => vec![Message { val }],
            _ => Vec::new(),
        },
        Rule::Match(branches) => branches
            .iter()
            .flat_map(|branch| {
                branch.iter().fold(vec![message], |cur, r| {
                    cur.iter()
                        .flat_map(|m| strip_prexix(*m, &rules[r], rules))
                        .collect()
                })
            })
            .collect(),
    }
}

impl AocSolution<usize, usize> for Solution {
    const YEAR: u32 = 2020;
    const DAY: u32 = 19;

    fn calc(input: &str) -> (usize, usize) {
        let mut input_parts = input.split_at_doubleblank();
        let mut char_rules = HashMap::<char, u32>::new();
        let rules = input_parts
            .next()
            .unwrap()
            .lines()
            .map(|str| {
                let mut parts = str.split(':');
                let num = parts.next().unwrap().parse().unwrap();
                let rule_parts = parts.next().unwrap();

                if rule_parts.contains('"') {
                    let ch = rule_parts.trim().replace('\"', "").chars().next().unwrap();
                    char_rules.insert(ch, num);
                    (num, Rule::Leaf(num))
                } else {
                    let rules = rule_parts.trim().split('|');

                    let matches = rules
                        .map(|r| r.trim().split(' ').map(|n| n.parse().unwrap()).collect())
                        .collect::<Vec<Vec<u32>>>();
                    (num, Rule::Match(matches))
                }
            })
            .collect::<HashMap<_, _>>();

        let messages_vecs = input_parts
            .next()
            .unwrap()
            .lines()
            .map(|l| l.chars().map(|c| char_rules[&c]).collect::<Vec<u32>>())
            .collect::<Vec<_>>();

        let p1 = messages_vecs
            .iter()
            .filter(|val| {
                strip_prexix(Message { val }, &rules[&0], &rules)
                    .iter()
                    .any(|r| r.val.is_empty())
            })
            .count();

        let mut new_rules = rules;
        new_rules.insert(8, Rule::Match(vec![vec![42], vec![42, 8]]));
        new_rules.insert(11, Rule::Match(vec![vec![42, 31], vec![42, 11, 31]]));

        let p2 = messages_vecs
            .iter()
            .filter(|val| {
                strip_prexix(Message { val }, &new_rules[&0], &new_rules)
                    .iter()
                    .any(|r| r.val.is_empty())
            })
            .count();
        (p1, p2)
    }
}
