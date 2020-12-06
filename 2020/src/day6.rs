#![feature(iterator_fold_self)]

#[macro_use]
extern crate advent_of_code;

use advent_of_code::get_string_row_groups;
use std::collections::HashSet;

pub fn main() {
    let input = get_row_group!("6");

    let group_any_answers_count: usize = input.iter().fold(0, |acc, ans| {
        acc + ans.join("").chars().collect::<HashSet<char>>().len()
    });

    let group_all_answers_count: usize = input.iter().fold(0, |acc, ans| {
        acc + ans
            .iter()
            .map(|c| c.chars().collect::<HashSet<char>>())
            .fold_first(|acc, x| acc.intersection(&x).into_iter().cloned().collect())
            .unwrap_or(HashSet::new())
            .len()
    });

    println!("Day 6 part 1: {:?}", group_any_answers_count);
    println!("Day 6 part 2: {:?}", group_all_answers_count);
}
