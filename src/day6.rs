use std::collections::HashSet;
use std::fs::read_to_string;

fn get_unique_chars(answers: &str) -> HashSet<char> {
    let mut chars: HashSet<char> = HashSet::new();

    for answer in answers.lines() {
        for c in answer.chars() {
            if !chars.contains(&c) {
                chars.insert(c);
            }
        }
    }
    chars
}

fn count_common_chars(answers: &str) -> usize {
    let mut chars: HashSet<char> = HashSet::new();

    let mut first_anwser = true;
    for answer in answers.lines() {
        if first_anwser {
            first_anwser = false;
            chars = get_unique_chars(answer);
        } else {
            let new_chars = get_unique_chars(answer);
            chars.retain(|&x| new_chars.contains(&x));
        }
    }
    chars.len()
}

pub fn main() {
    let input = read_to_string("inputs/day6.txt").unwrap();

    // PART 1
    let known_answer: usize = 6930;
    let part_1: usize = input
        .split("\r\n\r\n") // empty lines (this probably depends on the operating system..)
        .map(|line_group| get_unique_chars(line_group).len())
        .sum();
    if part_1 != known_answer {
        print!("INCORRECT || ")
    }
    println!("PART 1: {}", part_1);

    // PART 2
    let known_answer: usize = 3585;
    let part_2: usize = input
        .split("\r\n\r\n") // empty lines (this probably depends on the operating system..)
        .map(|line_group| count_common_chars(line_group))
        .sum();
    if part_2 != known_answer {
        print!("INCORRECT || ")
    }
    println!("PART 2: {}", part_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input: &str = "abc";
        let valid: usize = get_unique_chars(&input).len();
        assert_eq!(valid, 3);
    }

    #[test]
    fn test_example_2() {
        let input: &str = "a\nb\nc";
        let valid: usize = get_unique_chars(&input).len();
        assert_eq!(valid, 3);
    }

    #[test]
    fn test_example_3() {
        let input: &str = "ab\nac";
        let valid: usize = get_unique_chars(&input).len();
        assert_eq!(valid, 3);
    }

    #[test]
    fn test_example_4() {
        let input: &str = "a\na\na\na";
        let valid: usize = get_unique_chars(&input).len();
        assert_eq!(valid, 1);
    }

    #[test]
    fn test_example_5() {
        let input: &str = "b";
        let valid: usize = get_unique_chars(&input).len();
        assert_eq!(valid, 1);
    }

    #[test]
    fn test_example_6() {
        let input: &str = "abc";
        let valid: usize = count_common_chars(&input);
        assert_eq!(valid, 3);
    }

    #[test]
    fn test_example_7() {
        let input: &str = "a\nb\nc";
        let valid: usize = count_common_chars(&input);
        assert_eq!(valid, 0);
    }

    #[test]
    fn test_example_8() {
        let input: &str = "ab\nac";
        let valid: usize = count_common_chars(&input);
        assert_eq!(valid, 1);
    }

    #[test]
    fn test_example_9() {
        let input: &str = "a\na\na\na";
        let valid: usize = count_common_chars(&input);
        assert_eq!(valid, 1);
    }

    #[test]
    fn test_example_10() {
        let input: &str = "b";
        let valid: usize = count_common_chars(&input);
        assert_eq!(valid, 1);
    }
}
