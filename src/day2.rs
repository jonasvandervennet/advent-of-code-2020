use crate::util::{print_part_1, print_part_2};
use std::fs::read_to_string;
use std::time::Instant;

fn verify_password_part1(input: &str) -> bool {
    let low: usize = input.split('-').collect::<Vec<&str>>()[0]
        .parse()
        .unwrap_or(0);
    let high: usize = input.split('-').collect::<Vec<&str>>()[1]
        .split_whitespace()
        .collect::<Vec<&str>>()[0]
        .parse()
        .unwrap_or(0);
    let target: char = match input.split(':').collect::<Vec<&str>>()[0]
        .split_whitespace()
        .collect::<Vec<&str>>()[1]
        .chars()
        .next()
    {
        Some(c) => c,
        None => return false,
    };
    let password: &str = &input.split(':').collect::<Vec<&str>>()[1][1..];

    let count = password.matches(target).count();
    low <= count && count <= high
}

fn verify_password_part2(input: &str) -> bool {
    let low: usize = input.split('-').collect::<Vec<&str>>()[0]
        .parse()
        .unwrap_or(0);
    let high: usize = input.split('-').collect::<Vec<&str>>()[1]
        .split_whitespace()
        .collect::<Vec<&str>>()[0]
        .parse()
        .unwrap_or(0);
    let target: char = match input.split(':').collect::<Vec<&str>>()[0]
        .split_whitespace()
        .collect::<Vec<&str>>()[1]
        .chars()
        .next()
    {
        Some(c) => c,
        None => return false,
    };
    let password: &str = &input.split(':').collect::<Vec<&str>>()[1][1..];

    (password.chars().nth(low - 1).unwrap() == target)
        ^ (password.chars().nth(high - 1).unwrap() == target)
}

pub fn main() {
    let input = read_to_string("inputs/day2.txt").expect("Input not found..");
    // PART 1
    let start = Instant::now();
    let known_answer = "666";
    let part_1: usize = input
        .lines()
        .map(|line| if verify_password_part1(line) { 1 } else { 0 })
        .sum();
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "670";
    let part_2: usize = input
        .lines()
        .map(|line| if verify_password_part2(line) { 1 } else { 0 })
        .sum();
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "1-3 a: abcde";
        let valid: bool = verify_password_part1(&input);
        assert_eq!(valid, true);
    }

    #[test]
    fn test_example_2() {
        let input = "1-3 b: cdefg";
        let valid: bool = verify_password_part1(&input);
        assert_eq!(valid, false);
    }

    #[test]
    fn test_example_3() {
        let input = "2-9 c: ccccccccc";
        let valid: bool = verify_password_part1(&input);
        assert_eq!(valid, true);
    }

    #[test]
    fn test_example_4() {
        let input = "1-3 a: abcde";
        let valid: bool = verify_password_part2(&input);
        assert_eq!(valid, true);
    }

    #[test]
    fn test_example_5() {
        let input = "1-3 b: cdefg";
        let valid: bool = verify_password_part2(&input);
        assert_eq!(valid, false);
    }

    #[test]
    fn test_example_6() {
        let input = "2-9 c: ccccccccc";
        let valid: bool = verify_password_part2(&input);
        assert_eq!(valid, false);
    }
}
