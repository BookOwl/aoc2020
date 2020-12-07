use crate::Part;
use std::collections::{HashMap, HashSet};
use regex::Regex;

#[derive(Debug)]
struct Link {
    name: String,
    number: u32,
}

#[derive(Debug)]
struct Rules {
    rules: HashMap<String, Vec<Link>>,
    parents: HashMap<String, Vec<String>>,
}

impl Rules {
    fn new() -> Self {
        Rules {
            rules: HashMap::with_capacity(2048),
            parents: HashMap::with_capacity(2048),
        }
    }
    fn add_rule(&mut self, from: &str, to: Option<(&str, u32)>) {
        let rules = self.rules.entry(from.to_string()).or_insert_with(|| Vec::with_capacity(64));
        if let Some((to, number)) = to {
            rules.push(Link {
                name: to.to_string(),
                number,
            });
            let parents = self.parents.entry(to.to_string()).or_insert_with(|| Vec::with_capacity(64));
            parents.push(from.to_string());
        }
    }
    fn get_children(&self, node: &str) -> &[Link] {
        self.rules.get(&node.to_string()).unwrap()
    }
    fn get_parents(&self, node: &str) -> Vec<String> {
        self.parents.get(&node.to_string()).cloned().unwrap_or_else(|| Vec::new())
    }
}

pub fn run(part: Part, input_str: &str) {
    let bag_re = Regex::new(r"(?P<color>[a-z]+ [a-z]+) bags contain (?P<contains>.+?)\.").unwrap();
    let contents_re = Regex::new(r"(?P<number>\d+) (?P<color>[a-z]+ [a-z]+) bag(s)?").unwrap();
    let mut input = Rules::new();
    for cap in bag_re.captures_iter(input_str) {
       // println!("{:?}", cap);
        let bag_color = &cap["color"];
        let contents = &cap["contains"];
        if contents == "no other bags" {
            input.add_rule(bag_color, None);
        } else {
            for link in contents_re.captures_iter(contents) {
                //println!("{:?}", link);
                let to = &link["color"];
                let number: u32 = link["number"].parse().unwrap();
                input.add_rule(bag_color, Some((to, number)));
            }
            //println!();
        }
    }
    match part {
        Part::First => part1(&input),
        Part::Second => part2(&input),
        Part::All => {
            part1(&input);
            part2(&input);
        },
    }
}

fn part1(rules: &Rules) {
    //println!("{:#?}", rules);
    let mut ancestors = rules.get_parents("shiny gold");
    let mut seen_ancestors: HashSet<_> = ancestors.iter().cloned().collect();
    while let Some(ancestor) = ancestors.pop() {
        //println!("{}", ancestor);
        for grand_ancestor in rules.get_parents(&ancestor) {
            if seen_ancestors.insert(grand_ancestor.clone()) {
                ancestors.push(grand_ancestor.clone());
            }
        };
    }
    println!("{}", seen_ancestors.len());
}

fn part2(rules: &Rules) {
    fn contained_bags(rules: &Rules, bag: &str) -> u32 {
        rules.get_children(bag).iter().map(|Link {name, number}| number*(contained_bags(rules,&name) + 1)).sum()
    }
    println!("{}", contained_bags(rules,"shiny gold"));
}