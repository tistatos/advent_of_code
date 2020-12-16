#[macro_use]
extern crate advent_of_code;

use std::collections::HashMap;

use advent_of_code::get_string_rows;

const BIT_MASK_LENGTH: usize = 36;

fn permutations(result: &mut Vec<usize>, input: Vec<usize>, index: usize) {
    let val = input[index];
    let prev: Vec<usize> = result.iter().map(|v| v + val).collect();
    for p in prev {
        result.push(p);
    }
    result.push(input[index]);
    if index + 1 < input.len() {
        permutations(result, input, index + 1);
    }
}

pub fn main() {
    let input = get_row_input!("14");
    let mut mask: Vec<char> = vec![];
    let mut memory = HashMap::<usize, u64>::new();

    let parse_mask = |row: &String| -> Vec<char> {
        row.split(" = ").collect::<Vec<&str>>()[1].chars().collect()
    };
    let parse_mem = |row: &String| -> (usize, u64) {
        let splitted: Vec<&str> = row
            .split(|c| "[]= ".contains(c))
            .filter(|s| s.len() > 0)
            .collect::<Vec<&str>>();
        (
            splitted[1].parse::<usize>().unwrap(),
            splitted[2].parse::<u64>().unwrap(),
        )
    };
    for r in &input {
        match r.find("mask") {
            Some(_) => {
                mask = parse_mask(&r);
            }
            None => {}
        };

        match r.find("mem") {
            Some(_) => {
                let (mem, val) = parse_mem(&r);
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

    memory.clear();

    for r in input {
        match r.find("mask") {
            Some(_) => {
                mask = parse_mask(&r);
            }
            None => {}
        };

        match r.find("mem") {
            Some(_) => {
                let (mem, val) = parse_mem(&r);
                let intermediate_addr: usize =
                    mask.iter().enumerate().fold(0, |acc, (index, mask_bit)| {
                        let bit_index = (BIT_MASK_LENGTH - index - 1);
                        let bit = mem >> bit_index & 0x01;
                        match mask_bit {
                            '1' => acc + (1 << bit_index),
                            '0' => acc + (bit << bit_index),
                            _ => acc,
                        }
                    });
                let addr_mods: Vec<usize> = mask
                    .iter()
                    .enumerate()
                    .filter_map(|(index, bit)| {
                        let bit_index = (BIT_MASK_LENGTH - index - 1);
                        match bit {
                            'X' => Some((1 << bit_index)),
                            _ => None,
                        }
                    })
                    .collect();
                let mut mods = vec![0];
                permutations(&mut mods, addr_mods, 0);
                for m in mods {
                    memory.insert(intermediate_addr + m, val);
                }
            }
            None => {}
        };
    }

    println!(
        "Day 14 part 2: {:?}",
        memory.iter().fold(0, |acc, m| acc + m.1)
    );
}
