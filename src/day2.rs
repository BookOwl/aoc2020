use crate::Part;
use regex::Regex;

struct Password {
    min: usize,
    max: usize,
    rule: char,
    password: String,
}

pub fn run(part: Part, input_path: Option<&str>) {
    let input_str = crate::read_input(2, input_path);
    let re = Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<rule>[a-z]): (?P<password>.+)").unwrap();
    let input: Vec<Password> = input_str.split('\n')
                                        .filter(|line| line.trim() != "")
                                        .map(|line| {
                                            let caps = re.captures(line.trim()).unwrap();
                                            Password {
                                                min: caps["min"].parse().unwrap(),
                                                max: caps["max"].parse().unwrap(),
                                                rule: caps["rule"].chars().nth(0).unwrap(),
                                                password: caps["password"].to_string(),
                                            }
                                        })
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

fn part1(input: &[Password]) {
    let num_valid = input.iter().filter(|&p| {
        let num_instances = p.password.chars().filter(|&c| c == p.rule).count();
        p.min <= num_instances && num_instances <= p.max
    }).count();
    println!("{}", num_valid);
}

fn part2(input: &[Password]) {
    let num_valid = input.iter().filter(|&p| {
        let chars: Vec<_> = p.password.chars().collect();
        /*
            `^` is XOR
            false ^ false == false
            true ^ false == true
            false ^ true == true
            true ^ true == false
         */
        (chars[p.min - 1] == p.rule) ^ (chars[p.max - 1] == p.rule)
    }).count();
    println!("{}", num_valid);
}