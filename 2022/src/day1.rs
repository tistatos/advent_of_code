use advent_of_code::get_row_group;
use advent_of_code::get_row_groups;

pub fn main() {
    let mut calories: Vec<u32> = get_row_group!("1", u32)
        .iter()
        .map(|g| g.iter().sum::<u32>())
        .collect();
    calories.sort_by(|a, b| b.cmp(a));
    let most_cal = calories.first().unwrap();
    println!("Solution 1: {}", most_cal);
    let most_three_cal: u32 = calories.iter().take(3).sum();
    println!("Solution 2: {}", most_three_cal);
}
