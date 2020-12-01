use crate::util::read_lines;

fn get_2_values_with_sum_2020(values: &Vec<usize>) -> (usize, usize) {
    for (i, val) in values.iter().enumerate() {
        for val2 in values[(i + 1)..].iter() {
            if val + val2 == 2020 {
                return (*val, *val2);
            }
        }
    }
    (0, 0)
}

fn get_3_values_with_sum_2020(values: &Vec<usize>) -> (usize, usize, usize) {
    for (i, val) in values.iter().enumerate() {
        for val2 in values[(i + 1)..].iter() {
            for val3 in values[(i + 2)..].iter() {
                if val + val2 + val3 == 2020 {
                    return (*val, *val2, *val3);
                }
            }
        }
    }
    (0, 0, 0)
}

fn get_input() -> Result<Vec<usize>, std::io::Error> {
    match read_lines("inputs/day1.txt") {
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
    let known_answer: usize = 319531;
    let part_1: usize = {
        let values = get_2_values_with_sum_2020(&input);
        values.0 * values.1
    };
    if part_1 != known_answer {
        print!("INCORRECT || ")
    }
    println!("PART 1: {}", part_1);

    // PART 2
    let known_answer: usize = 4992931;
    let part_2: usize = {
        let values = get_3_values_with_sum_2020(&input);
        values.0 * values.1 * values.2
    };
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
        let input: Vec<usize> = vec![1721, 979, 366, 299, 675, 1456];
        let values = get_2_values_with_sum_2020(&input);
        assert_eq!(values.0 * values.1, 514579);
    }

    #[test]
    fn test_example_2() {
        let input: Vec<usize> = vec![1721, 979, 366, 299, 675, 1456];
        let values = get_3_values_with_sum_2020(&input);
        assert_eq!(values.0 * values.1 *values.2, 241861950);
    }
}