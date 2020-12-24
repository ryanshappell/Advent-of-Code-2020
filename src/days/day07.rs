use std::{collections::{HashMap, HashSet}, fs};
use regex::Regex;

pub fn run() {
    let file = fs::read_to_string("inputs/day07.txt").expect("Failed to read file");

    let mut bags: HashMap<String, HashMap<String, String>> = HashMap::new();
    for line in file.lines() {
        bags.insert(parse_bag(line), parse_rule(line));
    }

    // Part 1
    println!("{}", find_bag(&bags, "shiny gold").len());

    // Part 2
    println!("{:?}", count_bags(&bags, "shiny gold"));
}

fn find_bag(bags: &HashMap<String, HashMap<String, String>>, key: &str) -> HashSet<String> {
    let mut set: HashSet<String> = HashSet::new();
    for bag in bags.iter() {
        if bag.1.contains_key(key) {
            set.insert(bag.0.to_string());
            for i in find_bag(bags, bag.0) {
                set.insert(i);
            }
        }
    }
    return set;
}

fn count_bags(bags: &HashMap<String, HashMap<String, String>>, key: &str) -> i32 {
    let mut count = 0;
    let bag = bags.get(key).unwrap();
    for i in bag.iter() {
        count += i.1.parse::<i32>().unwrap();
        count += i.1.parse::<i32>().unwrap() * count_bags(bags, i.0);
    }
    return count;
}

fn parse_bag(line: &str) -> String {
    let re = Regex::new(r"^(?P<bag>.*?) bags").unwrap();
    let cap = re.captures(line).unwrap();
    return String::from(cap.name("bag").unwrap().as_str());
}

fn parse_rule(line: &str) -> HashMap<String, String> {
    let mut rule: HashMap<String, String> = HashMap::new();
    let re = Regex::new(r"(?P<val>\d) (?P<bag>.*?) bag").unwrap();
    for cap in re.captures_iter(line) {
        rule.insert(String::from(cap.name("bag").unwrap().as_str()), String::from(cap.name("val").unwrap().as_str()));
    }
    return rule;
}
