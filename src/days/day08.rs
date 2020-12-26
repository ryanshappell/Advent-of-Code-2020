use std::{collections::HashSet, fs};
use regex::Regex;

pub fn run() {
    let file = fs::read_to_string("inputs/day08.txt").expect("Failed to read file");

    let lines: Vec<&str> = file.lines().collect();

    // Part 1
    p1(&lines);

    // Part 2
    p2(&lines);
}

fn p1(lines: &Vec<&str>) {
    let mut curr_line: i32 = 0;
    let mut accumulator = 0;
    let mut run_lines: HashSet<i32> = HashSet::new();

    loop {
        // Check for an instruction being called twice
        if run_lines.contains(&curr_line) {
            println!("{}", accumulator);
            break;
        } else {
            run_lines.insert(curr_line);
        }

        // Instruction parsing and execution
        let re = Regex::new(r"(?P<op>.*?) (?P<sign>[+-])(?P<arg>\d*)").unwrap();
        let caps = re.captures(lines[curr_line as usize]).unwrap();
        match caps.name("op").unwrap().as_str() {
            "acc" => {
                accumulator += acc(caps.name("sign").unwrap().as_str(), caps.name("arg").unwrap().as_str().parse::<i32>().unwrap());
                curr_line += 1;
            },
            "jmp" => curr_line += jmp(caps.name("sign").unwrap().as_str(), caps.name("arg").unwrap().as_str().parse::<i32>().unwrap()),
            _ => curr_line += 1,
        }
    }
}

fn p2(lines: &Vec<&str>) {    
    'o: for i in 0..lines.len() {
        let mut curr_line: i32 = 0;
        let mut accumulator = 0;
        let mut run_lines: HashSet<i32> = HashSet::new();

        loop {
            let last = curr_line == (lines.len() - 1) as i32;
            
            // Check for an instruction being called twice
            if run_lines.contains(&curr_line) {
                break;
            } else {
                run_lines.insert(curr_line);
            }
    
            let re = Regex::new(r"(?P<op>.*?) (?P<sign>[+-])(?P<arg>\d*)").unwrap();
            let caps = re.captures(lines[curr_line as usize]).unwrap();
            match caps.name("op").unwrap().as_str() {
                "acc" => {
                    accumulator += acc(caps.name("sign").unwrap().as_str(), caps.name("arg").unwrap().as_str().parse::<i32>().unwrap());
                    curr_line += 1;
                },
                "jmp" => {
                    if curr_line == i as i32 {
                        curr_line += 1;
                    } else {
                        curr_line += jmp(caps.name("sign").unwrap().as_str(), caps.name("arg").unwrap().as_str().parse::<i32>().unwrap());
                    }
                },
                _ => {
                    if curr_line == i as i32 {
                        curr_line += jmp(caps.name("sign").unwrap().as_str(), caps.name("arg").unwrap().as_str().parse::<i32>().unwrap());
                    } else {
                        curr_line += 1;
                    }
                },
            }
    
            if last {
                println!("Program booted correctly: {}", accumulator);
                break 'o;
            }
        }
    }
}

fn acc(sign: &str, arg: i32) -> i32 {
    if sign == "-" {
        return -1 * arg;
    } else {
        return arg;
    }
}

fn jmp(sign: &str, arg: i32) -> i32 {
    if sign == "-" {
        return -1 * arg;
    } else {
        return arg;
    }
}