use std::str::FromStr;

use aoc_lib::AocSolution;

#[derive(Default)]
struct Submarine {
    x: i32,
    y: i32,
    aim: i32,
}

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Submarine {
    fn command(&mut self, commands: &[Command]) {
        for command in commands {
            match command {
                Command::Down(num) => self.y += num,
                Command::Up(num) => self.y -= num,
                Command::Forward(num) => self.x += num,
            }
        }
    }

    fn command_alt(&mut self, commands: &[Command]) {
        for command in commands {
            match command {
                Command::Down(num) => self.aim += num,
                Command::Up(num) => self.aim -= num,
                Command::Forward(num) => {
                    self.x += num;
                    self.y += self.aim * num;
                }
            }
        }
    }
}

impl FromStr for Command {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (t, num) = s.split_once(' ').ok_or("no space in input.")?;

        let num = num.parse()?;

        let result = match t {
            "forward" => Ok(Command::Forward(num)),
            "down" => Ok(Command::Down(num)),
            "up" => Ok(Command::Up(num)),
            _ => Err("Unknown command"),
        }?;

        Ok(result)
    }
}

impl Submarine {}

pub struct Solution;

impl AocSolution<usize, usize> for Solution {
    const YEAR: u32 = 2021;
    const DAY: u32 = 2;

    fn calc(input: &str) -> (usize, usize) {
        let commands: Vec<Command> = aoc_lib::parse_lines(input);
        let mut sub = Submarine::default();
        sub.command(&commands);
        let p1 = sub.x * sub.y;

        let mut sub = Submarine::default();
        sub.command_alt(&commands);
        let p2 = sub.x * sub.y;

        (p1 as usize, p2 as usize)
    }
}

#[test]
fn test() {
    Solution::test(15 * 10, 15 * 60);
}
