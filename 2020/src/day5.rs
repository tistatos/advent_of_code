#[macro_use]
extern crate advent_of_code;
use advent_of_code::get_string_rows;
use std::collections::HashSet;

const ROWS: usize = 127;
const ROW_WIDTH: usize = 7;

pub fn main() {
    let input = get_row_input!("5");
    let seat_number = |r: usize, s: usize| r * 8 + s;
    let bsp = |bsp: &mut (usize, usize), f: bool| {
        if f {
            bsp.1 -= (bsp.1 + 1 - bsp.0) / 2;
        } else {
            bsp.0 += (bsp.1 + 1 - bsp.0) / 2
        }
    };

    let mut seats: Vec<usize> = input
        .iter()
        .map(|ticket| {
            let mut row = (0, ROWS);
            let mut seat = (0, ROW_WIDTH);
            for c in ticket.chars() {
                if c == 'F' || c == 'B' {
                    bsp(&mut row, c == 'F');
                }
                if c == 'L' || c == 'R' {
                    bsp(&mut seat, c == 'L');
                }
            }
            seat_number(row.0, seat.0)
        })
        .collect();
    seats.sort();
    let first = seats.first().unwrap();
    let last = seats.last().unwrap();
    println!("day 5 part 1: {}", last);

    let all_seats: HashSet<usize> = (*first..*last).collect();
    let used_seats: HashSet<usize> = seats.drain(..).collect();
    println!("day 5 part 2: {:?}", all_seats.difference(&used_seats));
}
