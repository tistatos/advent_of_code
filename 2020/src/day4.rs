extern crate advent_of_code;
use advent_of_code::get_string_rows;

fn valid_passport(passport: &String) -> bool {
    passport.contains("byr")
        && passport.contains("iyr")
        && passport.contains("eyr")
        && passport.contains("hgt")
        && passport.contains("hcl")
        && passport.contains("ecl")
        && passport.contains("pid")
}

fn valid_passport_entries(passport: &String) -> bool {
    if passport.contains("byr")
        && passport.contains("iyr")
        && passport.contains("eyr")
        && passport.contains("hgt")
        && passport.contains("hcl")
        && passport.contains("ecl")
        && passport.contains("pid")
    {
        for entry in passport.split(" ") {
            let e: Vec<&str> = entry.split(":").collect();
            match e[0] {
                "byr" => {
                    let val = e[1].parse::<u32>().unwrap();
                    if val < 1920 || val > 2002 {
                        return false;
                    }
                }
                "iyr" => {
                    let val = e[1].parse::<u32>().unwrap();
                    if val < 2010 || val > 2020 {
                        return false;
                    }
                }
                "eyr" => {
                    let val = e[1].parse::<u32>().unwrap();
                    if val < 2020 || val > 2030 {
                        return false;
                    }
                }
                "hgt" => {
                    let val = e[1];
                    if val.contains("cm") {
                        let index = val.find("cm").unwrap();
                        let h = e[1][0..index].parse::<u32>().unwrap();
                        if h < 150 || h > 193 {
                            return false;
                        }
                    } else if val.contains("in") {
                        let index = val.find("in").unwrap();
                        let h = e[1][0..index].parse::<u32>().unwrap();
                        if h < 59 || h > 76 {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                "hcl" => {
                    let val = e[1];
                    if val.contains("#") {
                        let count = val
                            .chars()
                            .filter(|&c| "1234567890abcdef".contains(c))
                            .count();
                        if count != 6 {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                "ecl" => {
                    let val = e[1];
                    if val.contains("amb")
                        || val.contains("blu")
                        || val.contains("brn")
                        || val.contains("gry")
                        || val.contains("grn")
                        || val.contains("hzl")
                        || val.contains("oth")
                    {
                    } else {
                        return false;
                    }
                }
                "pid" => {
                    let val = e[1];
                    let count = val.chars().filter(|&c| "1234567890".contains(c)).count();
                    if count != 9 {
                        return false;
                    }
                }
                _ => {}
            }
        }
        true
    } else {
        false
    }
}

pub fn main() {
    let input = get_string_rows("input_data/day_4");

    let mut passports = vec![];
    let mut passport = String::new();
    for r in input {
        if r.len() == 0 {
            passports.push(passport);
            passport = "".to_string();
        } else {
            passport = format!("{} {}", passport, r);
        }
    }
    passports.push(passport);

    let valid = passports.iter().filter(|p| valid_passport(&p)).count();
    let valid_entries = passports
        .iter()
        .filter(|p| valid_passport_entries(&p))
        .count();
    println!("day 4 part 1: {}", valid);
    println!("day 4 part 1: {}", valid_entries);
}
