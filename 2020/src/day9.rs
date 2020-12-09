#[macro_use]
extern crate advent_of_code;

use advent_of_code::get_rows;

pub fn main() {
    let input: Vec<usize> = get_row_input!("9", usize);
    let window_size = 25;
    let number: usize = input
        .windows(window_size + 1)
        .skip_while(|d| {
            let target = d[window_size];
            d.iter().enumerate().any(|(i, a)| {
                d.iter()
                    .enumerate()
                    .any(|(j, b)| if i == j { false } else { *a + *b == target })
            })
        })
        .next()
        .map(|r| *r.last().unwrap())
        .unwrap();
    println!("Day 9 part 1: {:?}", number);

    let index = input.iter().position(|&i| i == number).unwrap();
    let mut window_size = 2;
    loop {
        match input[..index]
            .windows(window_size)
            .skip_while(|c| c.iter().sum::<usize>() != number)
            .next()
        {
            Some(v) => {
                let sum = v.iter().min().unwrap() + v.iter().max().unwrap();
                println!("Day 9 part 2: {} ", sum);
                break;
            }
            None => {
                window_size += 1;
            }
        }
    }
}
