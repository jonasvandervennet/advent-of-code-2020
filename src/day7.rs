use crate::util::{print_part_1, print_part_2};
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::time::Instant;

fn get_collection_small_to_large(lines: &str) -> HashMap<String, Vec<String>> {
    // Collection holds a 'fits in' relationship
    let mut coll: HashMap<String, Vec<String>> = HashMap::new();
    for line in lines.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        if words.len() == 7 {
            // X X bags contain no other bags.
            continue;
        }
        let num_contains = words.iter().filter(|&x| x.contains(',')).count() + 1;
        let first = words.iter().nth(0).unwrap();
        let second = words.iter().nth(1).unwrap();
        let outside: String = format!("{} {}", first, second);

        let start_index = 5;
        for i in 0..num_contains {
            let index = start_index + i * 4;
            let first = words.iter().nth(index).unwrap();
            let second = words.iter().nth(index + 1).unwrap();
            let inside: String = format!("{} {}", first, second);
            coll.entry(inside)
                .or_insert_with(Vec::new)
                .push(outside.to_owned());
        }
    }
    coll
}

fn get_collection_large_to_small(lines: &str) -> HashMap<String, HashSet<(usize, String)>> {
    // Collection holds a 'holds x' relationship
    let mut coll: HashMap<String, HashSet<(usize, String)>> = HashMap::new();
    for line in lines.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        if words.len() == 7 {
            // X X bags contain no other bags.
            continue;
        }
        let num_contains = words.iter().filter(|&x| x.contains(',')).count() + 1;
        let first = words.iter().nth(0).unwrap();
        let second = words.iter().nth(1).unwrap();
        let outside: String = format!("{} {}", first, second);

        let start_index = 4;
        for i in 0..num_contains {
            let index = start_index + i * 4;
            let amount: usize = words
                .iter()
                .nth(index)
                .unwrap()
                .parse()
                .expect("Could not parse amount of bags");
            let first = words.iter().nth(index + 1).unwrap();
            let second = words.iter().nth(index + 2).unwrap();
            let inside: String = format!("{} {}", first, second);
            coll.entry(outside.to_owned())
                .or_insert_with(HashSet::new)
                .insert((amount, inside));
        }
    }
    coll
}

fn get_shiny_gold_options(lines: &str) -> usize {
    let coll = get_collection_small_to_large(lines);
    let query = &"shiny gold".to_string();
    if !coll.contains_key(query) {
        return 0;
    }

    let mut bags: HashSet<&String> = HashSet::new();
    let mut queue: Vec<&String> = vec![query];
    loop {
        let mut new_queue: Vec<&String> = Vec::new();
        for container in queue {
            bags.insert(container);
            if coll.contains_key(container) {
                new_queue.extend(coll.get(container).unwrap())
            }
        }
        if new_queue.len() == 0 {
            break;
        }
        queue = new_queue;
    }

    bags.len() - 1 // contains original bag as well
}

fn get_total_bags_in_query(
    coll: &HashMap<String, HashSet<(usize, String)>>,
    query: &String,
) -> usize {
    if !coll.contains_key(query) {
        return 1;
    }

    let mut total = 0;
    for (amount, color) in coll.get(query).unwrap() {
        total += amount * get_total_bags_in_query(coll, color);
    }
    1 + total // unsure about reasoning behind calculation here
}

fn get_total_bags_in_shiny_gold(lines: &str) -> usize {
    let coll = get_collection_large_to_small(lines);
    get_total_bags_in_query(&coll, &"shiny gold".to_string()) - 1 // unsure about reasoning behind calculation here
}

pub fn main() {
    let input = read_to_string("inputs/day7.txt").expect("Input not found..");

    // PART 1
    let start = Instant::now();
    let known_answer = "226";
    let part_1: usize = get_shiny_gold_options(&input);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "9569";
    let part_2: usize = get_total_bags_in_shiny_gold(&input);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.\ndark orange bags contain 3 bright white bags, 4 muted yellow bags.\nbright white bags contain 1 shiny gold bag.\nmuted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\nshiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\ndark olive bags contain 3 faded blue bags, 4 dotted black bags.\nvibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\nfaded blue bags contain no other bags.\ndotted black bags contain no other bags.";
        let options: usize = get_shiny_gold_options(&input);
        assert_eq!(options, 4);
    }
    #[test]
    fn test_example_2() {
        let input: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.\ndark orange bags contain 3 bright white bags, 4 muted yellow bags.\nbright white bags contain 1 shiny gold bag.\nmuted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\nshiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\ndark olive bags contain 3 faded blue bags, 4 dotted black bags.\nvibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\nfaded blue bags contain no other bags.\ndotted black bags contain no other bags.";
        let options: usize = get_total_bags_in_shiny_gold(&input);
        assert_eq!(options, 32);
    }
    #[test]
    fn test_example_3() {
        let input: &str = "shiny gold bags contain 2 dark red bags.\ndark red bags contain 2 dark orange bags.\ndark orange bags contain 2 dark yellow bags.\ndark yellow bags contain 2 dark green bags.\ndark green bags contain 2 dark blue bags.\ndark blue bags contain 2 dark violet bags.\ndark violet bags contain no other bags.";
        let options: usize = get_total_bags_in_shiny_gold(&input);
        assert_eq!(options, 126);
    }
}
