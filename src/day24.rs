use crate::util::{print_part_1, print_part_2};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

//                                             NORTH  EAST
fn reduce_directions(directions: Vec<&str>) -> (i64, i64) {
    let mut coords = (0, 0);

    // calculate coordinate by counting steps north and east.
    // Cardinal directions count double, as they take greater strides
    for &dir in directions.iter() {
        match dir {
            "e" => coords.1 += 2,
            "w" => coords.1 -= 2,
            "ne" => {
                coords.0 += 1;
                coords.1 += 1
            }
            "nw" => {
                coords.0 += 1;
                coords.1 -= 1
            }
            "se" => {
                coords.0 -= 1;
                coords.1 += 1
            }
            "sw" => {
                coords.0 -= 1;
                coords.1 -= 1
            }
            _ => unreachable!(),
        }
    }
    coords
}

fn get_initial_flips(input: &str) -> Vec<(i64, i64)> {
    // directions need to be translated to a generalized form
    // in order to decide which lines influence the same tile!

    let directions_list: Vec<(i64, i64)> = input
        .lines()
        .map(|line| {
            let mut directions = Vec::new();
            let mut prev = line.chars().nth(0).unwrap();
            if prev == 'e' {
                directions.push("e");
            } else if prev == 'w' {
                directions.push("w");
            }
            for c in line.chars().skip(1) {
                match c {
                    'e' => {
                        if prev == 's' {
                            directions.push("se");
                        } else if prev == 'n' {
                            directions.push("ne");
                        } else {
                            directions.push("e");
                        }
                    }
                    'w' => {
                        if prev == 's' {
                            directions.push("sw");
                        } else if prev == 'n' {
                            directions.push("nw");
                        } else {
                            directions.push("w");
                        }
                    }
                    _ => {}
                }
                prev = c;
            }
            directions
        })
        .map(|d| reduce_directions(d))
        .collect();

    // Collect all reached coordinates modulo 2
    let mut flips: HashMap<_, bool> = HashMap::new();
    for directions in directions_list.iter() {
        flips
            .entry(directions)
            .and_modify(|e| *e ^= true)
            .or_insert(true);
    }

    // Only keep those that were flipped an odd amount of times
    let mut flipped = Vec::new();
    for (&tile, flip) in flips {
        if flip {
            flipped.push(tile);
        }
    }
    flipped
}

fn iterate_flips(flipped: &Vec<(i64, i64)>, iterations: usize) -> Vec<(i64, i64)> {
    let mut flipped = flipped.to_vec();
    for _ in 0..iterations {
        // Standard Game Of Life iteration structure
        let mut new_flipped = Vec::new();
        let mut neighbours_count: HashMap<(i64, i64), usize> = HashMap::new();
        for tile in flipped.iter() {
            for (x, y) in &[(0, -2), (0, 2), (1, 1), (-1, -1), (1, -1), (-1, 1)] {
                let neighbour = (tile.0 + x, tile.1 + y);
                neighbours_count
                    .entry(neighbour)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        }
        for (tile, count) in neighbours_count {
            let valid_flipped = flipped.contains(&tile) && (count == 1 || count == 2);
            let new_flip = !flipped.contains(&tile) && count == 2;
            if valid_flipped || new_flip {
                new_flipped.push(tile);
            }
        }
        flipped = new_flipped;
    }
    flipped
}

fn part1(input: &str) -> usize {
    get_initial_flips(input).len()
}

fn part2(input: &str, iterations: usize) -> usize {
    let initial = get_initial_flips(input);
    iterate_flips(&initial, iterations).len()
}

pub fn main() {
    let input = read_to_string("inputs/day24.txt").expect("Input not found..");

    // PART 1
    let start = Instant::now();
    let known_answer = "438";
    let part_1: usize = part1(&input);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "4038";
    let part_2: usize = part2(&input, 100);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = read_to_string("inputs/day24_test.txt").expect("Input not found..");
        let answer: usize = part1(&input);
        assert_eq!(answer, 10);
    }

    #[test]
    fn test_example_reduce() {
        let input = vec!["nw", "w", "sw", "e", "e"];
        let answer = reduce_directions(input);
        assert_eq!(answer, (0, 0));
    }
    #[test]
    fn test_example_iterate_1() {
        let input = read_to_string("inputs/day24_test.txt").expect("Input not found..");
        let answer: usize = part2(&input, 1);
        assert_eq!(answer, 15);
    }
    #[test]
    fn test_example_iterate_10() {
        let input = read_to_string("inputs/day24_test.txt").expect("Input not found..");
        let answer: usize = part2(&input, 10);
        assert_eq!(answer, 37);
    }
    #[test]
    fn test_example_iterate_50() {
        let input = read_to_string("inputs/day24_test.txt").expect("Input not found..");
        let answer: usize = part2(&input, 50);
        assert_eq!(answer, 566);
    }
}
