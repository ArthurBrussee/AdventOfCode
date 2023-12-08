use aoc_lib::AocSolution;

pub struct Solution;

fn search_digit(s: &str, cursor_it: impl Iterator<Item = usize>, with_letters: bool) -> u32 {
    for c in cursor_it {
        if let Some(d) = s.chars().next().and_then(|c| c.to_digit(10)) {
            return d;
        }

        if with_letters {
            let numbers = [
                "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            ];

            for (i, n) in numbers.iter().enumerate() {
                if s[c..].strip_prefix(n).is_some() {
                    return (i + 1) as u32;
                }
            }
        }
    }

    0
}

impl AocSolution<usize, usize> for Solution {
    const DATE: (u32, u32) = (2023, 1);

    fn calc(input: &str) -> (usize, usize) {
        let (p1, p2) = input
            .lines()
            .map(|l| {
                let it = 0..l.len();

                let p1 = search_digit(l, it.clone(), false) * 10
                    + search_digit(l, it.clone().rev(), false);
                let p2 = search_digit(l, it.clone(), true) * 10
                    + search_digit(l, it.clone().rev(), true);
                (p1, p2)
            })
            .fold((0, 0), |agg, el| (agg.0 + el.0, agg.1 + el.1));

        (p1 as usize, p2 as usize)
    }
}

#[test]
fn test() {
    Solution::test(209, 281);
}
