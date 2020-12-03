extern crate advent_of_code;
use advent_of_code::get_string_rows;

static TREE: char = '#';

pub fn main() {
    let input = get_string_rows("input_data/day_3");
    let patterns = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let tree_count: Vec<u64> = patterns
        .iter()
        .map(|p| {
            input
                .iter()
                .step_by(p.1)
                .enumerate()
                .map(|(i, r)| {
                    let index = (i * p.0) % r.len();
                    (r.chars().nth(index).unwrap() == TREE) as u64
                })
                .sum()
        })
        .collect();
    println!("day 3 part 1: {}", tree_count[1]);
    println!(
        "day 3 part 2: {}",
        tree_count.iter().fold(1, |x, acc| acc * x)
    );
}
