#[macro_use]
extern crate advent_of_code;

use advent_of_code::get_rows;

pub fn main() {
    let input: Vec<usize> = get_row_input!("9", usize);
    let window_size = 25 + 1;
    let c: Vec<usize> =
        input
            .windows(window_size)
            .filter(|d| {
                let target = d[25];
                let c =
                    d.iter()
                        .enumerate()
                        .filter(|(_, a)| **a < target)
                        .filter(|(i, a)| {
                            d.iter().enumerate().any(|(j, b)| {
                                if *i == j {
                                    false
                                } else {
                                    target - *a == *b
                                }
                            })
                        })
                        .count();
                c == 0
            })
            .map(|r| *r.last().unwrap())
            .collect();
    println!("Day 9 part 1: {:?}", c);
}
