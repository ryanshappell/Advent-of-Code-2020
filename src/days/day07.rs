use std::fs;
use regex::Regex;

pub fn run() {
    let file = fs::read_to_string("inputs/test07.txt").expect("Failed to read file");
    
    // Parse rules
    for line in file.lines() {
        let re = Regex::new(r"contain").unwrap();
        let test = re.captures(line);
        println!("{:?}", test);
    }
}
