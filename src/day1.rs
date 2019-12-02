mod aoc;

use self::aoc::get_rows;


fn main() {
    let input = get_rows("input_data/day_1");

    let sum: i32 = input.iter().map(|x| x / 3 - 2).sum();

    println!("sum is {}", sum);
}
