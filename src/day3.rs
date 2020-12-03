mod aoc;
use crate::aoc::get_string_rows;

static TREE: char = '#';

pub fn main() {
    let input = get_string_rows("input_data/day_3");
    let pattern = 3;
    let steps = input.len();
    let width = input[0].len();

    let mut tree_count = 0;
    for i in 1..steps {
        let ref row = input[i];
        let index = (i * pattern) % width;
        let spot = row.chars().nth(index).unwrap();
        if spot == TREE {
            tree_count += 1;
        }
    }

    println!("day 3 part 1: {}", tree_count);
}
