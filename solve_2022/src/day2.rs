use aoc_lib::AocSolution;

fn score(l: usize, r: usize) -> usize {
    if l == r {
        4 + r
    } else if (l + 1) % 3 == r {
        7 + r
    } else {
        1 + r
    }
}

pub struct Solution;

impl AocSolution<usize, usize> for Solution {
    const DATE: (u32, u32) = (2022, 2);

    fn calc(input: &str) -> (usize, usize) {
        input
            .lines()
            .map(|x| {
                let (l, r) = x.split_once(' ').unwrap();

                let l = match l {
                    "A" => 0,
                    "B" => 1,
                    "C" => 2,
                    _ => unreachable!(),
                };

                let r1 = match r {
                    "X" => 0,
                    "Y" => 1,
                    "Z" => 2,
                    _ => unreachable!(),
                };

                let r2 = match r {
                    "X" => (l + 2) % 3,
                    "Y" => l,
                    "Z" => (l + 1) % 3,
                    _ => unreachable!(),
                };

                (score(l, r1), score(l, r2))
            })
            .fold((0, 0), |(c1, c2), (s1, s2)| (c1 + s1, c2 + s2))
    }
}

#[test]
fn test() {
    Solution::test(15, 12);
}
