#[macro_use]
extern crate advent_of_code;

use advent_of_code::get_string_rows;

pub fn main() {
    let input = get_row_input!("13");
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

    let mut buses: Vec<(usize, usize)> = input[1]
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .enumerate()
        .filter_map(|(i, b)| match b.parse::<usize>() {
            Ok(n) => Some((n, i)),
            _ => None,
        })
        .collect();
    println!("{:?}", buses);

    // copied from the internet, original iterated over largest bus id
    // to find periodity
    let mut step = 1;
    let res = buses.iter().fold(0, |mut x, (n, o)| {
        let mut r = *n as i64 - (*o as i64) % *n as i64;
        if r == *n as i64 {
            r = 0;
        }

        let fac = x as i64 % (*n as i64);
        while x as i64 % (*n as i64) != r {
            x += step;
        }
        step *= n;
        x
    });
    println!("{}", res);
}
