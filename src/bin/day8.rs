
use std::collections::HashSet;

#[derive(Clone, Copy)]
enum Instruction {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

fn run_program(program : &Vec<Instruction>) -> (bool, i64) {
    let mut accumulator : i64 = 0;
    let mut pc : i64 = 0;
    let mut line_flags = HashSet::new();
    
    loop {
        if pc as usize >= program.len() {
            return (true, accumulator);
        }
        unsafe {
            match program.get_unchecked(pc as usize) {
                Instruction::Acc(arg) => {
                    accumulator += arg;
                    pc += 1;
                }
                Instruction::Jmp(arg) => pc += arg,
                Instruction::Nop(_arg) => pc += 1,
            }
        }

        if line_flags.contains(&pc) {
            return (false, accumulator);
        }
        line_flags.insert(pc);
    }
}

fn main() {

    let program : Vec<_> = aoc::read_lines_as_vec("./day8.txt").iter()
        .map(|line| {
            let mut line_parts = line.split(' ');
            let op = line_parts.next().unwrap();
            let arg = line_parts.next().unwrap().parse::<i64>().unwrap();
            match op {
                "acc" => Instruction::Acc(arg),
                "jmp" => Instruction::Jmp(arg),
                "nop" => Instruction::Nop(arg),
                x => panic!("Unknown opcode {}", x)
            }
        }).collect();

    let (_, accum) =  run_program(&program);
    println!("Part 1: {}", accum);

    for (i, instr) in program.iter().enumerate() {
        match instr {
            Instruction::Jmp(arg) => {
                let mut new_program = program.clone();
                new_program[i] = Instruction::Nop(*arg);
                let (exited, accumulator) = run_program(&new_program);
                if exited {
                    println!("Part 2: {}", accumulator);
                    break;
                }
            }
            Instruction::Nop(arg) => {
                let mut new_program = program.clone();
                new_program[i] = Instruction::Jmp(*arg);
                let (exited, accumulator) = run_program(&new_program);
                if exited {
                    println!("Part 2: {}", accumulator);
                    break;
                }
            }
            _ => (),
        }
    }
}

