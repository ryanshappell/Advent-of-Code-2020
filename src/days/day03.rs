use std::fs;

pub fn run() {
    let file = fs::read_to_string("inputs/day03.txt").expect("Failed to read file");
    let data: Vec<&str> = file.lines().collect();

    // Part 1
    println!("{}", count_trees(&data, 3, 1));

    // Part 2
    let slopes = [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]];
    let mut result = 1;
    for slope in slopes.iter() {
        result *= count_trees(&data, slope[0], slope[1]);
    }
    println!("{}", result);
}

fn count_trees(data: &Vec<&str>, dx: usize, dy:usize) -> i32 {
    let mut sled = Sled {
        x: 0,
        y: 0,
        dx,
        dy,
    };

    let mut count = 0;
    while sled.y < data.len() {
        if data[sled.y].chars().nth(sled.x % data[0].len()).unwrap() == '#' {
            count += 1;
        }
        sled.x += sled.dx;
        sled.y += sled.dy;
    }
    return count;
}

struct Sled {
    x: usize,
    y: usize,
    dx: usize,
    dy: usize,
}