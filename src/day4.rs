mod aoc;
use std::env;

fn get_digit(number: u32, position: u32) -> u32 {
    number / 10u32.pow(position - 1) % 10
}

pub fn main() {

    let mut args = env::args().skip(1);
    let start: u32 = args.next().unwrap().parse::<u32>().unwrap();
    let stop: u32 = args.next().unwrap().parse::<u32>().unwrap();

    let mut valid_codes_part_1 = 0;
    let mut valid_codes_part_2 = 0;

    for number in start..stop {

        let mut found_adjacent = false;
        let mut found_pair = false;
        let mut adjacent_count = 1;
        let mut valid = true;

        let mut previous_digit = get_digit(number, 1);
        for i in 2..7 {
            let current_digit = get_digit(number, i);
            if current_digit > previous_digit {
                valid = false;
                break;
            }
            else if current_digit == previous_digit {
                found_adjacent = true;
                adjacent_count += 1;
            }
            else {
                if adjacent_count == 2 {
                    found_pair = true;
                }
                adjacent_count = 1;
            }
            previous_digit = current_digit
        }
        if adjacent_count == 2 {
            found_pair = true;
        }

        if valid && found_adjacent {
            valid_codes_part_1 += 1;
        }

        if valid && found_pair {
            valid_codes_part_2 += 1;
        }
    }
    println!("number {} valid codes part 1", valid_codes_part_1);
    println!("number {} valid codes part 2", valid_codes_part_2);
}
