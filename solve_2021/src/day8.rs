use std::collections::{HashMap, HashSet};

fn parse_input(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    input
        .lines()
        .map(|l| {
            let (p1, p2) = l.split_once('|').unwrap();

            let words_p1 = p1.trim().split(' ').map(|x| x.to_owned()).collect();
            let words_p2 = p2.trim().split(' ').map(|x| x.to_owned()).collect();

            (words_p1, words_p2)
        })
        .collect()
}

fn count_1478(strs: &[String]) -> u32 {
    strs.iter()
        .filter(|x| {
            let c = x.chars().count();
            c == 2 || c == 3 || c == 4 || c == 7
        })
        .count() as u32
}

fn resolve_mapping(strs: &[&str]) -> HashMap<String, usize> {
    fn solve_panel<const N: usize>(
        strs: &[&str],
        panels: [usize; N],
        panel_map: &mut HashMap<usize, Vec<char>>,
    ) {
        for option in strs.iter().filter(|s| s.chars().count() == N) {
            for p in panels {
                let cur = panel_map.get(&p);

                match cur {
                    Some(vals) => {
                        let new_vals: HashSet<char> = option.chars().collect();
                        let cur_vals: HashSet<char> = vals.iter().cloned().collect();

                        let intersect = new_vals.intersection(&cur_vals).cloned().collect();
                        panel_map.insert(p, intersect);

                        println!("{}", panel_map[&p].len());

                        if panel_map[&p].len() == 1 {
                            println!("Solved one wohoo");
                        }
                    }
                    None => {
                        panel_map.insert(p, option.chars().collect());
                    }
                }
            }
        }
    }

    fn calc_str<const N: usize>(
        panel_map: &HashMap<usize, Vec<char>>,
        panels: [usize; N],
    ) -> String {
        let mut chars = Vec::new();
        for p in panels {
            let options = &panel_map[&p];
            assert!(options.len() == 1, "More than 1 option urgh...");
            chars.push(options[0]);
        }
        chars.sort_unstable();
        chars.iter().collect()
    }

    let map_0 = [0, 1, 2, 4, 5, 6];
    let map_1 = [2, 5];
    let map_2 = [0, 2, 3, 4, 6];
    let map_3 = [0, 2, 3, 5, 6];
    let map_4 = [1, 2, 3, 5];
    let map_5 = [0, 1, 3, 5, 6];
    let map_6 = [0, 1, 3, 4, 5, 6];
    let map_7 = [0, 2, 5];
    let map_8 = [0, 1, 2, 3, 4, 5, 6];
    let map_9 = [0, 1, 2, 3, 5, 6];

    let mut panel_map = HashMap::new();
    solve_panel(strs, map_1, &mut panel_map);
    solve_panel(strs, map_4, &mut panel_map);
    solve_panel(strs, map_7, &mut panel_map);
    solve_panel(strs, map_8, &mut panel_map);

    HashMap::from([
        (calc_str(&panel_map, map_0), 0),
        (calc_str(&panel_map, map_1), 1),
        (calc_str(&panel_map, map_2), 2),
        (calc_str(&panel_map, map_3), 3),
        (calc_str(&panel_map, map_4), 4),
        (calc_str(&panel_map, map_5), 5),
        (calc_str(&panel_map, map_6), 6),
        (calc_str(&panel_map, map_7), 7),
        (calc_str(&panel_map, map_8), 8),
        (calc_str(&panel_map, map_9), 9),
    ])
}

pub fn calc(input: &str) -> (u32, u32) {
    let lines = parse_input(input);
    let total_count: u32 = lines.iter().map(|(_, p2)| count_1478(p2)).sum();

    let p1 = total_count;
    let p2 = lines
        .iter()
        .map(|(p1, p2)| {
            let all_worlds: Vec<&str> = p1.iter().chain(p2.iter()).map(|x| x as &str).collect();
            let mapping = resolve_mapping(&all_worlds);

            p2.iter()
                .map(|x| {
                    println!("{:?}", mapping);
                    println!("{:?}", x);
                    let mut chars: Vec<char> = x.chars().collect();
                    chars.sort_unstable();
                    let sorted: String = chars.iter().collect();
                    mapping[&sorted]
                })
                .rev()
                .enumerate()
                .map(|(pos, e)| 10u32.pow(pos as u32) * e as u32)
                .sum::<u32>()
        })
        .sum();

    (p1, p2)
}

#[test]
fn test() {
    let input =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    let words = parse_input(input);
    let total_count: u32 = words.iter().map(|(_, p2)| count_1478(p2)).sum();

    assert_eq!(total_count, 26);
}
