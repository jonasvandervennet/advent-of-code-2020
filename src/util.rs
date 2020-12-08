use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Duration;

// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn print_answer(part_nr: usize, answer: &str, known_answer: &str, duration: Duration) {
    if answer != known_answer {
        print!("INCORRECT || ")
    }
    println!("PART {}: {}", part_nr, answer);
    println!("\t[{:?}]", duration);
}

pub fn print_part_1(answer: &str, known_answer: &str, duration: Duration) {
    print_answer(1, answer, known_answer, duration);
}
pub fn print_part_2(answer: &str, known_answer: &str, duration: Duration) {
    print_answer(2, answer, known_answer, duration);
}
