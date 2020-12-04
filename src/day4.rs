use crate::Part;
use regex::Regex;
use std::collections::HashMap;

struct Entry(HashMap<String, String>);

pub fn run(part: Part, input_str: &str) {
    let re = Regex::new(r"(?P<field>[a-z]+):(?P<value>\S+)").unwrap();
    let mut entries = Vec::new();
    for passport in input_str.split("\n\n") {
        let mut hash = HashMap::new();
        for cap in re.captures_iter(passport) {
            hash.insert(cap["field"].to_string(), cap["value"].to_string());
        }
        entries.push(Entry(hash))
    }
    match part {
        Part::First => part1(&entries),
        Part::Second => part2(&entries),
        Part::All => {
            part1(&entries);
            part2(&entries);
        },
    }
}

fn part1(entries: &[Entry]) {
    let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let num_valid = entries.iter().filter(|Entry(m)| {
        required_fields.iter().all(|field| m.contains_key(&field.to_string()))
    }).count();
    println!("{}", num_valid);

}

fn part2(entries: &[Entry]) {
    let hcl_re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let hgt_re = Regex::new(r"^(?P<height>\d+)(?P<unit>cm|in)$").unwrap();
    let ecl_re = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    let pid_re = Regex::new(r"^[0-9]{9}$").unwrap();
    let num_valid = entries.iter().filter(|Entry(m)| {
        let checks = [
            m.get("byr").map_or(false, |y| y.parse::<u32>().map_or(false, |y| 1920 <= y && y <= 2002)),
            m.get("iyr").map_or(false, |y| y.parse::<u32>().map_or(false, |y| 2010 <= y && y <= 2020)),
            m.get("eyr").map_or(false, |y| y.parse::<u32>().map_or(false, |y| 2020 <= y && y <= 2030)),
            {if let Some(hgt) = m.get("hgt") {
                hgt_re.captures(hgt).map_or(false, |cap| {
                    let x = cap["height"].parse::<u32>().unwrap();
                    if &cap["unit"] == "cm" {
                        150 <= x && x <= 193
                    } else {
                        59 <= x && x <= 76
                    }
                })
            } else {
                false
            }},
            m.get("hcl").map_or(false, |c| hcl_re.is_match(c)),
            m.get("ecl").map_or(false, |c| ecl_re.is_match(c)),
            m.get("pid").map_or(false, |pid| pid_re.is_match(pid)),
        ];
        checks.iter().all(|&b| b)
    }).count();
    println!("{}", num_valid);
}