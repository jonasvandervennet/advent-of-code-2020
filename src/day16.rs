use crate::util::{print_part_1, print_part_2};
use std::cmp;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug)]
struct Field {
    bounds: Vec<(usize, usize)>,
}

impl Field {
    fn possible(&self, value: usize) -> bool {
        for (lower, upper) in &self.bounds {
            if value >= *lower && value <= *upper {
                return true;
            }
        }
        false
    }
}

fn check_invalid_tickets(input: &str) -> usize {
    let mut sections = input.split("\r\n\r\n");

    let fields_s = sections.next().expect("No field ranges provided..");
    let fields = fields_s
        .lines()
        .map(|line| {
            let mut bounds = Vec::new();
            for bound in line.split(": ").collect::<Vec<&str>>()[1].split(" or ") {
                let split = bound.split("-").collect::<Vec<&str>>();
                bounds.push((
                    split[0].parse::<usize>().unwrap(),
                    split[1].parse::<usize>().unwrap(),
                ));
            }
            Field { bounds: bounds }
        })
        .collect::<Vec<Field>>();

    // unused atm
    sections.next().expect("No personal ticket provided..");

    let tickets_s = sections.next().expect("No other tickets provided..");
    tickets_s.lines().skip(1).fold(0, |acc, line| {
        for n in line.split(",") {
            let n = n.parse::<usize>().unwrap();
            if !fields.iter().map(|field| field.possible(n)).any(|x| x) {
                return acc + n;
            }
        }
        acc
    })
}

fn determine_field_order(input: &str) -> usize {
    let mut sections = input.split("\r\n\r\n");

    let fields_s = sections.next().expect("No field ranges provided..");
    let fields = fields_s
        .lines()
        .map(|line| {
            let mut bounds = Vec::new();
            for bound in line.split(": ").collect::<Vec<&str>>()[1].split(" or ") {
                let split = bound.split("-").collect::<Vec<&str>>();
                bounds.push((
                    split[0].parse::<usize>().unwrap(),
                    split[1].parse::<usize>().unwrap(),
                ));
            }
            Field { bounds: bounds }
        })
        .collect::<Vec<Field>>();

    // unused atm
    let your_ticket_s = sections.next().expect("No personal ticket provided..");
    let your_ticket = your_ticket_s
        .lines()
        .nth(1)
        .unwrap()
        .split(",")
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let tickets_s = sections.next().expect("No other tickets provided..");
    let valid_tickets = tickets_s
        .lines()
        .skip(1)
        .map(|line| {
            line.split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .filter(|numbers| {
            for n in numbers.iter() {
                if !fields.iter().map(|field| field.possible(*n)).any(|x| x) {
                    return false;
                }
            }
            true
        })
        .collect::<Vec<Vec<usize>>>();

    let mut matching_fields = vec![usize::MAX; fields.len()];
    loop {
        let mut potential_allocations: HashMap<usize, Vec<usize>> = HashMap::new();
        for (i, field) in fields.iter().enumerate() {
            if matching_fields[i] != usize::MAX {
                continue;
            }
            for j in 0..fields.len() {
                let mut valid = true;
                for ticket in valid_tickets.iter() {
                    if !field.possible(ticket[j]) {
                        valid = false;
                        break;
                    }
                }
                if valid && !matching_fields.contains(&j) {
                    if potential_allocations.contains_key(&i) {
                        potential_allocations.get_mut(&i).unwrap().push(j);
                    } else {
                        potential_allocations.insert(i, vec![j]);
                    }
                }
            }
        }

        for (field_nr, options) in potential_allocations {
            if options.len() == 1 {
                matching_fields[field_nr] = options[0];
            }
        }
        if !matching_fields.contains(&usize::MAX) {
            break;
        }
    }

    let mut reordered_ticket = vec![0; fields.len()];
    for (new_i, old_i) in matching_fields.iter().enumerate() {
        reordered_ticket[new_i] = your_ticket[*old_i];
    }

    // min check is for the test with only 3 fields
    reordered_ticket[..cmp::min(6, fields.len())]
        .iter()
        .product()
}

pub fn main() {
    let input = read_to_string("inputs/day16.txt").unwrap();

    // PART 1
    let start = Instant::now();
    let known_answer = "26869";
    let part_1: usize = check_invalid_tickets(&input);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "855275529001";
    let part_2: usize = determine_field_order(&input);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input: &str = "class: 1-3 or 5-7\nrow: 6-11 or 33-44\nseat: 13-40 or 45-50\r\n\r\nyour ticket:\n7,1,14\r\n\r\nnearby tickets:\n7,3,47\n40,4,50\n55,2,20\n38,6,12";
        let answer: usize = check_invalid_tickets(&input);
        assert_eq!(answer, 71);
    }

    #[test]
    fn test_example_2() {
        let input: &str = "class: 0-1 or 4-19\nrow: 0-5 or 8-19\nseat: 0-13 or 16-19\r\n\r\nyour ticket:\n11,12,13\r\n\r\nnearby tickets:\n3,9,18\n15,1,5\n55,2,20\n5,14,9";
        let answer: usize = determine_field_order(&input);
        assert_eq!(answer, 11 * 12 * 13);
    }
}
