use structopt::StructOpt;

mod util; // https://users.rust-lang.org/t/modules-what-am-i-doing-wrong/35186/2

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

mod example;
mod infi;

// https://docs.rs/structopt/0.3.20/structopt/#how-to-derivestructopt
#[derive(StructOpt)]
#[structopt(
    name = "aoc-2020",
    about = "Codebase for all of the 2020 Advent of Code challenges in Rust"
)]
struct Opt {
    /// Specify day to run
    #[structopt(short = "d", long = "day", default_value = "all")]
    day: String,
}

fn print_day_header(day: usize) {
    println!(
        "------------------------------------ DAY {} ------------------------------------",
        day
    );
}

fn main() {
    let args = Opt::from_args();
    let mains = [
        day1::main,
        day2::main,
        day3::main,
        day4::main,
        day5::main,
        day6::main,
        day7::main,
        day8::main,
        day9::main,
        day10::main,
        day11::main,
        day12::main,
        day13::main,
    ];

    match args.day.as_str() {
        "all" => {
            for (day, main) in mains.iter().enumerate() {
                print_day_header(day + 1);
                main();
                println!();
            }
        }
        "example" => example::main(),
        "infi" => infi::main(),
        _ => {
            let day: usize = args.day.parse().unwrap();
            print_day_header(day);
            mains[day - 1]();
        }
    }
}
