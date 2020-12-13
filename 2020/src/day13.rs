#[macro_use]
extern crate advent_of_code;

use advent_of_code::get_string_rows;

pub fn main() {
    let input = get_row_input!("13_test");
    let earliest_dep = input[0].parse::<usize>().unwrap();
    let buses: Vec<usize> = input[1]
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .filter_map(|b| b.parse::<usize>().ok())
        .collect();
    let dep = buses
        .iter()
        .map(|b| ((earliest_dep as f32 / *b as f32).ceil() as usize * b, b))
        .min()
        .unwrap();
    println!("day 13 part 1: {:?}", dep.1 * (dep.0 - earliest_dep));
}
