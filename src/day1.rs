mod aoc;

use self::aoc::get_rows;


fn main() {
    let input = get_rows("input_data/day_1");
    let fuel_function = |x: i32| x / 3 - 2;

    let sum: i32 = input.iter().map(|x| fuel_function(*x)).sum();

    println!("part 1 sum is {}", sum);


    let sum = input.iter().fold(0, |mut acc, x| {
        let mut fuel = *x;
        while fuel > 0 {
            fuel = fuel_function(fuel);
            if fuel > 0 {
                acc += fuel;
            }
        }
        acc
    });
    println!("part 2 sum is {}", sum);
}
