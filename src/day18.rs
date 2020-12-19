use crate::util::{print_part_1, print_part_2};
use std::collections::VecDeque;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Clone, Copy)]
struct State {
    lhs: usize,
    op: char,

    initialised: bool,
    parantheses_state: bool, // reason for push
}

impl State {
    fn new(by_parentheses: bool) -> Self {
        State {
            initialised: false,
            parantheses_state: by_parentheses,
            lhs: 0,
            op: '+',
        }
    }
    fn do_op(&mut self, rhs: usize) {
        if !self.initialised {
            self.initialised = true;
            self.lhs = rhs;
            return;
        }
        match self.op {
            '+' => {
                self.lhs += rhs;
            }
            '*' => {
                self.lhs *= rhs;
            }
            _ => {
                panic!("What is happening?");
            }
        }
    }

    fn value(&self) -> usize {
        self.lhs
    }
}

fn evaluate_expression(expr: &str, part: usize) -> usize {
    let mut states: VecDeque<State> = VecDeque::new();
    let mut curr_state = State::new(false);

    // put everything between a set of parentheses
    let expr = format!("({})", expr);

    for c in expr.chars() {
        match c {
            ' ' => {
                continue;
            }
            '+' => {
                curr_state.op = c;
            }
            '*' => {
                curr_state.op = c;
                if part == 2 {
                    // addition has precedence over muiltiplication
                    // so hold off on performing the mult for now
                    states.push_back(curr_state);
                    curr_state = State::new(false);
                }
            }
            '(' => {
                states.push_back(curr_state);
                curr_state = State::new(true);
            }
            ')' => loop {
                // reduce for all values inside the same parentheses
                let rhs = curr_state.value();
                let parentheses_state = curr_state.parantheses_state;
                curr_state = states.pop_back().unwrap();
                curr_state.do_op(rhs);
                if parentheses_state {
                    break;
                }
            },
            _ => {
                let digit = c.to_digit(10).unwrap() as usize;
                curr_state.do_op(digit);
            }
        }
    }
    curr_state.value() as usize
}

fn sum_expressions(input: &str, part: usize) -> usize {
    input
        .lines()
        .map(|line| evaluate_expression(line, part))
        .sum()
}

pub fn main() {
    let input = read_to_string("inputs/day18.txt").expect("Input not found..");

    // PART 1
    let start = Instant::now();
    let known_answer = "1451467526514";
    let part_1: usize = sum_expressions(&input, 1);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "224973686321527";
    let part_2: usize = sum_expressions(&input, 2);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input: &str = "1 + 2 * 3 + 4 * 5 + 6";
        let answer = evaluate_expression(&input, 1);
        assert_eq!(answer, 71);
    }
    #[test]
    fn test_example_2() {
        let input: &str = "1 + (2 * 3) + (4 * (5 + 6))";
        let answer = evaluate_expression(&input, 1);
        assert_eq!(answer, 51);
    }
    #[test]
    fn test_example_3() {
        let input: &str = "2 * 3 + (4 * 5)";
        let answer = evaluate_expression(&input, 1);
        assert_eq!(answer, 26);
    }
    #[test]
    fn test_example_4() {
        let input: &str = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
        let answer = evaluate_expression(&input, 1);
        assert_eq!(answer, 437);
    }
    #[test]
    fn test_example_5() {
        let input: &str = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        let answer = evaluate_expression(&input, 1);
        assert_eq!(answer, 12240);
    }
    #[test]
    fn test_example_6() {
        let input: &str = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let answer = evaluate_expression(&input, 1);
        assert_eq!(answer, 13632);
    }

    #[test]
    fn test_example_2_0() {
        let input: &str = "4 * 9 + 3";
        let answer = evaluate_expression(&input, 2);
        assert_eq!(answer, 48);
    }
    #[test]
    fn test_example_2_1() {
        let input: &str = "1 + 2 * 3 + 4 * 5 + 6";
        let answer = evaluate_expression(&input, 2);
        assert_eq!(answer, 231);
    }
    #[test]
    fn test_example_2_2() {
        let input: &str = "1 + (2 * 3) + (4 * (5 + 6))";
        let answer = evaluate_expression(&input, 2);
        assert_eq!(answer, 51);
    }
    #[test]
    fn test_example_2_3() {
        let input: &str = "2 * 3 + (4 * 5)";
        let answer = evaluate_expression(&input, 2);
        assert_eq!(answer, 46);
    }
    #[test]
    fn test_example_2_4() {
        let input: &str = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
        let answer = evaluate_expression(&input, 2);
        assert_eq!(answer, 1445);
    }
    #[test]
    fn test_example_2_5() {
        let input: &str = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        let answer = evaluate_expression(&input, 2);
        assert_eq!(answer, 669060);
    }
    #[test]
    fn test_example_2_6() {
        let input: &str = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let answer = evaluate_expression(&input, 2);
        assert_eq!(answer, 23340);
    }
}
