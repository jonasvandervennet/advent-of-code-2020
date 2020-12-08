use crate::util::{print_part_1, print_part_2};
use std::collections::HashSet;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Clone, Copy, PartialEq)]
enum InstructionType {
    ACC,
    NOP,
    JMP,
    INVALID,
}

#[derive(Clone)]
struct Instruction {
    i_type: InstructionType,
    arg: i32,
}

fn run_program(instructions: &Vec<Instruction>) -> (bool, i32) {
    let mut acc: i32 = 0;
    let mut i_pointer: usize = 0;

    let mut seen_instruction_pointers: HashSet<usize> = HashSet::new();

    loop {
        if i_pointer == instructions.len() {
            // program terminated
            return (true, acc);
        }
        if seen_instruction_pointers.contains(&i_pointer) {
            // loop detected
            return (false, acc);
        } else {
            seen_instruction_pointers.insert(i_pointer);
        }

        let ins: &Instruction = instructions
            .iter()
            .nth(i_pointer)
            .expect("could not reach instruction");

        match ins.i_type {
            InstructionType::NOP => {
                i_pointer += 1;
            }
            InstructionType::ACC => {
                acc += ins.arg;
                i_pointer += 1;
            }
            InstructionType::JMP => {
                let u_arg: usize;
                if ins.arg >= 0 {
                    u_arg = ins.arg as usize;
                    i_pointer += u_arg;
                } else {
                    // add negative number to usize
                    u_arg = -ins.arg as usize;
                    i_pointer -= u_arg;
                }
            }
            InstructionType::INVALID => {
                println!("unknown instruction. Exiting..");
                return (false, 0);
            }
        }
    }
}

fn make_program_terminate(instructions: &Vec<Instruction>) -> (bool, i32) {
    let mut instructions = instructions.clone();
    for index in 0..instructions.len() {
        let ins = &instructions[index];
        if ins.i_type == InstructionType::ACC {
            continue;
        }
        let orig_type = ins.i_type;
        if ins.i_type == InstructionType::NOP {
            instructions[index].i_type = InstructionType::JMP;
        } else if ins.i_type == InstructionType::JMP {
            instructions[index].i_type = InstructionType::NOP;
        }
        let (term, acc) = run_program(&instructions);
        if term {
            return (term, acc);
        }

        // reset change
        instructions[index].i_type = orig_type;
    }
    (false, 0)
}

fn read_program(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|ins| Instruction {
            i_type: match &ins[0..3] {
                "nop" => InstructionType::NOP,
                "acc" => InstructionType::ACC,
                "jmp" => InstructionType::JMP,
                _ => InstructionType::INVALID,
            },
            arg: ins[4..].parse::<i32>().expect("could not parse argument"),
        })
        .collect()
}

pub fn main() {
    let input = read_to_string("inputs/day8.txt").unwrap();
    let program = read_program(&input);

    // PART 1
    let start = Instant::now();
    let known_answer = "1801";
    let (_, part_1): (bool, i32) = run_program(&program);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "2060";
    let (_, part_2): (bool, i32) = make_program_terminate(&program);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_loop() {
        let input: &str = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6";
        let (terminated, acc): (bool, i32) = run_program(&read_program(input));
        assert_eq!(terminated, false);
        assert_eq!(acc, 5);
    }
    #[test]
    fn test_example_terminate() {
        let input: &str = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6";
        let (terminated, acc): (bool, i32) = make_program_terminate(&read_program(input));
        assert_eq!(terminated, true);
        assert_eq!(acc, 8);
    }
}
