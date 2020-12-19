use crate::util::{print_part_1, print_part_2};
use std::fs::read_to_string;
use std::time::Instant;

fn patch_cables(input: &Vec<usize>) -> usize {
    let mut input = input.clone();
    input.sort();

    let mut diff_1 = 0;
    let mut diff_3 = 1; // last connection is always a +3 connection
    let mut prev = 0;
    for num in input {
        match num - prev {
            1 => diff_1 += 1,
            2 => {}
            3 => diff_3 += 1,
            _ => {}
        }
        prev = num;
    }
    diff_1 * diff_3
}

fn num_arrangements(input: &Vec<usize>) -> usize {
    // "more than a trillion ways to combine arrangements"
    // Try some dynamic programming approach
    let mut input = input.clone();
    if !input.contains(&0) {
        input.push(0);
    }
    input.sort();

    let start = input[0];
    let end = input[input.len() - 1];
    let mut options: Vec<usize> = vec![0; end + 1];
    options[start] = 1; // zero-connection has one pathway to start

    // From the start, iterate over all possible values
    for (i, prev) in input.iter().enumerate() {
        let prev = *prev;
        // iterate over all values reachable from this one
        for next in &input[i + 1..] {
            let next = *next;
            if next > prev + 3 {
                // ignore any value 3 higher due to incompatibility
                continue;
            } else {
                // keep track of the total amount of
                // possible pathways to reach this point
                options[next] += options[prev];
            }
        }
    }
    // return the amount of possible pathways to the end
    options[end]
}

pub fn main() {
    let input = read_to_string("inputs/day10.txt").expect("Input not found..");
    let input: Vec<usize> = input
        .lines()
        .map(|line| line.parse::<usize>().expect("Could not decode input.."))
        .collect();

    // PART 1
    let start = Instant::now();
    let known_answer = "2201";
    let part_1: usize = patch_cables(&input);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "169255295254528";
    let part_2 = num_arrangements(&input);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input: &str = "16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4";
        let input: Vec<usize> = input
            .lines()
            .map(|line| line.parse::<usize>().expect("Could not decode input.."))
            .collect();

        let answer: usize = patch_cables(&input);
        assert_eq!(answer, 7 * 5);
    }
    #[test]
    fn test_example_2() {
        let input: &str = "28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3";
        let input: Vec<usize> = input
            .lines()
            .map(|line| line.parse::<usize>().expect("Could not decode input.."))
            .collect();

        let answer: usize = patch_cables(&input);
        assert_eq!(answer, 22 * 10);
    }
    #[test]
    fn test_example_arrangements_1() {
        let input: &str = "16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4";
        let input: Vec<usize> = input
            .lines()
            .map(|line| line.parse::<usize>().expect("Could not decode input.."))
            .collect();

        let answer: usize = num_arrangements(&input);
        assert_eq!(answer, 8);
    }
    #[test]
    fn test_example_arrangements_2() {
        let input: &str = "28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3";
        let input: Vec<usize> = input
            .lines()
            .map(|line| line.parse::<usize>().expect("Could not decode input.."))
            .collect();

        let answer: usize = num_arrangements(&input);
        assert_eq!(answer, 19208);
    }
}
