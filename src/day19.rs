use crate::util::{print_part_1, print_part_2};
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Clone)]
struct Rule {
    successors: Vec<Vec<usize>>,
    literal: bool,
    value: char,
}

impl Rule {
    fn empty() -> Self {
        Rule {
            successors: vec![],
            literal: false,
            value: '*',
        }
    }
    fn match_rule(&self, poss_match: &str, rules: &Vec<Rule>) -> Vec<usize> {
        if self.literal {
            return if poss_match.chars().nth(0).unwrap() == self.value {
                vec![1]
            } else {
                vec![]
            };
        }
        let mut offsets = Vec::new();
        for succ_list in self.successors.iter() {
            let mut local_offsets = vec![0];
            for rule_index in succ_list.iter() {
                local_offsets = local_offsets
                    .iter()
                    .map(|&offset| {
                        if offset == poss_match.len() {
                            vec![]
                        } else {
                            let rule = &rules[*rule_index];
                            let result: Vec<usize> = rule
                                .match_rule(&poss_match[offset..], rules)
                                .iter()
                                .map(|r| r + offset)
                                .collect();
                            if result.len() == 0 {
                                vec![]
                            } else {
                                result
                            }
                        }
                    })
                    .flatten()
                    .collect();
            }
            offsets.extend(local_offsets);
        }
        offsets
    }
}

fn validate_rules(input: &str, part: usize) -> usize {
    let mut parts = input.split("\r\n\r\n");
    let rules_s = parts.next().unwrap();

    let num_rules = rules_s.lines().count();
    let mut rules: Vec<Rule> = vec![Rule::empty(); num_rules];
    for line in rules_s.lines() {
        let div = line.split(": ").collect::<Vec<&str>>();
        let index = div[0].parse::<usize>().expect("Invalid index..");
        let content = div[1];
        let rule = match content {
            "\"a\"" => Rule {
                successors: vec![],
                literal: true,
                value: 'a',
            },
            "\"b\"" => Rule {
                successors: vec![],
                literal: true,
                value: 'b',
            },
            _ => {
                let mut successors = Vec::new();
                for list in content.split("|") {
                    successors.push(
                        list.split_whitespace()
                            .map(|i| i.parse::<usize>().expect("Invalid index in succ list.."))
                            .collect(),
                    );
                }
                Rule {
                    successors: successors,
                    literal: false,
                    value: '*',
                }
            }
        };
        if index > rules.len() {
            rules.resize(index + 1, Rule::empty());
        }
        rules[index] = rule;
    }

    // part 2: replace rules to introduce loops
    if part == 2 {
        rules[8].successors = vec![vec![42], vec![42, 8]];
        rules[11].successors = vec![vec![42, 31], vec![42, 11, 31]];
    }

    let poss_matches = parts.next().unwrap();

    poss_matches
        .lines()
        .map(|poss_match| {
            // check if the whole string was validated
            if rules[0]
                .match_rule(poss_match, &rules)
                .contains(&poss_match.len())
            {
                1
            } else {
                0
            }
        })
        .sum()
}

pub fn main() {
    let input = read_to_string("inputs/day19.txt").unwrap();

    // PART 1
    let start = Instant::now();
    let known_answer = "299";
    let part_1: usize = validate_rules(&input, 1);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "414";
    let part_2: usize = validate_rules(&input, 2);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_example_1() {
        let input: &str =
            "0: 4 1 5\n1: 2 3 | 3 2\n2: 4 4 | 5 5\n3: 4 5 | 5 4\n4: \"a\"\n5: \"b\"\r\n\r\nababbb\nbababa\nabbbab\naaabbb\naaaabbb";
        let answer: usize = validate_rules(&input, 1);
        assert_eq!(answer, 2);
    }

    #[test]
    fn test_example_2() {
        let input = read_to_string("inputs/day19_test.txt").unwrap();
        let answer: usize = validate_rules(&input, 1);
        assert_eq!(answer, 3);
    }

    #[test]
    fn test_example_3() {
        let input = read_to_string("inputs/day19_test.txt").unwrap();
        let answer: usize = validate_rules(&input, 2);
        assert_eq!(answer, 12);
    }
}
