use std::{collections::HashSet, fs};

pub fn run() {
    let file = fs::read_to_string("inputs/test10.txt").expect("Failed to read file");

    let mut adapters: Vec<u32> = file.lines().map(|i| i.parse::<u32>().unwrap()).collect();
    adapters.push(0);
    adapters.sort();
    adapters.push(adapters.last().unwrap() + 3);

    // Part 1
    let mut ones = 0;
    let mut threes = 0;
    for i in 0..adapters.len()-1 {
        if adapters[i]+1 == adapters[i+1] {
            ones += 1;
        } else {
            threes += 1;
        }
    }
    println!("{} * {} = {}", ones, threes, ones * threes);

    // Part 2
    
}

fn find_arrangements(adapters: &Vec<u32>) -> HashSet<Vec<u32>> {
    let mut set: HashSet<Vec<u32>> = HashSet::new();
    for i in (1..adapters.len()-1).rev() {
        match find_remove(&adapters, i) {
            Some(s) => {
                set.insert(s.clone());
                for a in find_arrangements(&s).iter() {
                    set.insert(a.to_vec());
                }
            },
            None => (),
        }
    }
    return set;
}

fn find_remove(adapters: &Vec<u32>, i: usize) -> Option<Vec<u32>> {
    if adapters[i+1] - adapters[i-1] <= 3 {
        let mut temp = adapters.clone();
        temp.remove(i);
        return Some(temp);
    }
    None
}
