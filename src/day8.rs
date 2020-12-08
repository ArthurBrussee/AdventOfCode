use std::fs;

#[derive(Copy, Clone)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

fn execute_program(instructions: &Vec<Instruction>) -> (bool, i32) {
    let mut ic: usize = 0;
    let mut acc: i32 = 0;
    let mut seen: Vec<bool> = instructions.iter().map(|_| false).collect();

    loop {
        if ic >= instructions.len() {
            return (true, acc);
        }
        if seen[ic] {
            return (false, acc);
        }
        seen[ic] = true;

        match instructions[ic] {
            Instruction::Nop(_) => ic += 1,
            Instruction::Acc(num) => {
                acc += num;
                ic += 1;
            }
            Instruction::Jmp(num) => ic = (ic as i32 + num) as usize,
        }
    }
}

pub fn calc() -> (i32, i32) {
    let instructions = fs::read_to_string("./inputs/day8.txt")
        .expect("Can't find input file.")
        .lines()
        .map(|l| {
            let mut parts = l.split(" ");
            let instr = parts.next().unwrap();
            let num = parts.next().and_then(|x| x.parse().ok()).unwrap();
            match instr {
                "nop" => Instruction::Nop(num),
                "acc" => Instruction::Acc(num),
                "jmp" => Instruction::Jmp(num),
                _ => unreachable!("Invalid instruction"),
            }
        })
        .collect::<Vec<_>>();

    let (_, p1) = execute_program(&instructions);
    let mut new_program = instructions.to_vec();

    for i in 0..instructions.len() {
        let instruction = new_program[i];
        match instruction {
            Instruction::Acc(_) => continue,
            Instruction::Nop(0) => continue,
            Instruction::Nop(num) => new_program[i] = Instruction::Jmp(num),
            Instruction::Jmp(num) => new_program[i] = Instruction::Nop(num),
        }
        let (completed, p2) = execute_program(&new_program);
        new_program[i] = instruction;
        if completed {
            return (p1, p2);
        }
    }

    unreachable!("No way to fix the program.");
}
