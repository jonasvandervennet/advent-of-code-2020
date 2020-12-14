use crate::util::{print_part_1, print_part_2};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::time::Instant;

fn sum_memory(input: &str) -> usize {
    let mut memory: HashMap<usize, usize> = HashMap::new();
    let mut and_mask: usize = 0;
    let mut or_mask: usize = 0;
    const BASE: usize = 2;

    for line in input.lines() {
        if line.chars().nth(1).expect("invalid input..") == 'a' {
            // line: mask = XXXXX
            and_mask = 0;
            or_mask = 0;
            for (i, c) in line[7..].chars().enumerate() {
                match c {
                    'X' => {
                        and_mask += BASE.pow(35 - i as u32);
                    }
                    '1' => {
                        or_mask += BASE.pow(35 - i as u32);
                    }
                    '0' | _ => {}
                }
            }
        } else {
            // line: mem[i] = XXXX
            let i = line.split("[").collect::<Vec<&str>>()[1]
                .split("]")
                .collect::<Vec<&str>>()[0]
                .parse::<usize>()
                .expect("invalid input..");
            let value = line.split("= ").collect::<Vec<&str>>()[1]
                .parse::<usize>()
                .expect("invalid input..");
            let masked_value = (value & and_mask) | or_mask;
            memory.insert(i, masked_value); // overwrites value if already present
        }
    }
    memory.values().sum()
}

fn sum_memory_2(input: &str) -> usize {
    let mut memory: HashMap<usize, usize> = HashMap::new();
    let mut and_mask: usize = 0;
    let mut or_mask: usize = 0;
    let mut flip_masks: Vec<usize> = Vec::new();
    const BASE: usize = 2;

    for line in input.lines() {
        if line.chars().nth(1).expect("invalid input..") == 'a' {
            // line: mask = XXXXX
            and_mask = 0;
            or_mask = 0;
            flip_masks = vec![0];
            for (i, c) in line[7..].chars().enumerate() {
                match c {
                    '0' => {
                        and_mask += BASE.pow(35 - i as u32);
                    }
                    '1' => {
                        or_mask += BASE.pow(35 - i as u32);
                    }
                    'X' => {
                        flip_masks.push(BASE.pow(35 - i as u32));
                    }
                    _ => {}
                }
            }
        } else {
            // line: mem[i] = XXXX
            let i = line.split("[").collect::<Vec<&str>>()[1]
                .split("]")
                .collect::<Vec<&str>>()[0]
                .parse::<usize>()
                .expect("invalid input..");
            let value = line.split("= ").collect::<Vec<&str>>()[1]
                .parse::<usize>()
                .expect("invalid input..");
            let masked_address = (i & and_mask) | or_mask;
            let mut used_flipmasks = HashSet::new();
            for n in 1..=flip_masks.len() {
                for comb in flip_masks.iter().combinations(n) {
                    let mut flip_mask = 0;
                    for mask in comb.iter() {
                        flip_mask += *mask;
                    }
                    if used_flipmasks.contains(&flip_mask) {
                        continue;
                    }
                    used_flipmasks.insert(flip_mask);
                    let flipped_masked_address = masked_address ^ flip_mask;
                    memory.insert(flipped_masked_address, value); // overwrites value if already present
                }
            }
        }
    }
    memory.values().sum()
}

pub fn main() {
    let input = read_to_string("inputs/day14.txt").unwrap();

    // PART 1
    let start = Instant::now();
    let known_answer = "13105044880745";
    let part_1: usize = sum_memory(&input);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "3505392154485";
    let part_2: usize = sum_memory_2(&input);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input: &str =
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11\nmem[7] = 101\nmem[8] = 0";
        let answer: usize = sum_memory(&input);
        assert_eq!(answer, 165);
    }
    #[test]
    fn test_example_2() {
        let input: &str = "mask = 000000000000000000000000000000X1001X\nmem[42] = 100\nmask = 00000000000000000000000000000000X0XX\nmem[26] = 1";
        let answer: usize = sum_memory_2(&input);
        assert_eq!(answer, 208);
    }
}
