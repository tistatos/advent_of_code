#[macro_use]
extern crate advent_of_code;
use advent_of_code::get_string_rows;

fn valid_password(pass: &String, key: char, limit: &Vec<usize>) -> bool {
    let key_count: usize = pass
        .chars()
        .filter(|c| *c == key)
        .collect::<Vec<char>>()
        .len();
    key_count >= limit[0] && key_count <= limit[1]
}

fn valid_password_position(pass: &String, key: char, position: &Vec<usize>) -> bool {
    let check_char = |pass: &String, pos: usize| match pass.chars().nth(pos) {
        Some(c) => c == key,
        None => false,
    };
    check_char(pass, position[0] - 1) ^ check_char(pass, position[1] - 1)
}
struct PasswordSet {
    password: String,
    key: char,
    requirement: Vec<usize>,
}

pub fn main() {
    let rows = get_row_input!("2");
    let passwords: Vec<PasswordSet> = rows
        .iter()
        .map(|r| {
            let parts: Vec<&str> = r.split(" ").collect();
            let requirement: Vec<usize> = parts[0]
                .split("-")
                .map(|v| v.parse::<usize>().unwrap())
                .collect();
            let key = parts[1].chars().next().unwrap();
            let password = parts[2].to_string();
            PasswordSet {
                password,
                key,
                requirement,
            }
        })
        .collect();

    let valid_first: usize = passwords
        .iter()
        .map(|p| valid_password(&p.password, p.key, &p.requirement) as usize)
        .sum();
    println!("Day 2 part 1: {}", valid_first);

    let valid_second: usize = passwords
        .iter()
        .map(|p| valid_password_position(&p.password, p.key, &p.requirement) as usize)
        .sum();
    println!("Day 2 part 2: {}", valid_second);
}
