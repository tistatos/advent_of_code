use advent_of_code::get_row_input;
use advent_of_code::get_string_rows;

const ROCK: i8 = 'A' as i8;
const DRAW: i8 = 1;
const WIN: i8 = 2;

pub fn main() {
    let first_strategy = |opp: i8, me: i8| {
        let mut score = me + 1;
        let diff = me - opp;
        if diff == 0 {
            score += 3;
        } else if diff < 0 {
            score += diff.abs() / 2 * 6;
        } else {
            score += diff % 2 * 6;
        }
        score as u32
    };

    let second_strategy = |opp: i8, res: i8| {
        if opp - res == 0 && res != DRAW {
            if res == WIN {
                7
            } else {
                3
            }
        } else {
            (4 * res + opp) as u32
        }
    };

    let games: Vec<(i8, i8)> = get_row_input!("2")
        .iter()
        .map(|p| {
            let mut i = p.chars();
            let opp = i.next().unwrap() as i8 - ROCK;
            let strategy = i.skip(1).next().unwrap() as i8 - (ROCK + 23);
            (opp, strategy)
        })
        .collect();

    println!(
        "Solution 1: {:?}",
        games
            .iter()
            .map(|(a, b)| first_strategy(*a, *b))
            .sum::<u32>()
    );

    println!(
        "Solution 2: {:?}",
        games
            .iter()
            .map(|(a, b)| second_strategy(*a, *b))
            .sum::<u32>()
    );
}
