use aoc_lib::AocSolution;

pub struct Solution;

impl AocSolution<i32, String> for Solution {
    const DATE: (u32, u32) = (2022, 10);

    fn calc(input: &str) -> (i32, String) {
        let commands: Vec<Option<i32>> = input
            .lines()
            .flat_map(|l| {
                if l == "noop" {
                    vec![None]
                } else {
                    vec![None, l.split_once(' ').and_then(|(_, n)| n.parse().ok())]
                }
            })
            .collect();

        let mut sprite_pos: i32 = 1;
        let mut ic = 0;

        let mut check_strengths = Vec::new();
        let mut pixels = "\n".to_string();

        for command in commands {
            ic += 1;
            if [20, 60, 100, 140, 180, 220].contains(&ic) {
                check_strengths.push(ic * sprite_pos);
            }
            let crt_loc = (ic - 1) % 40;
            let pixel = if (crt_loc - sprite_pos).abs() <= 1 {
                '#'
            } else {
                '.'
            };
            pixels.push(pixel);

            if ic % 40 == 0 && ic > 0 {
                pixels.push('\n');
            }

            if let Some(v) = command {
                sprite_pos += v;
            }
        }
        (check_strengths.iter().sum(), pixels)
    }
}

#[test]
fn test() {
    let p2_answer = r"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

    let (p1, p2) = Solution::calc(&Solution::read_file(true));

    assert_eq!(p1, 13140);

    for (a, b) in p2.lines().zip(p2_answer.lines()) {
        assert_eq!(a, b);
    }
}
