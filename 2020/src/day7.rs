#[macro_use]
extern crate advent_of_code;
use std::collections::HashMap;

use advent_of_code::get_string_rows;

fn contains_shiny_gold(bags: &HashMap<String, Vec<(usize, String)>>, bag: &String) -> bool {
    let content = bags.get(bag).unwrap();
    content
        .iter()
        .any(|b| b.1 == "shiny gold" || contains_shiny_gold(bags, &b.1))
}

fn bag_count(bags: &HashMap<String, Vec<(usize, String)>>, bag: &String) -> usize {
    let content = bags.get(bag).unwrap();
    content
        .iter()
        .map(|b| b.0 + b.0 * bag_count(&bags, &b.1))
        .sum()
}

pub fn main() {
    let input = get_row_input!("7");

    let mut bags: HashMap<String, Vec<(usize, String)>> = HashMap::new();

    for r in input {
        let splitted: Vec<&str> = r.split("bag").collect();
        let first_bag = splitted[0].trim().to_string();

        bags.insert(first_bag.clone(), vec![]);

        for s in splitted.iter().skip(1) {
            match s.find(char::is_numeric) {
                Some(i) => {
                    let count = &s[i..i + 1].parse::<usize>().unwrap();
                    let name = format!("{}", &s[i + 2..].trim());
                    match bags.get(&name) {
                        Some(_) => {}
                        None => {
                            bags.insert(name.clone(), vec![]);
                        }
                    };

                    match bags.get_mut(&first_bag) {
                        Some(b) => b.push((*count, name)),
                        _ => panic!("couldn't find {}!", first_bag),
                    };
                }
                _ => {}
            };
        }
    }

    let result = bags
        .iter()
        .filter(|b| contains_shiny_gold(&bags, &b.0))
        .count();
    println!("day 7 part 1: {}", result);
    println!(
        "day 7 part 2: {}",
        bag_count(&bags, &"shiny gold".to_string())
    );
}
