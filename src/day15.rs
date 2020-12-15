use crate::util::{print_part_1, print_part_2};
use std::collections::HashMap;
use std::time::Instant;

fn play_repeat(input: &str, goal_iterations: usize) -> usize {
    let mut spoken_numbers_history: HashMap<usize, usize> = HashMap::new();

    let mut num_spoken = 0;
    let mut last_num = 0;
    let mut last_num_new = true;
    let mut age = 0;

    for n in input.split(",") {
        let n = n.parse::<usize>().unwrap();

        spoken_numbers_history.insert(n, num_spoken + 1);
        num_spoken += 1;
        last_num = n;
    }

    while num_spoken != goal_iterations {
        if last_num_new {
            // say 0
            last_num = 0;
        } else {
            // use age
            last_num = num_spoken - age;
        }
        match spoken_numbers_history.insert(last_num, num_spoken + 1) {
            None => last_num_new = true,
            Some(n) => {
                age = n;
                last_num_new = false;
            }
        }
        num_spoken += 1;
    }
    last_num
}

pub fn main() {
    let input = "0,13,16,17,1,10,6";

    // PART 1
    let start = Instant::now();
    let known_answer = "276";
    let part_1: usize = play_repeat(&input, 2020);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "31916";
    let part_2: usize = play_repeat(&input, 30000000);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1_1() {
        let input: &str = "0,3,6";
        let answer: usize = play_repeat(&input, 2020);
        assert_eq!(answer, 436);
    }

    #[test]
    fn test_example_1_2() {
        let input: &str = "1,3,2";
        let answer: usize = play_repeat(&input, 2020);
        assert_eq!(answer, 1);
    }

    #[test]
    fn test_example_1_3() {
        let input: &str = "2,1,3";
        let answer: usize = play_repeat(&input, 2020);
        assert_eq!(answer, 10);
    }

    #[test]
    fn test_example_1_4() {
        let input: &str = "1,2,3";
        let answer: usize = play_repeat(&input, 2020);
        assert_eq!(answer, 27);
    }

    #[test]
    fn test_example_1_5() {
        let input: &str = "2,3,1";
        let answer: usize = play_repeat(&input, 2020);
        assert_eq!(answer, 78);
    }

    #[test]
    fn test_example_1_6() {
        let input: &str = "3,2,1";
        let answer: usize = play_repeat(&input, 2020);
        assert_eq!(answer, 438);
    }

    #[test]
    fn test_example_1_7() {
        let input: &str = "3,1,2";
        let answer: usize = play_repeat(&input, 2020);
        assert_eq!(answer, 1836);
    }

    // TAKES SOME TIME TO RUN THESE

    // #[test]
    // fn test_example_2_1() {
    //     let input: &str = "0,3,6";
    //     let answer: usize = play_repeat(&input, 30000000);
    //     assert_eq!(answer, 175594);
    // }

    // #[test]
    // fn test_example_2_2() {
    //     let input: &str = "1,3,2";
    //     let answer: usize = play_repeat(&input, 30000000);
    //     assert_eq!(answer, 2578);
    // }

    // #[test]
    // fn test_example_2_3() {
    //     let input: &str = "2,1,3";
    //     let answer: usize = play_repeat(&input, 30000000);
    //     assert_eq!(answer, 3544142);
    // }

    // #[test]
    // fn test_example_2_4() {
    //     let input: &str = "1,2,3";
    //     let answer: usize = play_repeat(&input, 30000000);
    //     assert_eq!(answer, 261214);
    // }

    // #[test]
    // fn test_example_2_5() {
    //     let input: &str = "2,3,1";
    //     let answer: usize = play_repeat(&input, 30000000);
    //     assert_eq!(answer, 6895259);
    // }

    // #[test]
    // fn test_example_2_6() {
    //     let input: &str = "3,2,1";
    //     let answer: usize = play_repeat(&input, 30000000);
    //     assert_eq!(answer, 18);
    // }

    // #[test]
    // fn test_example_2_7() {
    //     let input: &str = "3,1,2";
    //     let answer: usize = play_repeat(&input, 30000000);
    //     assert_eq!(answer, 362);
    // }
}
