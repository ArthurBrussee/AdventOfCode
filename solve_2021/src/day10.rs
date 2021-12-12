pub fn calc(input: &str) -> (usize, usize) {
    let mut score_p1 = 0;
    let mut p2_scores = Vec::new();

    for l in input.lines() {
        let mut stack: Vec<char> = Vec::new();
        let mut parses = true;

        for c in l.chars() {
            match c {
                '{' | '(' | '[' | '<' => stack.push(c),
                ')' => {
                    if !matches!(stack.pop(), Some('(')) && parses {
                        score_p1 += 3;
                        parses = false;
                    }
                }
                ']' => {
                    if !matches!(stack.pop(), Some('[')) && parses {
                        score_p1 += 57;
                        parses = false;
                    }
                }
                '}' => {
                    if !matches!(stack.pop(), Some('{')) && parses {
                        score_p1 += 1197;
                        parses = false;
                    }
                }
                '>' => {
                    if !matches!(stack.pop(), Some('<')) && parses {
                        score_p1 += 25137;
                        parses = false;
                    }
                }
                _ => panic!("Not handled {}", c),
            };
        }

        // Valid line, check for missing in stack.
        if parses && !stack.is_empty() {
            let mut completion = Vec::new();
            for c in stack.iter().rev() {
                match c {
                    '(' => completion.push(1),
                    '[' => completion.push(2),
                    '{' => completion.push(3),
                    '<' => completion.push(4),
                    _ => panic!("Not handled {}", c),
                };
            }
            p2_scores.push(completion.iter().fold(0, |cum, x| cum * 5 + x));
        }
    }

    p2_scores.sort_unstable();
    let score_p2 = p2_scores[p2_scores.len() / 2];
    (score_p1, score_p2)
}

#[test]
fn test() {
    let (p1, p2) = calc(&aoc_lib::read_file(2021, 10, true));
    assert_eq!(p1, 26397);
    assert_eq!(p2, 288957);
}
