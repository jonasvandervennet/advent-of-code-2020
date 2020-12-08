use crate::util::{print_part_1, print_part_2};
use std::collections::HashSet;
use std::fs::read_to_string;
use std::time::Instant;

fn run_program(instructions: &Vec<&str>) -> (bool, i32) {
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

        let ins = instructions
            .iter()
            .nth(i_pointer)
            .expect("could not reach instruction");
        let op: &str = &ins[0..3];
        let arg: &i32 = &ins[4..].parse::<i32>().expect("could not parse argument");

        match op {
            "nop" => {
                i_pointer += 1;
            }
            "acc" => {
                acc += arg;
                i_pointer += 1;
            }
            "jmp" => {
                let u_arg: usize;
                if *arg >= 0 {
                    u_arg = *arg as usize;
                    i_pointer += u_arg;
                } else {
                    u_arg = -arg as usize;
                    i_pointer -= u_arg;
                }
            }
            _ => {
                println!("unknown instruction. Exiting..");
                break;
            }
        }
    }
    (true, acc)
}

fn make_program_terminate(instructions: &Vec<&str>) -> (bool, i32) {
    for index in 0..instructions.len() {
        let ins = instructions[index];
        let op: &str = &ins[0..3];
        if op == "acc" {
            continue;
        }
        let mut new_instructions: Vec<&str> = instructions.to_owned();
        let new_ins;
        if op == "nop" {
            new_ins = format!("jmp{}", &ins[3..]);
            new_instructions[index] = &new_ins;
        } else if op == "jmp" {
            new_ins = format!("nop{}", &ins[3..]);
            new_instructions[index] = &new_ins;
        }
        let (term, acc) = run_program(&new_instructions);
        if term {
            return (term, acc);
        }
    }
    (false, 0)
}

pub fn main() {
    let input = read_to_string("inputs/day8.txt").unwrap();

    // PART 1
    let start = Instant::now();
    let known_answer: i32 = 1801;
    let (_, part_1): (bool, i32) = run_program(&input.lines().collect::<Vec<&str>>());
    let duration = start.elapsed();
    if part_1 != known_answer {
        print!("INCORRECT || ")
    }
    println!("PART 1: {}", part_1);
    println!("\t[{:?}]", duration);
    // print_part_1(part_1, known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer: i32 = 2060;
    let (_, part_2): (bool, i32) = make_program_terminate(&input.lines().collect::<Vec<&str>>());
    let duration = start.elapsed();
    if part_2 != known_answer {
        print!("INCORRECT || ")
    }
    println!("PART 2: {}", part_2);
    println!("\t[{:?}]", duration);
    // print_part_2(part_2, known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_loop() {
        let input: &str = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6";
        let (terminated, acc): (bool, i32) = run_program(&input.lines().collect::<Vec<&str>>());
        assert_eq!(terminated, false);
        assert_eq!(acc, 5);
    }
    #[test]
    fn test_example_terminate() {
        let input: &str = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6";
        let (terminated, acc): (bool, i32) =
            make_program_terminate(&input.lines().collect::<Vec<&str>>());
        assert_eq!(terminated, true);
        assert_eq!(acc, 8);
    }
}
