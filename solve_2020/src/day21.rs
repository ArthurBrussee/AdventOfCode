use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn calc() -> (usize, String) {
    let input_str = fs::read_to_string("./solve_2020/inputs/day21.txt").unwrap();

    let mut all_ingredients: Vec<&str> = Vec::new();
    let mut allergy_potential: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in input_str.lines() {
        let mut parts = line.split(" (contains ");
        let food_list: HashSet<&str> = parts.next().unwrap().split(' ').collect();
        let allergy_list = parts
            .next()
            .and_then(|f| f.strip_suffix(')'))
            .unwrap()
            .split(", ");

        all_ingredients.extend(food_list.iter());

        for allergy in allergy_list {
            if let Some(cur_potential) = allergy_potential.get_mut(allergy) {
                *cur_potential = cur_potential
                    .intersection(&food_list)
                    .to_owned()
                    .copied()
                    .collect::<HashSet<&str>>();
            } else {
                let mut set = HashSet::new();
                for &val in &food_list {
                    set.insert(val);
                }
                allergy_potential.insert(allergy, set);
            }
        }
    }

    for _ in 0..allergy_potential.len() {
        let solved_keys = allergy_potential
            .iter()
            .filter_map(|(k, v)| {
                if v.len() == 1 {
                    Some((k.to_owned(), v.iter().next().unwrap().to_owned()))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        for (k, val) in solved_keys {
            for (_, cur_list) in allergy_potential.iter_mut().filter(|(&kp, _)| kp != k) {
                cur_list.remove(val);
            }
        }
    }

    let safe_count = all_ingredients
        .iter()
        .filter(|&&i| !allergy_potential.values().any(|v| v.contains(i)))
        .count();

    let mut allergen_tuples: Vec<(&str, &str)> = allergy_potential
        .iter()
        .map(|(&k, v)| (k, *v.iter().next().unwrap()))
        .collect();

    allergen_tuples.sort_by(|a, b| a.0.cmp(b.0));

    let bad_list = allergen_tuples
        .iter()
        .map(|(_, v)| v.to_owned())
        .collect::<Vec<&str>>()
        .join(",");

    (safe_count, bad_list)
}
