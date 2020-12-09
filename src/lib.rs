use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Part {
    First,
    Second,
    All,
}

static INPUT: [&'static str; 25] = [include_str!("inputs/day1.txt"), include_str!("inputs/day2.txt"),
                                    include_str!("inputs/day3.txt"), include_str!("inputs/day4.txt"),
                                    include_str!("inputs/day5.txt"), include_str!("inputs/day6.txt"),
                                    include_str!("inputs/day7.txt"), include_str!("inputs/day8.txt"),
                                    "", "",
                                    "", "",
                                    "", "",
                                    "", "",
                                    "", "",
                                    "", "",
                                    "", "",
                                    "", "", ""];

fn read_input(day: usize, file_path: Option<&str>) -> String {
    if let Some(path) = file_path {
        let mut file = File::open(path).expect(&format!("Error opening '{}'", path));
        let mut contents = String::with_capacity(file.metadata().unwrap().len() as usize);
        file.read_to_string(&mut contents).unwrap();
        contents
    } else {
        INPUT[day - 1].to_string()
    }
}

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;

pub const SOLUTIONS: [&dyn Fn(Part, &str) -> (); 8] = [&day1::run, &day2::run, &day3::run, &day4::run, &day5::run,
                                                       &day6::run, &day7::run, &day8::run];

pub fn solve(day: usize, part: Part, input_path: Option<&str>) {
    let input_str = read_input(day, input_path);
    SOLUTIONS.get(day - 1).expect(&format!("Day {} is not coded yet", day))(part, &input_str)
}