use crate::util::{print_part_1, print_part_2};
use std::fs::read_to_string;
use std::time::Instant;

fn sum_memory(input: &str) -> usize {
    0
}

pub fn main() {
    let input = read_to_string("inputs/day21.txt").expect("Input not found..");

    // PART 1
    let start = Instant::now();
    let known_answer = "13105044880745";
    let part_1: usize = sum_memory(&input);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    // let start = Instant::now();
    // let known_answer = "3505392154485";
    // let part_2: usize = sum_memory_2(&input);
    // let duration = start.elapsed();
    // print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input: &str = "";
        let answer: usize = sum_memory(&input);
        assert_eq!(answer, 165);
    }
}
