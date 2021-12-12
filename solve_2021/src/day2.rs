use std::str::FromStr;

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
        let (t, num) = s.split_once(" ").ok_or("no space in input.")?;

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

pub fn calc(input: &str) -> (usize, usize) {
    let commands: Vec<Command> = aoc_lib::parse_lines(input);
    let mut sub = Submarine::default();
    sub.command(&commands);
    let p1 = sub.x * sub.y;

    let mut sub = Submarine::default();
    sub.command_alt(&commands);
    let p2 = sub.x * sub.y;

    (p1 as usize, p2 as usize)
}

#[test]
fn test_p1() {
    let inputs = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#;

    let commands: Vec<Command> = aoc_lib::parse_lines::<Command>(inputs);
    let mut sub = Submarine::default();
    sub.command(&commands);

    assert_eq!(sub.x, 15);
    assert_eq!(sub.y, 10);
}

#[test]
fn test_p2() {
    let inputs = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#;

    let commands: Vec<Command> = aoc_lib::parse_lines::<Command>(inputs);
    let mut sub = Submarine::default();
    sub.command_alt(&commands);

    assert_eq!(sub.x, 15);
    assert_eq!(sub.y, 60);
}
