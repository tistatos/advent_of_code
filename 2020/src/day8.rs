#[macro_use]
extern crate advent_of_code;

use std::collections::HashSet;

use advent_of_code::get_string_rows;

fn parse_command(command: &String, ip: &mut i64, acc: &mut i64) {
    let split: Vec<&str> = command.split(" ").collect();
    let inst = split[0];
    let operand = split[1].parse::<i64>().unwrap();
    match inst {
        "nop" => {
            *ip += 1;
        }
        "jmp" => {
            *ip += operand;
        }
        "acc" => {
            *ip += 1;
            *acc += operand;
        }
        _ => panic!("unrecognized instruction"),
    }
}

pub fn main() {
    let mut input = get_row_input!("8");
    let mut previous_ip = HashSet::<i64>::new();
    let mut ip = 0;
    let mut acc = 0;

    loop {
        parse_command(&input[ip as usize], &mut ip, &mut acc);
        if !previous_ip.insert(ip) {
            println!("Day 8 part 1: {}", acc);
            break;
        }
    }

    let mut orig = String::new();
    let patch: String = "nop +0".to_string();
    let mut patch_i = 0;

    loop {
        ip = 0;
        acc = 0;
        previous_ip.clear();

        for i in patch_i..input.len() {
            let ref r = input[i];
            if r.find("jmp").is_some() {
                orig = r.clone();
                input[i] = patch.clone();
                patch_i = i;
                break;
            }
        }
        loop {
            if ip as usize >= input.len() {
                println!("Day 8 part 2: {}", acc);
                return;
            }
            parse_command(&input[ip as usize], &mut ip, &mut acc);
            if !previous_ip.insert(ip) {
                input[patch_i] = orig.clone();
                patch_i += 1;
                break;
            }
        }
    }
}
