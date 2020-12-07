use std::fs;

pub fn run() {
    let file = fs::read_to_string("inputs/test06.txt").expect("Failed to read file");

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
    
}
