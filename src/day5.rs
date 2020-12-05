use std::fs::read_to_string;

// Seat is specified by a binary number
// R and B are 1, L and F are 0.
// seat_id is the converted number in decimal
// due to the row*8+col calculation.
fn seat_id(line: &str) -> usize {
    const BASE: usize = 2; // an explicit type is required
    let mut id = 0;

    for (i, c) in line.chars().enumerate() {
        if c == 'B' || c == 'R' {
            id += BASE.pow(9 - i as u32);
        };
    }
    id
}

fn get_missing_seat_id(seat_ids: Vec<usize>) -> usize {
    let before = seat_ids.iter().nth(0).unwrap();
    let mut seat = seat_ids.iter().nth(1).unwrap();

    if before != &(seat - 1) {
        // edge case, only have to check 'seat + 1' with 'after'
        return seat - 1;
    }

    for after in seat_ids[2..].iter() {
        if after != &(seat + 1) {
            return seat + 1;
        }
        seat = after;
    }
    0
}

pub fn main() {
    let input = read_to_string("inputs/day5.txt").unwrap();
    let mut seat_ids: Vec<usize> = input.lines().map(|line| seat_id(line)).collect();

    // PART 1
    let known_answer: usize = 963;
    let part_1: usize = seat_ids.iter().max().unwrap().to_owned();
    if part_1 != known_answer {
        print!("INCORRECT || ")
    }
    println!("PART 1: {}", part_1);

    // PART 2
    let known_answer: usize = 592;
    seat_ids.sort();
    let part_2: usize = get_missing_seat_id(seat_ids);
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
        let input: &str = "FBFBBFFRLR";
        let valid: usize = seat_id(&input);
        assert_eq!(valid, 357);
    }
}
