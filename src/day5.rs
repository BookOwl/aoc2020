use crate::Part;

pub fn run(part: Part, input_str: &str) {
    let input: Vec<&str> = input_str.split("\n").filter(|l| *l != "").collect();
    match part {
        Part::First => part1(&input),
        Part::Second => part2(&input),
        Part::All => {
            part1(&input);
            part2(&input);
        },
    }
}

fn part1(input: &[&str]) {
    let max_id = input.iter().map(|pass| seat_id(pass)).max().unwrap();
    println!("{}", max_id);
}

fn part2(input: &[&str]) {
    let seen_ids: Vec<u64> = input.iter().map(|pass| seat_id(pass)).collect();
    let min = seen_ids.iter().min().unwrap() + 1;
    let max = seen_ids.iter().max().unwrap() - 1;
    for id in (min..=max) {
        if !seen_ids.contains(&id) && seen_ids.contains(&(id-1)) && seen_ids.contains(&(id+1)) {
            println!("{}", id);
            return
        }
    }

}

fn seat_id(pass: &str) -> u64 {
    let mut min_row: u64 = 0;
    let mut max_row: u64 = 127;
    let mut min_column: u64 = 0;
    let mut max_column: u64 = 7;
    for step in pass.chars() {
        match step {
            'F' => max_row = (min_row + max_row) / 2,
            'B' => min_row = (min_row + max_row) / 2 + 1,
            'L' => max_column = (min_column + max_column) / 2,
            'R' => min_column = (min_column + max_column) / 2 + 1,
            _ => unreachable!(),
        }
    }
    min_row * 8 + min_column
}