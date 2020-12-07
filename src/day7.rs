use std::collections::HashMap;
use std::fs;

struct BagHold {
    name: String,
    count: usize,
}

struct BagRules {
    rules: HashMap<String, Vec<BagHold>>,
}

fn bag_name(name: &str) -> String {
    name.replace("bags", "")
        .replace("bag", "")
        .trim()
        .to_string()
}

fn parse_bagrule(format: &str) -> (String, Vec<BagHold>) {
    let parts = format.split(" contain ").collect::<Vec<_>>();

    let bags = parts[1]
        .strip_suffix(".")
        .unwrap()
        .split(",")
        .filter_map(|l| {
            let l = l.trim();
            if !l.starts_with("no other") {
                Some(BagHold {
                    count: l[0..1].parse().unwrap(),
                    name: bag_name(&l[2..]),
                })
            } else {
                None
            }
        })
        .collect();

    (bag_name(parts[0]), bags)
}

impl BagRules {
    fn contains(
        &self,
        root: &str,
        search: &str,
        memoize: &mut HashMap<(String, String), bool>,
    ) -> bool {
        let cache = (root.to_string(), search.to_string());
        match memoize.get(&cache) {
            Some(ret) => *ret,
            None => {
                let contains = self.rules[root]
                    .iter()
                    .any(|bag| bag.name == search || self.contains(&bag.name, search, memoize));

                memoize.insert(cache, contains);
                contains
            }
        }
    }

    fn count_inner(&self, root: &str) -> usize {
        self.rules[root].iter().fold(0, |acc, bag| {
            acc + bag.count * (1 + self.count_inner(&bag.name))
        })
    }
}

pub fn calc() -> (usize, usize) {
    let bag_rule = BagRules {
        rules: fs::read_to_string("./inputs/day7.txt")
            .unwrap()
            .lines()
            .map(|l| parse_bagrule(l))
            .collect(),
    };

    let mut memoize: HashMap<(String, String), bool> = HashMap::new();

    let contains_gold = bag_rule
        .rules
        .keys()
        .filter(|bag| bag_rule.contains(bag, "shiny gold", &mut memoize))
        .count();
    let bags_in_gold = bag_rule.count_inner("shiny gold");
    (contains_gold, bags_in_gold)
}
