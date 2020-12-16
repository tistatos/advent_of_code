#[macro_use]
extern crate advent_of_code;

use std::collections::HashMap;

use advent_of_code::get_csv;

fn play_game(start: &Vec<usize>, iterations: usize) -> usize {
    let mut previous = HashMap::<usize, usize>::new();
    for i in 0..start.len() - 1 {
        previous.insert(start[i], i + 1);
    }

    let mut last_digit = *start.last().unwrap();
    for i in start.len() + 1..iterations + 1 {
        match previous.get(&last_digit) {
            Some(digit_i) => {
                let index = *digit_i;
                previous.insert(last_digit, i - 1);
                last_digit = (i - 1) - index;
            }
            None => {
                previous.insert(last_digit, i - 1);
                last_digit = 0;
            }
        };
    }
    last_digit
}

pub fn main() {
    let start_numbers: Vec<usize> = get_csv_input!("15", ",", usize);
    println!("Day 15 part 1: {}", play_game(&start_numbers, 2020));
    println!("Day 15 part 2: {}", play_game(&start_numbers, 30000000));
}
