#[macro_use]
extern crate advent_of_code;

use advent_of_code::get_rows;

pub fn main() {
    let mut input: Vec<usize> = get_row_input!("10", usize);

    input.sort();
    input.insert(0, 0);
    input.push(input.last().unwrap() + 3);

    let diffs: Vec<usize> = input.windows(2).map(|j| j[1] - j[0]).collect();
    let (ones, threes): (Vec<usize>, Vec<usize>) = diffs.iter().partition(|d| **d == 1);
    println!("day 10 part 1: {}", ones.len() * threes.len());

    let mut diffs: Vec<usize> = input
        .windows(3)
        .filter_map(|j| if j[2] - j[0] <= 3 { Some(j[1]) } else { None })
        .collect();
    diffs.push(*input.last().unwrap());

    let mut perm = 0;
    let res: u64 = diffs
        .windows(2)
        .fold(vec![0, 0, 0], |mut acc, x| {
            if x[1] - x[0] == 1 {
                perm += 1;
                acc
            } else {
                acc[perm] += 1;
                perm = 0;
                acc
            }
        })
        .iter()
        .zip(vec![2, 4, 7])
        .map(|(x, f)| (f as u64).pow(*x))
        .product();
    println!("day 10 part 2: {}", res);
}
