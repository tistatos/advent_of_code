use std::collections::HashSet;
use std::iter::FromIterator;

use advent_of_code::get_row_input;
use advent_of_code::get_string;
use advent_of_code::get_string_input;
use advent_of_code::get_string_rows;

pub fn main() {
    let buffer = get_string_input!("6");

    let find_idx = |buffer: &String, win| {
        buffer
            .chars()
            .collect::<Vec<char>>()
            .windows(win)
            .enumerate()
            .find_map(|(i, w)| {
                if HashSet::<&char>::from_iter(w.iter()).len() == win {
                    Some(i + win)
                } else {
                    None
                }
            })
            .unwrap()
    };

    println!("Solution 1: {}", find_idx(&buffer, 4));
    println!("Solution 2: {}", find_idx(&buffer, 14));
}
