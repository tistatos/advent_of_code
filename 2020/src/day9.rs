#[macro_use]
extern crate advent_of_code;

use advent_of_code::get_rows;

pub fn main() {
    let input: Vec<usize> = get_row_input!("9", usize);
    let window_size = 25;
    let number: usize =
        *input
            .windows(window_size + 1)
            .filter(|d| {
                let target = d[window_size];
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
            .collect::<Vec<usize>>()
            .first()
            .unwrap();
    println!("Day 9 part 1: {:?}", number);
    let index = input.iter().position(|&i| i == number).unwrap();
    let mut window_size = 2;
    loop {
        let sum: Vec<usize> = input[..index]
            .windows(window_size)
            .filter(|c| c.iter().sum::<usize>() == number)
            .map(|c| c.iter().min().unwrap() + c.iter().max().unwrap())
            .collect();
        if sum.len() == 1 {
            println!("Day 9 part 2: {} ", sum.first().unwrap());
            break;
        }
        window_size += 1;
    }
}
