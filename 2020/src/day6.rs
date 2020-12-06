#[macro_use]
extern crate advent_of_code;

use advent_of_code::get_string_row_groups;
use std::collections::HashSet;

pub fn main() {
    let input = get_row_group!("6");

    let group_answers: usize = input
        .iter()
        .map(|ans| {
            ans.iter()
                .fold(String::new(), |x, acc| format!("{}{}", acc, x))
                .chars()
                .collect::<HashSet<char>>()
                .len()
        })
        .sum();
    println!("Day 6 part 1: {:?}", group_answers);
}
