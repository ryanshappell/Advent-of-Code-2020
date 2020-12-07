use std::fs;

pub fn run() {
    let file = fs::read_to_string("inputs/day04.txt").expect("Failed to read file");
    
    let mut passports: Vec<Passport> = Vec::new();
    passports.push(Passport::blank());

    let mut curr_passport = 0;
    for line in file.lines() {
        if line == "" {
            passports.push(Passport::blank());
            curr_passport += 1;
        } else {
            for p in line.split(' ') {
                let field_name = &p[0..3];
                let field = &p[4..];
                match field_name {
                    "byr" => {
                        passports[curr_passport].byr = match p[4..].parse() {
                            Ok(val) => val,
                            Err(e) => {passports[curr_passport].valid = false; 123456789},
                        };
                        passports[curr_passport].check_byr();
                    },
                    "iyr" => {
                        passports[curr_passport].iyr = match p[4..].parse() {
                            Ok(val) => val,
                            Err(e) => {passports[curr_passport].valid = false; 123456789},
                        };
                        passports[curr_passport].check_iyr();
                    },
                    "eyr" => {
                        passports[curr_passport].eyr = match p[4..].parse() {
                            Ok(val) => val,
                            Err(e) => {passports[curr_passport].valid = false; 123456789},
                        };
                        passports[curr_passport].check_eyr();
                    },
                    "hgt" => {
                        passports[curr_passport].hgt.push_str(field);
                        passports[curr_passport].check_hgt();
                    },
                    "hcl" => {
                        passports[curr_passport].hcl.push_str(field);
                        passports[curr_passport].check_hcl();
                    },
                    "ecl" => {
                        passports[curr_passport].ecl.push_str(field);
                        passports[curr_passport].check_ecl();
                    },
                    "pid" => {
                        passports[curr_passport].pid.push_str(field);
                        passports[curr_passport].check_pid();
                    },
                    "cid" => {
                        passports[curr_passport].cid = match p[4..].parse() {
                            Ok(val) => val,
                            Err(e) => {passports[curr_passport].valid = false; 123456789},
                        }
                    },
                    _ => panic!("WTF HAPPENED WHEN PARSING DATA"),
                }
                if field_name != "cid" {
                    passports[curr_passport].field_count += 1;
                }
            }
        }
    }

    let mut p1_valid_count = 0;
    let mut p2_valid_count = 0;
    for passport in &passports {
        if passport.field_count == 7 {
            p1_valid_count += 1;
            if passport.valid == true {
                p2_valid_count += 1;
            }
        }
    }
    println!("{}", p1_valid_count);
    println!("{}", p2_valid_count);
}
#[derive(Debug)]
struct Passport {
    byr: u32,
    iyr: u32,
    eyr: u32,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: u32,
    valid: bool,
    field_count: u32,
}

impl Passport {
    fn blank() -> Passport {
        Passport {
            byr: 0,
            iyr: 0,
            eyr: 0,
            hgt: String::new(),
            hcl: String::new(),
            ecl: String::new(),
            pid: String::new(),
            cid: 0,
            valid: true,
            field_count: 0,
        }
    }

    fn check_byr(&mut self) {
        if self.byr < 1920 || self.byr > 2002 {
            self.valid = false;
        }
    }

    fn check_iyr(&mut self) {
        if self.iyr < 2010 || self.iyr > 2020 {
            self.valid = false;
        }
    }

    fn check_eyr(&mut self) {
        if self.eyr < 2020 || self.eyr > 2030 {
            self.valid = false;
        }
    }

    fn check_hgt(&mut self) {
        match &self.hgt[self.hgt.len()-2..] {
            "cm" => {
                if self.hgt[..self.hgt.len()-2].parse::<u32>().unwrap() < 150 || self.hgt[..self.hgt.len()-2].parse::<u32>().unwrap() > 193 {
                    self.valid = false;
                }
            },
            "in" => {
                if self.hgt[..self.hgt.len()-2].parse::<u32>().unwrap() < 59 || self.hgt[..self.hgt.len()-2].parse::<u32>().unwrap() > 76 {
                    self.valid = false;
                }
            },
            _ => self.valid = false,
        }
    }

    fn check_hcl(&mut self) {
        if self.hcl.len() != 7 || self.hcl.chars().nth(0).unwrap() != '#' || !self.hcl[1..].chars().all(char::is_alphanumeric) {
            self.valid = false;
        }
    }

    fn check_ecl(&mut self) {
        let eye_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        if !eye_colors.contains(&self.ecl.as_str()) {
            self.valid = false;
        }
    }

    fn check_pid(&mut self) {
        if self.pid.len() != 9 {
            self.valid = false;
        }
    }
}