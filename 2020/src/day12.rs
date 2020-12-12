#[macro_use]
extern crate advent_of_code;

use advent_of_code::{get_string_rows, manhattan_distance};

pub fn main() {
    let input: Vec<(char, i32)> = get_row_input!("12")
        .into_iter()
        .map(|mut s| {
            let n = s.split_off(1);
            (s.chars().nth(0).unwrap(), n.parse::<i32>().unwrap())
        })
        .collect();
    let final_pos = input.iter().fold((0, 0, 90), |mut acc, (dir, dist)| {
        match dir {
            'N' => acc.1 += dist,
            'E' => acc.0 += dist,
            'S' => acc.1 -= dist,
            'W' => acc.0 -= dist,
            'L' => {
                acc.2 -= dist;
                if acc.2 < 0 {
                    acc.2 += 360;
                }
            }
            'R' => {
                acc.2 += dist;
                if acc.2 >= 360 {
                    acc.2 -= 360;
                }
            }
            'F' => match acc.2 {
                0 => acc.1 += dist,
                90 => acc.0 += dist,
                180 => acc.1 -= dist,
                270 => acc.0 -= dist,
                _ => {}
            },
            _ => {}
        }
        acc
    });
    println!(
        "day 12 part 1: {}",
        manhattan_distance((final_pos.0, final_pos.1), (0, 0))
    );
    let final_pos = input.iter().fold((0, 0, 10, 1), |mut acc, (dir, dist)| {
        match dir {
            'N' => acc.3 += dist,
            'E' => acc.2 += dist,
            'S' => acc.3 -= dist,
            'W' => acc.2 -= dist,
            'L' => match dist {
                90 => {
                    let x = acc.2;
                    let y = acc.3;
                    acc.2 = -y;
                    acc.3 = x;
                }
                180 => {
                    let x = acc.2;
                    let y = acc.3;
                    acc.2 = -x;
                    acc.3 = -y;
                }
                270 => {
                    let x = acc.2;
                    let y = acc.3;
                    acc.2 = y;
                    acc.3 = -x;
                }
                _ => {}
            },
            'R' => match dist {
                90 => {
                    let x = acc.2;
                    let y = acc.3;
                    acc.2 = y;
                    acc.3 = -x;
                }
                180 => {
                    let x = acc.2;
                    let y = acc.3;
                    acc.2 = -x;
                    acc.3 = -y;
                }
                270 => {
                    let x = acc.2;
                    let y = acc.3;
                    acc.2 = -y;
                    acc.3 = x;
                }
                _ => {}
            },
            'F' => {
                acc.0 += acc.2 * dist;
                acc.1 += acc.3 * dist;
            }
            _ => {}
        }
        acc
    });
    println!(
        "day 12 part 2: {}",
        manhattan_distance((final_pos.0, final_pos.1), (0, 0))
    );
}
