mod aoc;

use self::aoc::get_rows;

fn main() {
    let input: Vec<u32> = get_rows("input_data/day_1");

    for i in &input {
        if *i > 2020 {
            continue;
        }
        for j in &input {
            if *i + *j == 2020 {
                println!("Solution day 1: {}", *i * *j);
                break;
            }
        }
    }
}
