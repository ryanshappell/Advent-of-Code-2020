use std::fs;

pub fn run() {
    let file = fs::read_to_string("inputs/day01.txt").expect("Failed to read file");
    let data: Vec<i32> = file.lines().map(|line| line.parse::<i32>().expect("Failed to parse to i32")).collect();

    // Part 1
    'o1: for i in &data {
        for j in &data {
            if i + j == 2020 {
                println!("{} + {} = {}", i, j, i + j);
                println!("{} * {} = {}", i, j, i * j);
                break 'o1;
            }
        }
    }

    // Part 2
    'o2: for i in &data {
        for j in &data {
            for k in &data {
                if i + j + k == 2020 {
                    println!("{} + {} + {} = {}", i, j, k, i + j + k);
                    println!("{} * {} * {} = {}", i, j, k, i * j * k);
                    break 'o2;
                }
            }
        }
    }
}
