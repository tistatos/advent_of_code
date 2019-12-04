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
    for number in start..stop {
        let mut previous_digit = get_digit(number, 1);
        let mut found_adjacent = false;
        let mut valid = true;
        for i in 2..7 {
            let current_digit = get_digit(number, i);
            if current_digit > previous_digit {
                valid = false;
                break;
            }
            else if current_digit == previous_digit {
                found_adjacent = true;
            }
            previous_digit = current_digit
        }

        if valid && found_adjacent {
            valid_codes_part_1 += 1;
        }
    }
    println!("number {} valid codes part 1", valid_codes_part_1);
}
