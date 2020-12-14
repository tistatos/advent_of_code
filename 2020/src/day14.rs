#[macro_use]
extern crate advent_of_code;

use std::collections::HashMap;

use advent_of_code::get_string_rows;

const BIT_MASK_LENGTH: usize = 36;

pub fn main() {
    let input = get_row_input!("14");
    let mut mask: Vec<char> = vec![];
    let mut memory = HashMap::<usize, u64>::new();
    for r in input {
        match r.find("mask") {
            Some(_) => {
                mask = r.split(" = ").collect::<Vec<&str>>()[1].chars().collect();
            }
            None => {}
        };

        match r.find("mem") {
            Some(_) => {
                let row: Vec<&str> = r
                    .split(|c| "[]= ".contains(c))
                    .filter(|s| s.len() > 0)
                    .collect::<Vec<&str>>();
                let mem = row[1].parse::<usize>().unwrap();
                let val = row[2].parse::<u64>().unwrap();
                let masked_value: u64 =
                    mask.iter().enumerate().fold(0, |acc, (index, mask_bit)| {
                        let bit_index = (BIT_MASK_LENGTH - index - 1) as u64;
                        let bit = val >> bit_index & 0x01;
                        match mask_bit {
                            '1' => acc + (1 << bit_index),
                            'X' => acc + (bit << bit_index),
                            _ => acc,
                        }
                    });
                memory.insert(mem, masked_value);
            }
            None => {}
        };
    }
    println!(
        "Day 14 part 1: {:?}",
        memory.iter().fold(0, |acc, m| acc + m.1)
    );
}
