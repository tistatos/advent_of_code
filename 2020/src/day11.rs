#[macro_use]
extern crate advent_of_code;

use advent_of_code::get_string_rows;

const FREE: char = 'L';
const OCCUPIED: char = '#';
const FLOOR: char = '.';

type Seating = Vec<Vec<char>>;

fn evaluate(
    map: &Seating,
    occupied_limit: usize,
    seat_fn: fn(&Seating, usize, usize, (i32, i32), &mut usize),
) -> (Seating, usize) {
    let mut changes = 0;
    let mut result = map.clone();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let mut occupied_count = 0;
            let seat = map[y][x];
            if seat == FLOOR {
                continue;
            }

            for i in -1..2 {
                for j in -1..2 {
                    if i == 0 && j == 0 {
                        continue;
                    }
                    seat_fn(&map, y, x, (i, j), &mut occupied_count);
                }
            }

            if occupied_count == 0 && seat == FREE {
                result[y][x] = OCCUPIED;
                changes += 1;
            } else if occupied_count >= occupied_limit && seat == OCCUPIED {
                result[y][x] = FREE;
                changes += 1;
            }
        }
    }
    (result, changes)
}

fn calculate_occupied(map: Vec<Vec<char>>) -> usize {
    map.iter().flatten().filter(|s| **s == OCCUPIED).count()
}

fn closest_tile(seats: &Seating, y: usize, x: usize, dir: (i32, i32), count: &mut usize) {
    let y = y as i32 + dir.0;
    let x = x as i32 + dir.1;
    let height = seats.len();
    let width = seats[0].len();
    if x < 0 || y < 0 || y as usize >= height || x as usize >= width {
        return;
    }
    let seat = seats[y as usize][x as usize];
    if seat == OCCUPIED {
        *count += 1;
    }
}

fn closest_seat(seats: &Seating, y: usize, x: usize, dir: (i32, i32), count: &mut usize) {
    let y = y as i32 + dir.0;
    let x = x as i32 + dir.1;
    let height = seats.len();
    let width = seats[0].len();
    if x < 0 || y < 0 || y as usize >= height || x as usize >= width {
        return;
    }

    let seat = seats[y as usize][x as usize];
    if seat == OCCUPIED {
        *count += 1;
    } else if seat == FLOOR {
        closest_seat(&seats, y as usize, x as usize, dir, count)
    }
}

pub fn main() {
    let input: Seating = get_row_input!("11")
        .iter()
        .map(|s| s.chars().collect())
        .collect();

    let mut res = evaluate(&input, 4, closest_tile);
    while res.1 > 0 {
        res = evaluate(&res.0, 4, closest_tile);
    }
    println!("Day 11 part 1: {}", calculate_occupied(res.0));

    let mut res = evaluate(&input, 5, closest_seat);
    while res.1 > 0 {
        res = evaluate(&res.0, 5, closest_seat);
    }
    println!("Day 11 part 2: {}", calculate_occupied(res.0));
}
