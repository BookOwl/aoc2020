use crate::Part;

pub fn run(part: Part, input_str: &str) {
    let input: Vec<i64> = input_str.split('\n')
                                .filter_map(|i| i.parse().ok())
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

fn first_invalid_number(input: &[i64]) -> i64 {
    for chunk in input.windows(26) {
        let (n, prev) = chunk.split_last().unwrap();
        let valid = prev.iter().any(|x| {
            if prev.contains(&(n - x)) {
                if n - x == *x {
                    prev.iter().filter(|y| *y == x).count() > 1
                } else {
                    true
                }
            } else {
                false
            }
        });
        if !valid {

            return *n
        }
    }
    panic!("could not find solution!");
}

fn part1(input: &[i64]) {
    println!("{}", first_invalid_number(input));
}
fn part2(input: &[i64]) {
    let invalid = first_invalid_number(input);
    for l in (2..input.len()).rev() {
        for set in input.windows(l) {
            if set.iter().sum::<i64>() == invalid {
                println!("{}", set.iter().min().unwrap() + set.iter().max().unwrap());
                return
            }
        }
    }
    panic!("could not find valid solution!");
}