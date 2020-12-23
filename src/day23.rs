use crate::util::{print_part_1, print_part_2};
use std::time::Instant;

fn next_dest(curr: usize, min: usize, max: usize) -> usize {
    // custom overflow rules
    if curr == min {
        max
    } else {
        curr - 1
    }
}

// uses array where cup_nr are indices and values are next_index
// => faster lookup of the destination
fn play_cups(cups: &mut [usize], start: usize, n_moves: usize) -> Vec<usize> {
    let min_cup = 1;
    let max_cup = *cups.iter().max().unwrap();

    let mut curr_value = start;

    for _ in 0..n_moves {
        let pickup_value1 = cups[curr_value];

        // pick up three cups
        let pickup_value2 = cups[pickup_value1];
        let pickup_value3 = cups[pickup_value2];
        let after_pickups = cups[pickup_value3];
        let pickup_values = [pickup_value1, pickup_value2, pickup_value3];

        // wrap around picked up values
        cups[curr_value] = after_pickups;

        // find destination
        let mut destination = next_dest(curr_value, min_cup, max_cup);
        while pickup_values.contains(&destination) {
            destination = next_dest(destination, min_cup, max_cup);
        }
        let after_dest_index = cups[destination];

        // plug in picked up cups after the destination
        cups[destination] = pickup_value1; // destination links to first picked up value
        cups[pickup_value3] = after_dest_index; // last picked up value links to value after destination

        // update current index for next iteration
        curr_value = cups[curr_value];
    }

    // create chain starting after cup 1
    let mut chain = Vec::new();
    chain.reserve(cups.len());
    let mut cup = cups[1];
    while cup != 1 {
        chain.push(cup); // keep track of values
        cup = cups[cup]; // next cup (don't include 1!)
    }
    chain
}

fn part1(input: &str, n_moves: usize) -> String {
    let mut cups = vec![0; input.chars().count() + 1];
    let input: Vec<usize> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    for window in input.windows(2) {
        cups[window[0]] = window[1];
    }
    cups[input[input.len() - 1]] = input[0];

    play_cups(&mut cups, input[0], n_moves)
        .iter()
        .map(|d| d.to_string())
        .collect::<Vec<String>>()
        .join("")
}

fn part2(input: &str, n_moves: usize) -> usize {
    let mut cups = vec![0; 1_000_001]; // stack overflows is this is an array :D
    let input: Vec<usize> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .chain(10..=1_000_000) // add all values from 10 to 1 million
        .collect();
    for window in input.windows(2) {
        cups[window[0]] = window[1];
    }
    cups[input[input.len() - 1]] = input[0];

    let chain = play_cups(&mut cups, input[0], n_moves);
    chain[0] * chain[1]
}

pub fn main() {
    let input = "315679824";

    // PART 1
    let start = Instant::now();
    let known_answer = "72496583";
    let part_1: String = part1(&input, 100);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "41785843847";
    let part_2: usize = part2(&input, 10_000_000);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input: &str = "389125467";
        let answer: String = part1(&input, 10);
        assert_eq!(answer, "92658374");
    }

    #[test]
    fn test_example_1_full() {
        let input: &str = "389125467";
        let answer: String = part1(&input, 100);
        assert_eq!(answer, "67384529");
    }

    #[test]
    fn test_example_2() {
        let input: &str = "389125467";
        let answer: usize = part2(&input, 10_000_000);
        assert_eq!(answer, 149245887792);
    }
}
