use clap::App;
use aoc2020::Part;

fn main() {
    let matches = App::new("aoc2020")
        .arg("<day> 'Which day's puzzles to solve'")
        .arg("--part [part] 'Which part of the day's puzzles to solve, defaults to all")
        .arg("--input [file] 'Which file to read the day's input from, defaults to my input'")
        .get_matches();
    let day: usize  = matches.value_of_t("day").unwrap_or_else(|e| e.exit());
    if day == 0 || day > 25 {
        panic!("Day must be greater than 0 and less than or equal to 25")
    }
    let part = match matches.value_of("part") {
        Some("1") => Part::First,
        Some("2") => Part::Second,
        _ => Part::All,
    };
    let input = matches.value_of("input");

    aoc2020::solve(day, part, input);
}
