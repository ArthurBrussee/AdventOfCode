use std::fs;

enum Instruction {
    North(f32),
    South(f32),
    East(f32),
    West(f32),
    Left(f32),
    Right(f32),
    Front(f32),
}

fn rotate(dir: (f32, f32), angle: f32) -> (f32, f32) {
    let c = angle.to_radians().cos();
    let s = angle.to_radians().sin();
    (dir.0 * c - dir.1 * s, dir.0 * s + dir.1 * c)
}

pub fn calc() -> (i32, i32) {
    let instructions = fs::read_to_string("./inputs/day12.txt")
        .expect("Can't find input file.")
        .lines()
        .map(|l| {
            let mut ch = l.chars();
            let t = ch.next().unwrap();
            let amount = ch.collect::<String>().parse().unwrap();
            match t {
                'N' => Instruction::North(amount),
                'S' => Instruction::South(amount),
                'E' => Instruction::East(amount),
                'W' => Instruction::West(amount),
                'L' => Instruction::Left(amount),
                'R' => Instruction::Right(amount),
                'F' => Instruction::Front(amount),
                _ => unreachable!("Invalid instruction."),
            }
        })
        .collect::<Vec<_>>();

    let mut pos = (0.0, 0.0);
    let mut dir = (1.0, 0.0);

    for instr in &instructions {
        match instr {
            Instruction::North(amount) => pos.1 += amount,
            Instruction::South(amount) => pos.1 -= amount,
            Instruction::East(amount) => pos.0 += amount,
            Instruction::West(amount) => pos.0 -= amount,
            Instruction::Left(amount) => dir = rotate(dir, *amount),
            Instruction::Right(amount) => dir = rotate(dir, -amount),
            Instruction::Front(amount) => pos = (pos.0 + dir.0 * amount, pos.1 + dir.1 * amount),
        }
    }
    let p1 = (pos.0.abs() + pos.1.abs()) as i32;

    let mut pos = (0.0, 0.0);
    let mut waypoint = (10.0, 1.0);

    for instr in &instructions {
        match instr {
            Instruction::North(amount) => waypoint.1 += amount,
            Instruction::South(amount) => waypoint.1 -= amount,
            Instruction::East(amount) => waypoint.0 += amount,
            Instruction::West(amount) => waypoint.0 -= amount,
            Instruction::Left(amount) => waypoint = rotate(waypoint, *amount),
            Instruction::Right(amount) => waypoint = rotate(waypoint, -amount),
            Instruction::Front(amount) => {
                pos = (pos.0 + waypoint.0 * amount, pos.1 + waypoint.1 * amount)
            }
        }
    }

    let p2 = (pos.0.abs() + pos.1.abs()) as i32;
    (p1, p2)
}
