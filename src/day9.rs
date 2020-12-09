use crate::util::{print_part_1, print_part_2};
use std::collections::VecDeque;
use std::fs::read_to_string;
use std::time::Instant;

fn exists_sum(options: &VecDeque<&usize>, target: &usize) -> bool {
    for x in options {
        for y in options {
            if *x + *y == *target {
                return true;
            }
        }
    }
    false
}

fn find_broken_sequence(sequence: &Vec<usize>, preamble_size: usize) -> usize {
    let mut history: VecDeque<&usize> = VecDeque::with_capacity(preamble_size);

    for num in sequence[..preamble_size].iter() {
        history.push_back(num);
    }

    for num in sequence[preamble_size..].iter() {
        if !exists_sum(&history, num) {
            return *num;
        }
        history.pop_front();
        history.push_back(num);
    }

    0
}

fn contiguous_bounds_to_sum(sequence: &Vec<usize>, target: usize) -> (usize, usize) {
    let mut history: VecDeque<usize> = VecDeque::new();
    for num in sequence {
        let num = *num;
        history.push_back(num);
        let mut sum: usize = history.iter().sum();
        while sum > target {
            // trim contiguous sequence to bare minimum after adding new number
            history.pop_front();
            sum = history.iter().sum();
        }
        if sum == target {
            return (
                *history.iter().min().unwrap(),
                *history.iter().max().unwrap(),
            );
        }
    }
    (0, 0)
}

pub fn main() {
    let input = read_to_string("inputs/day9.txt").unwrap();
    let input: Vec<usize> = input
        .lines()
        .map(|line| line.parse::<usize>().expect("Could not parse input.."))
        .collect();
    // PART 1
    let start = Instant::now();
    let known_answer = "14144619";
    let part_1: usize = find_broken_sequence(&input, 25);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "1766397";
    let (min, max): (usize, usize) = contiguous_bounds_to_sum(&input, part_1);
    let part_2 = min + max;
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input: &str = "35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576";
        let input: Vec<usize> = input
            .lines()
            .map(|line| line.parse::<usize>().expect("Could not parse input.."))
            .collect();
        let answer: usize = find_broken_sequence(&input, 5);
        assert_eq!(answer, 127);
    }

    #[test]
    fn test_example_2() {
        let input: &str = "35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576";
        let input: Vec<usize> = input
            .lines()
            .map(|line| line.parse::<usize>().expect("Could not parse input.."))
            .collect();
        let (min, max): (usize, usize) = contiguous_bounds_to_sum(&input, 127);
        println!("{}, {}", min, max);
        let answer = min + max;
        assert_eq!(answer, 62);
    }
}
