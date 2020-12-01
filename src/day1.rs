mod aoc;

use self::aoc::get_rows;

fn main() {
    let input: Vec<u32> = get_rows("input_data/day_1");

    let mut part_one = 0;
    let mut part_two = 0;

    for i in 0..input.len() {
        let a = input[i];
        if a > 2020 {
            continue;
        }
        for j in (i + 1)..input.len() {
            let b = input[j];
            if a + b == 2020 {
                part_one = a * b;
            }
            for k in (j + 1)..input.len() {
                if a + b > 2020 {
                    continue;
                }
                let c = input[k];
                if a + b + c == 2020 {
                    part_two = a * b * c;
                }
            }
        }
    }

    println!("Solution day 1 part 1: {}", part_one);
    println!("Solution day 1 part 2: {}", part_two);
}
