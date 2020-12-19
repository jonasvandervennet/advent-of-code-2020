use crate::util::{print_part_1, print_part_2};
use std::fs::read_to_string;
use std::time::Instant;

fn follow_path_1(input: &str) -> i64 {
    let mut north: i64 = 0;
    let mut east: i64 = 0;
    let dirs = ['N', 'E', 'S', 'W'];
    let mut curr_dir: usize = 1;

    for line in input.lines() {
        let mut dir: char = line[..1].chars().nth(0).expect("Invalid input..");
        let amount: i64 = line[1..].parse::<i64>().expect("Invalid input..");

        if dir == 'F' {
            dir = dirs[curr_dir];
        }

        match dir {
            'N' => {
                north += amount;
            }
            'S' => {
                north -= amount;
            }
            'E' => {
                east += amount;
            }
            'W' => {
                east -= amount;
            }
            'L' => {
                curr_dir = (curr_dir - (amount as usize) / 90) % 4;
            }
            'R' => {
                curr_dir = (curr_dir + (amount as usize) / 90) % 4;
            }
            _ => unreachable!(),
        }
    }

    north.abs() + east.abs()
}

fn follow_path_2(input: &str) -> i64 {
    let mut north: i64 = 0;
    let mut east: i64 = 0;
    let mut waypoint = [1, 10];

    for line in input.lines() {
        let dir: char = line[..1].chars().nth(0).expect("Invalid input..");
        let amount: i64 = line[1..].parse::<i64>().expect("Invalid input..");

        match dir {
            'N' => {
                waypoint[0] += amount;
            }
            'S' => {
                waypoint[0] -= amount;
            }
            'E' => {
                waypoint[1] += amount;
            }
            'W' => {
                waypoint[1] -= amount;
            }
            'L' => {
                for _ in 0..amount / 90 {
                    waypoint = [waypoint[1], -waypoint[0]];
                }
            }
            'R' => {
                for _ in 0..amount / 90 {
                    waypoint = [-waypoint[1], waypoint[0]];
                }
            }
            'F' => {
                north += waypoint[0] * amount;
                east += waypoint[1] * amount;
            }
            _ => unreachable!(),
        }
    }

    north.abs() + east.abs()
}

pub fn main() {
    let input = read_to_string("inputs/day12.txt").expect("Input not found..");

    // PART 1
    let start = Instant::now();
    let known_answer = "415";
    let part_1: i64 = follow_path_1(&input);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "29401";
    let part_2: i64 = follow_path_2(&input);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input: &str = "F10\nN3\nF7\nR90\nF11";
        let answer: i64 = follow_path_1(&input);
        assert_eq!(answer, 25);
    }

    #[test]
    fn test_example_2() {
        let input: &str = "F10\nN3\nF7\nR90\nF11";
        let answer: i64 = follow_path_2(&input);
        assert_eq!(answer, 286);
    }
}
