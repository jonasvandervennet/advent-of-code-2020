use crate::util::{print_part_1, print_part_2};
use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;
use std::time::Instant;

fn winner_score(deck: &VecDeque<usize>) -> usize {
    let mut score = 0;

    let size = deck.len();
    for (i, card) in deck.iter().enumerate() {
        score += card * (size - i);
    }

    score
}

fn combat(p1: &mut VecDeque<usize>, p2: &mut VecDeque<usize>, recursive: bool) -> (usize, usize) {
    let mut p1_history: HashSet<VecDeque<usize>> = HashSet::new();
    let mut p2_history: HashSet<VecDeque<usize>> = HashSet::new();
    while p1.len() > 0 && p2.len() > 0 {
        // only have to check infinity rule if recursion is enabled
        if recursive {
            // check history for this exact matchup
            if p1_history.contains(p1) && p2_history.contains(p2) {
                return (1, winner_score(p1));
            }
            p1_history.insert(p1.iter().map(|&c| c).collect());
            p2_history.insert(p2.iter().map(|&c| c).collect());
        }

        let card1 = p1.pop_front().unwrap();
        let card2 = p2.pop_front().unwrap();

        let winner = if recursive && p1.len() >= card1 && p2.len() >= card2 {
            // run a sub game
            // number of cards drawn in copy is equal to card drawn just now!
            let p1_copy: &mut VecDeque<usize> = &mut p1.iter().map(|&c| c).collect();
            p1_copy.resize(card1, 0); // this will always shrink, as we checked the length beforehand
            let p2_copy: &mut VecDeque<usize> = &mut p2.iter().map(|&c| c).collect();
            p2_copy.resize(card2, 0);
            combat(p1_copy, p2_copy, true).0
        } else {
            if card1 > card2 {
                1
            } else {
                2
            }
        };
        if winner == 1 {
            p1.push_back(card1);
            p1.push_back(card2);
        } else {
            p2.push_back(card2);
            p2.push_back(card1);
        }
    }

    // Select winner deck
    if p2.len() == 0 {
        (1, winner_score(p1))
    } else {
        (2, winner_score(p2))
    }
}

fn parse_input(input: &str) -> (VecDeque<usize>, VecDeque<usize>) {
    let mut parts = input.split("\r\n\r\n");

    let part1 = parts.next().unwrap();
    let deck1 = part1
        .lines()
        .filter(|line| !line.starts_with("P"))
        .map(|d| d.parse().expect("invalid card.."))
        .collect();
    let part2 = parts.next().unwrap();
    let deck2 = part2
        .lines()
        .filter(|line| !line.starts_with("P"))
        .map(|d| d.parse().expect("invalid card.."))
        .collect();

    (deck1, deck2)
}

pub fn main() {
    let input = read_to_string("inputs/day22.txt").expect("Input not found..");
    let (p1, p2) = parse_input(&input);

    // PART 1
    let start = Instant::now();
    let known_answer = "30780";
    let part_1: usize = combat(
        &mut p1.iter().map(|&c| c).collect(),
        &mut p2.iter().map(|&c| c).collect(),
        false,
    )
    .1;
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "36621";
    let part_2: usize = combat(
        &mut p1.iter().map(|&c| c).collect(),
        &mut p2.iter().map(|&c| c).collect(),
        true,
    )
    .1;
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input: &str = "Player 1:\n9\n2\n6\n3\n1\r\n\r\nPlayer 2:\n5\n8\n4\n7\n10";
        let (mut p1, mut p2) = parse_input(&input);
        let answer: usize = combat(&mut p1, &mut p2, false).1;
        assert_eq!(answer, 306);
    }
    #[test]
    fn test_example_recursive() {
        let input: &str = "Player 1:\n9\n2\n6\n3\n1\r\n\r\nPlayer 2:\n5\n8\n4\n7\n10";
        let (mut p1, mut p2) = parse_input(&input);
        let answer: usize = combat(&mut p1, &mut p2, true).1;
        assert_eq!(answer, 291);
    }
    #[test]
    fn test_example_infinite() {
        let input: &str = "Player 1:\n43\n19\r\n\r\nPlayer 2:\n2\n29\n14";
        let (mut p1, mut p2) = parse_input(&input);
        let _answer: usize = combat(&mut p1, &mut p2, true).1;
    }
}
