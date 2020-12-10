#[macro_use]
extern crate advent_of_code;

use advent_of_code::get_rows;

pub fn main() {
    let mut input: Vec<usize> = get_row_input!("10", usize);

    input.sort();
    input.insert(0, 0);
    println!("{:?}", input);
    let diffs: Vec<usize> = input.windows(2).map(|j| j[1] - j[0]).collect();
    let ones: usize = diffs.iter().filter(|d| **d == 1).count();
    let threes: usize = diffs.iter().filter(|d| **d == 3).count();

    println!("day 10 ones: {}", ones);
    println!("day 10 threes: {}", threes + 1);

    println!("day 10 part 1: {}", ones * (threes + 1));
}
