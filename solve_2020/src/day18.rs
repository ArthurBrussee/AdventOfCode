use std::collections::HashMap;
use std::fs;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Token {
    Number(u32),
    Times,
    Plus,
    OpenParen,
    CloseParen,
}

pub fn calc() -> (u64, u64) {
    let file_str =
        fs::read_to_string("./solve_2020/inputs/day18.txt").expect("Can't find input file.");
    let tokens = file_str.lines().map(tokenize).collect::<Vec<_>>();
    let p1 = tokens.iter().map(|l| evaluate(l, 1, 1)).sum();
    let p2 = tokens.iter().map(|l| evaluate(l, 2, 1)).sum();
    (p1, p2)
}

fn tokenize(s: &str) -> Vec<Token> {
    s.replace(" ", "")
        .chars()
        .map(|s| match s {
            '*' => Token::Times,
            '+' => Token::Plus,
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            _ => Token::Number(s.to_string().parse().unwrap()),
        })
        .collect()
}

fn evaluate(tokens: &[Token], prec_add: i32, prec_times: i32) -> u64 {
    let mut prec = HashMap::<Token, i32>::new();
    prec.insert(Token::Times, prec_times);
    prec.insert(Token::Plus, prec_add);
    prec.insert(Token::OpenParen, i32::MIN);

    let mut op_stack = Vec::new();
    let mut postfix = Vec::new();

    for token in tokens {
        match token {
            Token::Number(_) => postfix.push(*token),
            Token::OpenParen => op_stack.push(*token),
            Token::CloseParen => loop {
                let top_token = op_stack.pop().unwrap();
                if top_token == Token::OpenParen {
                    break;
                }
                postfix.push(top_token);
            },
            Token::Plus | Token::Times => {
                while !op_stack.is_empty() && prec[op_stack.last().unwrap()] >= prec[token] {
                    postfix.push(op_stack.pop().unwrap());
                }
                op_stack.push(*token)
            }
        }
    }

    postfix.extend(op_stack.iter().rev());

    let mut vm = Vec::<u64>::new();
    for token in postfix {
        match token {
            Token::Number(num) => vm.push(num as u64),
            Token::Plus => {
                let (left, right) = (vm.pop().unwrap(), vm.pop().unwrap());
                vm.push(left + right);
            }
            Token::Times => {
                let (left, right) = (vm.pop().unwrap(), vm.pop().unwrap());
                vm.push(left * right);
            }
            _ => unreachable!(),
        }
    }
    vm[0]
}
