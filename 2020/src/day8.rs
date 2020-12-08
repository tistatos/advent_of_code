#[macro_use]
extern crate advent_of_code;

use std::collections::HashSet;

use advent_of_code::get_string_rows;

fn parse_command(command: &String, ip: &mut i64, acc: &mut i64) {
    let split: Vec<&str> = command.split(" ").collect();
    let inst = split[0];
    let operand = split[1].parse::<i64>().unwrap();
    println!("{}: {} {}", ip, inst, operand);
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
    let input = get_row_input!("8");
    let mut previous_ip = HashSet::<i64>::new();
    let mut ip = 0;
    let mut acc = 0;
    loop {
        parse_command(&input[ip as usize], &mut ip, &mut acc);

        if !previous_ip.contains(&ip) {
            previous_ip.insert(ip);
        } else {
            println!("Day 8 part 1: {}", acc);
            break;
        }
    }
}
