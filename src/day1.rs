use crate::Part;

pub fn run(part: Part, input_path: Option<&str>) {
    let input_str = crate::read_input(1, input_path);
    let input: Vec<usize> = input_str.split('\n')
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

fn part1(input: &[usize]) {
    'outer: for (i, x) in input.iter().enumerate() {
        for y in input[i+1..].iter() {
            if x + y == 2020 {
                println!("{}", x*y);
                break 'outer
            }
        }
    }

}

fn part2(input: &[usize]) {
    'outer: for (i, x) in input.iter().enumerate() {
        for (j, y) in input[i+1..].iter().enumerate() {
            for z in input[j+1..].iter() {
                if x + y + z == 2020 {
                    println!("{}", x * y * z);
                    break 'outer
                }
            }
        }
    }
}