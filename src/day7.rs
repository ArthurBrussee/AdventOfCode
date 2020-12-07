use std::collections::HashMap;
use std::fs;

type RuleBook = HashMap<String, Vec<(String, usize)>>;
type BagCache = HashMap<(String, String), bool>;

fn contains(rules: &RuleBook, bag: &str, search: &str, memoize: &mut BagCache) -> bool {
    let cache = (bag.to_string(), search.to_string());
    if let Some(ret) = memoize.get(&(cache)) {
        return *ret;
    }
    let contains = rules[bag]
        .iter()
        .any(|bag| bag.0 == search || contains(rules, &bag.0, search, memoize));
    memoize.insert(cache, contains);
    contains
}

fn count_inner(rules: &RuleBook, root: &str) -> usize {
    rules[root]
        .iter()
        .map(|bag| bag.1 * (1 + count_inner(rules, &bag.0)))
        .sum()
}

pub fn calc() -> (usize, usize) {
    fn parse_rulebook(format: &str) -> (String, Vec<(String, usize)>) {
        let bag_name = |name: &str| name[0..name.find("bag").unwrap() - 1].to_string();
        let parts = format.split(" contain ").collect::<Vec<_>>();
        let bags = parts[1]
            .split(", ")
            .filter_map(|l| {
                if !l.starts_with("no other bags") {
                    Some((bag_name(&l[2..]), l[0..1].parse().unwrap()))
                } else {
                    None
                }
            })
            .collect();
        (bag_name(parts[0]), bags)
    }
    let rulebook: RuleBook = fs::read_to_string("./inputs/day7.txt")
        .unwrap()
        .lines()
        .map(parse_rulebook)
        .collect();

    let p1 = rulebook
        .keys()
        .filter(|bag| contains(&rulebook, bag, "shiny gold", &mut HashMap::new()))
        .count();
    (p1, count_inner(&rulebook, "shiny gold"))
}
