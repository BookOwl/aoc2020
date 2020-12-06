use crate::Part;
use std::collections::HashSet;

pub fn run(part: Part, input_str: &str) {
    let input: Vec<Vec<&str>> = input_str.split("\n\n")
                                         .map(|l| l.split("\n").filter(|a| a.trim() != "").collect())
                                         .collect();
    match part {
        Part::First => part1(&input),
        Part::Second => part2(&input),
        Part::All => {
            part1(&input);
            part2(&input);
        },
    }
}

fn part1(input: &[Vec<&str>]) {
    let mut answered = HashSet::with_capacity(26);
    let mut total_answered = 0;
    for answers in input {
        answered.clear();
        for answer in answers {
            for question in answer.chars() {
                answered.insert(question);
            }
        }
        total_answered += answered.len();
    }
    println!("{}", total_answered);
}

fn part2(input: &[Vec<&str>]) {
    let mut total_answered = 0;
    for answers in input {
        let mut answered: HashSet<_> = answers[0].chars().collect();
        for answer in &answers[1..] {
            let a: HashSet<_> = answer.chars().collect();
            answered = answered.intersection(&a).cloned().collect();
        }
        total_answered += answered.len();
    }
    println!("{}", total_answered);
}