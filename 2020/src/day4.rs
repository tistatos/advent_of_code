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
    //passport.contains("cid")
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
    println!("day 4 part 1: {}", valid);
}
