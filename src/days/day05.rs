use std::fs;

pub fn run() {
    let file = fs::read_to_string("inputs/test05.txt").expect("Failed to read file");

    // Part 1
    let mut seats: Vec<Seat> = file.lines().map(|line| find_seat(line)).collect();
    seats.sort_by(|a, b| a.id.cmp(&b.id));
    println!("{:?}", &seats[0].id);

    // Part 2
    for i in 1..seats.len()-1 {
        if seats[i].id + 1 != seats[i+1].id {
            println!("{:?}", seats[i].id + 1);
            break;
        }
    }
}

fn find_seat(bp: &str) -> Seat {
    let mut seat = Seat {
        row_l: 0,
        row_u: 127,
        col_l: 0,
        col_u: 7,
        id: 0,
    };
    for c in bp.chars() {
        match c {
            'F' => seat.row_u -= (seat.row_u - seat.row_l + 1) / 2,
            'B' => seat.row_l += (seat.row_u - seat.row_l + 1) / 2,
            'L' => seat.col_u -= (seat.col_u - seat.col_l + 1) / 2,
            'R' => seat.col_l += (seat.col_u - seat.col_l + 1) / 2,
            _ => (),
        }
    }
    seat.id = seat.row_l * 8 + seat.col_l;
    return seat;
}

#[derive(Debug)]
struct Seat {
    row_l: u32,
    row_u: u32,
    col_l: u32,
    col_u: u32,
    id: u32,
}
