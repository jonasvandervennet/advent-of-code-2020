// An example implementation of an AoC challenge
// (source: aoc 2019 day 1 )

use crate::util::read_lines;

fn calculate_fuel(mass: usize) -> usize {
    // can be checked with a checked_sub(2).unwrap_or(0) too!
    let tmp = mass / 3;
    if tmp <= 2 {
        return 0;
    }
    tmp - 2
}

fn recurse_fuel(mass: usize) -> usize {
    let fuel = calculate_fuel(mass);
    if fuel <= 0 {
        return 0;
    }
    fuel + recurse_fuel(fuel)
}

fn get_input() -> Result<Vec<usize>, std::io::Error> {
    match read_lines("inputs/example.txt") {
        Ok(input_str) =>
        // https://stackoverflow.com/a/30608280
        {
            Ok(input_str
                .map(|line| match line {
                    Ok(s) => match s.parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Could not convert line {} to usize", s);
                            0
                        }
                    },
                    Err(_) => {
                        println!("Invalid line read from file");
                        0
                    }
                })
                .collect())
        }
        Err(e) => Err(e),
    }
}

pub fn main() {
    let input: Vec<usize> = match get_input() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("ERR: {}", e);
            return;
        }
    };
    // PART 1
    let known_answer: usize = 3330521;
    let part_1: usize = input.iter().map(|&i| calculate_fuel(i)).sum();
    if part_1 != known_answer {
        print!("INCORRECT || ")
    }
    println!("PART 1: {}", part_1);

    // PART 2
    let known_answer: usize = 4992931;
    let part_2: usize = input.iter().map(|&i| recurse_fuel(i)).sum();
    if part_2 != known_answer {
        print!("INCORRECT || ")
    }
    println!("PART 2: {}", part_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel_example_1() {
        assert_eq!(calculate_fuel(12), 2);
    }

    #[test]
    fn test_fuel_example_2() {
        assert_eq!(calculate_fuel(14), 2);
    }

    #[test]
    fn test_fuel_example_3() {
        assert_eq!(calculate_fuel(1969), 654);
    }

    #[test]
    fn test_fuel_example_4() {
        assert_eq!(calculate_fuel(100756), 33583);
    }
}
