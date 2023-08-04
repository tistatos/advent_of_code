use std::collections::HashSet;
use std::iter::FromIterator;

use advent_of_code::get_row_input;
use advent_of_code::get_string_rows;

pub fn main() {
    let to_prio = |a: &char| {
        let k = *a as u8 as usize;
        if k > 96 {
            k - 96
        } else {
            k - 38
        }
    };

    let backpacks: Vec<(HashSet<char>, HashSet<char>)> = get_row_input!("3")
        .iter()
        .map(|bp| {
            let l = bp.len() / 2;
            (
                HashSet::from_iter(bp[..l].chars()),
                HashSet::from_iter(bp[l..].chars()),
            )
        })
        .collect();

    let prio_sum: usize = backpacks
        .iter()
        .map(|(a, b)| to_prio(a.intersection(&b).next().unwrap()))
        .sum();

    let group_sum: usize = backpacks
        .iter()
        .map(|(a, b)| a.union(&b).collect())
        .collect::<Vec<HashSet<_>>>()
        .chunks(3)
        .map(|s| {
            to_prio(
                s[0].intersection(&HashSet::from_iter(s[1].intersection(&s[2]).cloned()))
                    .next()
                    .unwrap(),
            )
        })
        .sum();

    println!("Solution 1:{}", prio_sum);
    println!("Solution 2:{}", group_sum);
}
