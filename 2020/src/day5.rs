extern crate advent_of_code;
use advent_of_code::get_string_rows;

const ROWS: usize = 127;
const ROW_WIDTH: usize = 7;

pub fn main() {
    let input = get_string_rows("input_data/day_5");
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
    println!("day 5 part 1: {}", seats.last().unwrap());
}
