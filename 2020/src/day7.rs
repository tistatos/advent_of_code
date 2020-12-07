#[macro_use]
extern crate advent_of_code;
use std::collections::HashMap;

use advent_of_code::get_string_rows;

fn contains_shiny_gold(bags: &HashMap<String, Vec<String>>, bag: &String) -> bool {
    let content = bags.get(bag).unwrap();
    for b in content {
        if b == "shiny gold" || contains_shiny_gold(bags, b) {
            return true;
        }
    }
    false
}

pub fn main() {
    let input = get_row_input!("7");

    let mut bags: HashMap<String, Vec<String>> = HashMap::new();

    for r in input {
        let splitted: Vec<&str> = r.split("bag").collect();
        let first_bag = splitted[0].trim().to_string();
        bags.insert(first_bag.clone(), vec![]);

        for s in splitted.iter().skip(1) {
            match s.find(char::is_numeric) {
                Some(i) => {
                    let name = format!("{}", &s[i + 2..].trim());
                    match bags.get(&name) {
                        Some(_) => {}
                        None => {
                            bags.insert(name.clone(), vec![]);
                        }
                    };

                    match bags.get_mut(&first_bag) {
                        Some(b) => b.push(name.clone()),
                        _ => panic!("couldn't find {}!", first_bag),
                    };
                }
                _ => {}
            };
        }
    }

    let result = bags
        .iter()
        .filter_map(|b| {
            let res = contains_shiny_gold(&bags, &b.0);
            if res {
                Some(b)
            } else {
                None
            }
        })
        .count();
    println!("day 7 part 1: {}", result);
}
