mod aoc;
use crate::aoc::get_string_rows;

static TREE: char = '#';

pub fn main() {
    let input = get_string_rows("input_data/day_3");
    let patterns = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let steps = input.len();
    let width = input[0].len();

    let mut tree_count = vec![];
    for p in patterns {
        let mut p_count: u64 = 0;
        let mut loop_c = 0;
        for i in (0..steps).step_by(p.1) {
            let ref row = input[i];
            let index = (loop_c * p.0) % width;
            loop_c += 1;
            let spot = row.chars().nth(index).unwrap();
            if spot == TREE {
                p_count += 1;
            }
        }
        println!("p: {:?} {}", p, p_count);
        tree_count.push(p_count)
    }
    println!("day 3 part 1: {}", tree_count[1]);
    println!(
        "day 3 part 2: {}",
        tree_count.iter().fold(1, |x, acc| acc * x)
    );
}
