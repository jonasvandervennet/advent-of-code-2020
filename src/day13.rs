use crate::util::{print_part_1, print_part_2};
use std::fs::read_to_string;
use std::time::Instant;

fn schedule_bus(input: &str) -> usize {
    let mut lines = input.lines();
    let estimated_departure: usize = lines
        .next()
        .expect("Invalid input..")
        .parse::<usize>()
        .expect("Invalid number..");
    let bus_ids: Vec<usize> = lines
        .next()
        .expect("Invalid input..")
        .split(",")
        .filter(|&x| x != "x")
        .map(|x| x.parse::<usize>().expect("Invalid number.."))
        .collect();

    let mut waiting_time = 0;
    loop {
        waiting_time += 1;
        let departure = estimated_departure + waiting_time;
        for bus_id in bus_ids.iter() {
            if departure % bus_id == 0 {
                return bus_id * waiting_time;
            }
        }
    }
}

fn win_contest(input: &str) -> usize {
    let mut lines = input.lines();
    lines.next(); // skip first line of input
    let bus_ids: Vec<usize> = lines
        .next()
        .expect("Invalid input..")
        .split(",")
        .map(|x| {
            if x != "x" {
                x.parse::<usize>().expect("Invalid number..")
            } else {
                0
            }
        })
        .collect();

    /*
    SOLVE
    x       = 0 mod bus_1
    x + 1   = 0 mod bus_2
    => Chinese remainder theorem
    ...

    */
    let mut history = bus_ids[0];
    let mut x = 0;
    let mut diff = 0;
    for bus_id in &bus_ids[1..] {
        diff += 1;
        if *bus_id == 0 {
            continue;
        }
        loop {
            x += history;
            if (x + diff) % bus_id == 0 {
                break;
            }
        }
        history *= bus_id;
    }
    x
}

pub fn main() {
    let input = read_to_string("inputs/day13.txt").expect("Input not found..");

    // PART 1
    let start = Instant::now();
    let known_answer = "2406";
    let part_1: usize = schedule_bus(&input);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "225850756401039";
    let part_2: usize = win_contest(&input);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input: &str = "939\n7,13,x,x,59,x,31,19";
        let answer: usize = schedule_bus(&input);
        assert_eq!(answer, 295);
    }

    #[test]
    fn test_example_2() {
        let input: &str = "939\n7,13,x,x,59,x,31,19";
        let answer: usize = win_contest(&input);
        assert_eq!(answer, 1068781);
    }

    #[test]
    fn test_example_3() {
        let input: &str = "939\n17,x,13,19";
        let answer: usize = win_contest(&input);
        assert_eq!(answer, 3417);
    }

    #[test]
    fn test_example_4() {
        let input: &str = "939\n67,7,59,61";
        let answer: usize = win_contest(&input);
        assert_eq!(answer, 754018);
    }

    #[test]
    fn test_example_5() {
        let input: &str = "939\n67,x,7,59,61";
        let answer: usize = win_contest(&input);
        assert_eq!(answer, 779210);
    }

    #[test]
    fn test_example_6() {
        let input: &str = "939\n67,7,x,59,61";
        let answer: usize = win_contest(&input);
        assert_eq!(answer, 1261476);
    }

    #[test]
    fn test_example_7() {
        let input: &str = "939\n1789,37,47,1889";
        let answer: usize = win_contest(&input);
        assert_eq!(answer, 1202161486);
    }
}
