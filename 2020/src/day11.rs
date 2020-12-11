#[macro_use]
extern crate advent_of_code;

use advent_of_code::get_string_rows;

const FREE: char = 'L';
const OCCUPIED: char = '#';
const FLOOR: char = '.';

type Seating = Vec<Vec<char>>;

fn evaluate(
    map: Seating,
    width: usize,
    height: usize,
    occupied_limit: usize,
    seat_fn: fn(&Seating, usize, usize, &mut usize) -> bool,
) -> (Seating, usize) {
    let mut changes = 0;
    let mut result = map.clone();
    for y in 0..map.len() {
        let ref row = map[y];
        for x in 0..row.len() {
            let mut occupied_count = 0;
            let seat = map[y][x];
            if seat == FLOOR {
                continue;
            }

            if x > 0 {
                seat_fn(&map, y, x - 1, &mut occupied_count);
                if y > 0 {
                    seat_fn(&map, y - 1, x - 1, &mut occupied_count);
                }
                if y < height - 1 {
                    seat_fn(&map, y + 1, x - 1, &mut occupied_count);
                }
            }
            if y > 0 {
                seat_fn(&map, y - 1, x, &mut occupied_count);
            }
            if y < height - 1 {
                seat_fn(&map, y + 1, x, &mut occupied_count);
            }
            if x < width - 1 {
                seat_fn(&map, y, x + 1, &mut occupied_count);
                if y > 0 {
                    seat_fn(&map, y - 1, x + 1, &mut occupied_count);
                }
                if y < height - 1 {
                    seat_fn(&map, y + 1, x + 1, &mut occupied_count);
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

fn closest_seat(seats: &Seating, y: usize, x: usize, count: &mut usize) -> bool {
    let seat = seats[y][x];
    if seat == OCCUPIED {
        *count += 1;
        true
    } else if seat == FREE {
        true
    } else {
        false
    }
}

pub fn main() {
    let input: Seating = get_row_input!("11")
        .iter()
        .map(|s| s.chars().collect())
        .collect();
    let height = input.len();
    let width = input[0].len();

    let mut res = evaluate(input, width, height, 4, closest_seat);
    let mut changes = res.1;
    while changes > 0 {
        res = evaluate(res.0, width, height, 4, closest_seat);
        changes = res.1;
    }
    println!("Day 11 part 1: {}", calculate_occupied(res.0));
}
