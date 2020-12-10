use std::{collections::HashSet, fs};

pub fn run() {
    let file = fs::read_to_string("inputs/day06.txt").expect("Failed to read file");

    // Part 1
    let mut count = 0;
    let mut curr_group = String::new();
    for line in file.lines() {
        if line == "" {
            count += curr_group.len();
            curr_group.clear();
        } else {
            for c in line.chars() {
                if !curr_group.contains(c) {
                    curr_group.push(c);
                }
            }
        }
    }
    count += curr_group.len();
    println!("{}", count);

    // Part 2
    count = 0;
    let mut curr_group: HashSet<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    for line in file.lines() {
        if line == "" {
            count += curr_group.len();
            curr_group = "abcdefghijklmnopqrstuvwxyz".chars().collect();
        } else {
            curr_group = &curr_group & &line.chars().collect();
        }
    }
    count += curr_group.len();
    println!("{}", count);
}
