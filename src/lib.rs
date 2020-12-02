use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Part {
    First,
    Second,
    All,
}

static INPUT: [&'static str; 1] = [include_str!("inputs/day1.txt")];

fn read_input(day: usize, file_path: Option<&str>) -> String {
    if let Some(path) = file_path {
        let mut file = File::open(path).expect(&format!("Error opening '{}'", path));
        let mut contents = String::with_capacity(file.metadata().unwrap().len() as usize);
        file.read_to_string(&mut contents);
        contents
    } else {
        INPUT[day - 1].to_string()
    }
}

pub mod day1;

pub const SOLUTIONS: [&dyn Fn(Part, Option<&str>) -> (); 1] = [&day1::run];