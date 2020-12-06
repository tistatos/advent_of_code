#[macro_use]
extern crate advent_of_code;

use advent_of_code::get_string_row_groups;
use std::collections::HashSet;

pub fn main() {
    let input = get_row_group!("6");

    let group_any_answers_count: usize = input
        .iter()
        .map(|ans| {
            ans.iter()
                .fold(String::new(), |x, acc| format!("{}{}", acc, x))
                .chars()
                .collect::<HashSet<char>>()
                .len()
        })
        .sum();
    let group_all_answers_count: usize = input
        .iter()
        .map(|ans| {
            let mut first = true;
            ans.iter()
                .map(|c| c.chars().collect::<HashSet<char>>())
                .fold(HashSet::new(), |acc, x| {
                    if first {
                        first = false;
                        x
                    } else {
                        let intersection = acc.intersection(&x);
                        let mut hs: HashSet<char> = HashSet::new();
                        for a in intersection {
                            hs.insert(*a);
                        }
                        hs
                    }
                })
        })
        .collect::<Vec<HashSet<char>>>()
        .iter()
        .map(|h| h.len())
        .collect::<Vec<usize>>()
        .iter()
        .sum();

    println!("Day 6 part 1: {:?}", group_any_answers_count);
    println!("Day 6 part 2: {:?}", group_all_answers_count);
}
