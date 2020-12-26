use std::{collections::HashSet, fs};

pub fn run() {
    let file = fs::read_to_string("inputs/day09.txt").expect("Failed to read file");
    let lines: Vec<u64> = file.lines().map(|i| i.parse().unwrap()).collect();

    // Part 1
    let num = find_invalid_num(&lines);
    println!("{}", num);

    // Part 2
    let set = find_set(num, &lines);
    println!("{}", set.iter().min().unwrap() + set.iter().max().unwrap());
}

fn find_invalid_num(lines: &Vec<u64>) -> u64 {
    let pre = 25;

    let mut ret = 0;
    for i in pre..lines.len() {
        if !preamble_vals(pre, i, lines).contains(&lines[i]) {
            ret = lines[i];
            break;
        }
    }
    ret
}

fn preamble_vals(pre: usize, i: usize, lines: &Vec<u64>) -> HashSet<u64> {
    let preamble = &lines[i-pre..i];
    let mut vals: HashSet<u64> = HashSet::new();
    for i in preamble {
        for j in preamble {
            if i != j {
                vals.insert(i+j);
            }
        }
    }
    return vals;
}

fn find_set(num: u64, lines: &Vec<u64>) -> Vec<u64> {
    let mut ret: Vec<u64> = Vec::new();
    'o: for i in 0..lines.len()-1 {
        let mut j = i+1;
        let mut sum = lines[i];
        while j != lines.len()-1 || sum < num {
            sum += lines[j];
            j += 1;
            if sum == num {
                ret = lines[i..j].to_vec();
                break 'o;
            }
        }
    }
    ret
}