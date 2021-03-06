use crate::util::{print_part_1, print_part_2};
use std::fs::read_to_string;
use std::time::Instant;

fn count_trees_slope(map: &str, right: usize, down: usize) -> usize {
    let mut x: usize = 0; // current offset in line
    let mut y: usize = 0; // current line number

    let width: usize = map.lines().nth(0).unwrap().chars().count();
    let y_size: usize = map.lines().count();

    let mut tree_count: usize = 0;
    while y < y_size {
        let curr: char = map.lines().nth(y).unwrap().chars().nth(x).unwrap();
        if curr == '#' {
            tree_count += 1;
        }
        x = (x + right) % width;
        y = y + down;
    }
    tree_count
}

pub fn main() {
    let map = read_to_string("inputs/day3.txt").expect("Input not found..");
    // PART 1
    let start = Instant::now();
    let known_answer = "252";
    let part_1: usize = count_trees_slope(&map, 3, 1);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "2608962048";
    let part_2: usize = count_trees_slope(&map, 1, 1)
        * count_trees_slope(&map, 3, 1)
        * count_trees_slope(&map, 5, 1)
        * count_trees_slope(&map, 7, 1)
        * count_trees_slope(&map, 1, 2);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1_1() {
        let input = "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#";
        let trees: usize = count_trees_slope(&input, 1, 1);
        assert_eq!(trees, 2);
    }

    #[test]
    fn test_example_3_1() {
        let input = "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#";
        let trees: usize = count_trees_slope(&input, 3, 1);
        assert_eq!(trees, 7);
    }

    #[test]
    fn test_example_5_1() {
        let input = "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#";
        let trees: usize = count_trees_slope(&input, 5, 1);
        assert_eq!(trees, 3);
    }

    #[test]
    fn test_example_7_1() {
        let input = "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#";
        let trees: usize = count_trees_slope(&input, 7, 1);
        assert_eq!(trees, 4);
    }

    #[test]
    fn test_example_1_2() {
        let input = "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#";
        let trees: usize = count_trees_slope(&input, 1, 2);
        assert_eq!(trees, 2);
    }

    #[test]
    fn test_example_part_2() {
        let input = "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#";
        let trees: usize = count_trees_slope(&input, 1, 1)
            * count_trees_slope(&input, 3, 1)
            * count_trees_slope(&input, 5, 1)
            * count_trees_slope(&input, 7, 1)
            * count_trees_slope(&input, 1, 2);
        assert_eq!(trees, 336);
    }
}
