use crate::util::{print_part_1, print_part_2};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::time::Instant;

fn booting_sequence(input: &str, dim: usize) -> usize {
    let mut actives: HashSet<Vec<i64>> = HashSet::new();

    // input parsing
    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c == '#' {
                let mut v = vec![x as i64, y as i64];
                v.resize(dim, 0); // pad zeros until size=dim is reached
                actives.insert(v);
            }
        }
    }

    // 6 boot cycles
    for _ in 0..6 {
        let mut active_neighbours: HashMap<Vec<i64>, usize> = HashMap::new();
        let mut new_actives: HashSet<Vec<i64>> = HashSet::new();
        for orig_coords in actives.iter() {
            let mut n = 0;
            for offsets in vec![[-1, 0, 1]; dim]
                .iter()
                .multi_cartesian_product()
                .filter(|o| o.iter().any(|&&x| x != 0))
            {
                let coords: Vec<i64> = orig_coords
                    .iter()
                    .zip(offsets)
                    .map(|(a, b)| a + b)
                    .collect();

                if actives.contains(&coords) {
                    n += 1;
                } else {
                    active_neighbours
                        .entry(coords)
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                }
            }
            if n == 2 || n == 3 {
                new_actives.insert(orig_coords.to_vec());
            }
        }

        for (coord, num_active) in active_neighbours {
            if num_active == 3 {
                new_actives.insert(coord);
            }
        }
        actives = new_actives;
    }
    actives.len()
}

pub fn main() {
    let input = read_to_string("inputs/day17.txt").unwrap();

    // PART 1
    let start = Instant::now();
    let known_answer = "247";
    // let part_1: usize = booting_sequence_3d(&input);
    let part_1: usize = booting_sequence(&input, 3);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "1392";
    let part_2: usize = booting_sequence(&input, 4);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input: &str = ".#.\n..#\n###";
        let answer: usize = booting_sequence(&input, 3);
        assert_eq!(answer, 112);
    }

    #[test]
    fn test_example_2() {
        let input: &str = ".#.\n..#\n###";
        let answer: usize = booting_sequence(&input, 4);
        assert_eq!(answer, 848);
    }
}
