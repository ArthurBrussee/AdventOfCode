use aoc_lib::AocSolution;

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

fn mutual_count(a: &str, b: &str) -> u8 {
    a.as_bytes()
        .iter()
        .filter(|x| b.as_bytes().contains(x))
        .count() as u8
}

fn solve_map(strs: &[String]) -> [&str; 10] {
    let mut maps = [""; 10];
    maps[1] = strs.iter().find(|x| x.len() == 2).unwrap();
    maps[4] = strs.iter().find(|x| x.len() == 4).unwrap();
    maps[7] = strs.iter().find(|x| x.len() == 3).unwrap();
    maps[8] = strs.iter().find(|x| x.len() == 7).unwrap();

    let deduct = move |overlaps: [u8; 4]| {
        strs.iter()
            .find(|x| {
                mutual_count(x, maps[1]) == overlaps[0]
                    && mutual_count(x, maps[4]) == overlaps[1]
                    && mutual_count(x, maps[7]) == overlaps[2]
                    && x.len() as u8 == overlaps[3]
            })
            .unwrap()
    };

    maps[0] = deduct([2, 3, 3, 6]);
    maps[2] = deduct([1, 2, 2, 5]);
    maps[3] = deduct([2, 3, 3, 5]);
    maps[5] = deduct([1, 3, 2, 5]);
    maps[6] = deduct([1, 3, 2, 6]);
    maps[9] = deduct([2, 4, 3, 6]);
    maps
}

pub struct Solution;

impl AocSolution<usize, usize> for Solution {
    const DATE: (u32, u32) = (2021, 8);

    fn calc(input: &str) -> (usize, usize) {
        let lines = parse_input(input);

        let total_count = lines
            .iter()
            .map(|(_, p2)| {
                p2.iter()
                    .filter(|x| x.len() == 2 || x.len() == 3 || x.len() == 4 || x.len() == 7)
                    .count()
            })
            .sum();
        let p1 = total_count;
        let p2 = lines
            .iter()
            .map(|(p1, p2)| {
                let mapping = solve_map(p1);

                p2.iter()
                    .map(|x| {
                        mapping
                            .iter()
                            .position(|m| m.len() == x.len() && mutual_count(m, x) == x.len() as u8)
                            .unwrap()
                    })
                    .rev()
                    .enumerate()
                    .map(|(pos, e)| 10usize.pow(pos as u32) * e)
                    .sum::<usize>()
            })
            .sum();

        (p1, p2)
    }
}

#[test]
fn test() {
    Solution::test(26, 61229);
}
