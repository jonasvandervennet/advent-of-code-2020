use crate::util::{print_part_1, print_part_2};
use regex::Regex;
use std::fs::read_to_string;
use std::time::Instant;

fn verify_password(pass: &str, verify_value: bool) -> bool {
    let required_fields = ["byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"]; // doesn't include country id field
    for field in required_fields.iter() {
        if !pass.contains(field) {
            return false;
        }
        if !verify_value {
            continue;
        }
        let value: &str = pass.split(field).collect::<Vec<&str>>()[1]
            .split_whitespace()
            .collect::<Vec<&str>>()[0];

        match field {
            &"byr:" => {
                let year = value.parse::<usize>().unwrap_or(0);
                if year < 1920 || year > 2002 {
                    return false;
                }
            }
            &"iyr:" => {
                let year = value.parse::<usize>().unwrap_or(0);
                if year < 2010 || year > 2020 {
                    return false;
                }
            }
            &"eyr:" => {
                let year = value.parse::<usize>().unwrap_or(0);
                if year < 2020 || year > 2030 {
                    return false;
                }
            }
            &"hgt:" => {
                if !value.contains("cm") && !value.contains("in") {
                    return false;
                }
                let height = value[..value.len() - 2].parse::<usize>().unwrap();
                if &value[value.len() - 2..] == "in" {
                    if height < 59 || height > 76 {
                        return false;
                    }
                } else {
                    if height < 150 || height > 193 {
                        return false;
                    }
                }
            }
            &"hcl:" => {
                let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
                if !re.is_match(value) {
                    return false;
                }
            }
            &"ecl:" => {
                if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value) {
                    return false;
                }
            }
            &"pid:" => {
                let re = Regex::new(r"^[0-9]{9}$").unwrap();
                if !re.is_match(value) {
                    return false;
                }
            }
            _ => {}
        };
    }
    true
}

pub fn main() {
    let input = read_to_string("inputs/day4.txt").unwrap();

    // PART 1
    let start = Instant::now();
    let known_answer: usize = 264;
    let part_1: usize = input
        .split("\r\n\r\n") // empty lines (this probably depends on the operating system..)
        .map(|line| if verify_password(line, false) { 1 } else { 0 })
        .sum();
    let duration = start.elapsed();
    print_part_1(part_1, known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer: usize = 224;
    let part_2: usize = input
        .split("\r\n\r\n") // empty lines (this probably depends on the operating system..)
        .map(|line| if verify_password(line, true) { 1 } else { 0 })
        .sum();
    let duration = start.elapsed();
    print_part_2(part_2, known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input: &str =
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm";
        let valid: bool = verify_password(&input, false);
        assert_eq!(valid, true);
    }
    #[test]
    fn test_example_2() {
        let input: &str = "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929";
        let valid: bool = verify_password(&input, false);
        assert_eq!(valid, false);
    }
    #[test]
    fn test_example_3() {
        let input: &str =
            "hcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm";
        let valid: bool = verify_password(&input, false);
        assert_eq!(valid, true);
    }
    #[test]
    fn test_example_7() {
        let input: &str = "hcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in";
        let valid: bool = verify_password(&input, false);
        assert_eq!(valid, false);
    }
    #[test]
    fn test_example_8() {
        let input: &str =
            "eyr:1972 cid:100\nhcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926";
        let valid: bool = verify_password(&input, true);
        assert_eq!(valid, false);
    }
    #[test]
    fn test_example_9() {
        let input: &str =
            "iyr:2019\nhcl:#602927 eyr:1967 hgt:170cm\necl:grn pid:012533040 byr:1946";
        let valid: bool = verify_password(&input, true);
        assert_eq!(valid, false);
    }
    #[test]
    fn test_example_10() {
        let input: &str =
            "hcl:dab227 iyr:2012\necl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277";
        let valid: bool = verify_password(&input, true);
        assert_eq!(valid, false);
    }
    #[test]
    fn test_example_11() {
        let input: &str = "hgt:59cm ecl:zzz\neyr:2038 hcl:74454a iyr:2023\npid:3556412378 byr:2007";
        let valid: bool = verify_password(&input, true);
        assert_eq!(valid, false);
    }
    #[test]
    fn test_example_12() {
        let input: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\nhcl:#623a2f";
        let valid: bool = verify_password(&input, true);
        assert_eq!(valid, true);
    }
    #[test]
    fn test_example_13() {
        let input: &str =
            "eyr:2029 ecl:blu cid:129 byr:1989\niyr:2014 pid:896056539 hcl:#a97842 hgt:165cm";
        let valid: bool = verify_password(&input, true);
        assert_eq!(valid, true);
    }
    #[test]
    fn test_example_14() {
        let input: &str =
            "hcl:#888785\nhgt:164cm byr:2001 iyr:2015 cid:88\npid:545766238 ecl:hzl\neyr:2022";
        let valid: bool = verify_password(&input, true);
        assert_eq!(valid, true);
    }
    #[test]
    fn test_example_15() {
        let input: &str = "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        let valid: bool = verify_password(&input, true);
        assert_eq!(valid, true);
    }
}
