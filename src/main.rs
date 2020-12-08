use std::env;

mod days;
mod err;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

fn main() {
    let aoc_days: Vec<Box<dyn days::Day>> = vec![
        Box::new(day1::Day {}),
        Box::new(day2::Day {}),
        Box::new(day3::Day {}),
        Box::new(day4::Day {}),
        Box::new(day5::Day {}),
        Box::new(day6::Day {}),
        Box::new(day7::Day {}),
        Box::new(day8::Day {}),
    ];

    match env::args().collect::<Vec<String>>().get(1) {
        Some(arg) => match arg.parse::<usize>() {
            Ok(day) => {
                let idx = day - 1;
                match aoc_days.get(idx) {
                    Some(runner) => runner.run(),
                    None => println!("could not find runner for day {}", day),
                }
            }
            Err(e) => println!("arg error: {}", e),
        },
        None => match aoc_days.last() {
            Some(runner) => runner.run(),
            None => println!("couldn't find anything to run!"),
        },
    }
}
