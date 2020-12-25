use crate::util::print_part_1;
use std::time::Instant;

fn encryption_key(card: usize, door: usize) -> usize {
    let mut card_loop_size = 0;
    let mut value = 1;
    while card != value {
        value = (value * 7) % 20201227;
        card_loop_size += 1;
    }

    let mut key = 1;
    for _ in 0..card_loop_size {
        key = (key * door) % 20201227;
    }
    key
}

pub fn main() {
    let public_key_card = 3418282;
    let public_key_door = 8719412;

    // PART 1
    let start = Instant::now();
    let known_answer = "9620012";
    let part_1: usize = encryption_key(public_key_card, public_key_door);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let public_key_card = 5764801;
        let public_key_door = 17807724;
        let answer: usize = encryption_key(public_key_card, public_key_door);
        assert_eq!(answer, 14897079);
    }
}
