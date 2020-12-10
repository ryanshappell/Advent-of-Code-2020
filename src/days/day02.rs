use std::fs;

pub fn run() {
    let file = fs::read_to_string("inputs/day02.txt").expect("Failed to read file");
    let data: Vec<&str> = file.lines().collect();

    // Part 1
    let mut valid_count = 0;

    for line in &data {
        let mut split_line: Vec<&str> = line.split(|x| x == '-' || x == ':' || x == ' ').collect();
        split_line.remove(3);

        let lower = split_line[0].parse::<i32>().expect("Could not parse to i32");
        let upper = split_line[1].parse::<i32>().expect("Could not parse to i32");
        let letter = split_line[2].parse::<char>().expect("Could not parse to char");
        let password = split_line[3];

        let mut count = 0;
        for i in password.chars() {
            if i == letter {
                count += 1;
            }
        }
        if count >= lower && count <= upper {
            valid_count += 1;
        }
    }
    println!("{}", valid_count);

    // Part 2
    valid_count = 0;

    for line in &data {
        let mut split_line: Vec<&str> = line.split(|x| x == '-' || x == ':' || x == ' ').collect();
        split_line.remove(3);

        let pos_1 = split_line[0].parse::<usize>().expect("Could not parse to usize");
        let pos_2 = split_line[1].parse::<usize>().expect("Could not parse to usize");
        let letter: char = split_line[2].parse().expect("Could not parse to char");
        let password = split_line[3];
        
        if password.chars().nth(pos_1 - 1).unwrap() == letter && password.chars().nth(pos_2 - 1).unwrap() != letter {
            valid_count += 1;
        }
        if password.chars().nth(pos_1 - 1).unwrap() != letter && password.chars().nth(pos_2 - 1).unwrap() == letter {
            valid_count += 1;
        }
    }
    println!("{}", valid_count);
}