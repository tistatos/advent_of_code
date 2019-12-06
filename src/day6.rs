mod aoc;
use std::collections::{HashMap, HashSet, VecDeque};

use self::aoc::get_string_rows;

fn count_orbits(mut input: Vec<String>) -> i32 {
    let mut jumps: HashMap<String, i32> = HashMap::new();

    jumps.insert("COM".to_string(), 0);
    while let Some(orb) = input.pop() {
        let orb_split: Vec<String> = orb.split(")").map(|s| s.to_string()).collect();

        if let Some(body) = jumps.get(&orb_split[0]) {
            let origin_value = *body;
            let satellite = jumps.entry(orb_split[1].clone()).or_insert(origin_value);
                *satellite +=1;
        }
        else {
            input.push(orb);
        }
    }

    jumps.iter_mut().map(|(_v, k)| *k).sum()
}

fn find_parent_body(input: &Vec<String>, child: &String) -> Option<String> {
    for b in input {
        let orb_split: Vec<String> = b.split(")").map(|s| s.to_string()).collect();
        if orb_split[1] == *child {
            return Some(orb_split[0].clone());
        }
    }
    None
}

fn find_child_bodies(input: &Vec<String>, parent: &String) -> Vec<String> {
    let mut children = vec!();
    for b in input {
        let orb_split: Vec<String> = b.split(")").map(|s| s.to_string()).collect();
        if orb_split[0] == *parent {
            children.push(orb_split[1].clone());
        }
    }
    children
}

fn transfer_orbits(input: &Vec<String>) -> i32 {
    let you_current_body = find_parent_body(input, &"YOU".to_string());

    let mut frontier = Vec::new();
    frontier.push((you_current_body.unwrap(), 0));

    let mut visited = HashSet::new();

    loop {
        if let Some(current_body) = frontier.pop() {

            visited.insert(current_body.0.clone());
            let children = find_child_bodies(input, &current_body.0);

            for c in children {
                if c == "YOU" {
                    continue;
                }
                if c != "SAN" {
                    if visited.contains(&c) {
                        continue;
                    }
                    frontier.push((c, current_body.1 + 1));
                }
                else {
                    return current_body.1;
                }
            }
            if let Some(parent) = find_parent_body(input, &current_body.0) {
                if visited.contains(&parent) {
                    continue;
                }
                frontier.push((parent, current_body.1 + 1));
            }
            frontier.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        }
        else {
            panic!("unexpected end of frontier!");
        }
    }
}

fn main() {
    //let input: Vec<String> = vec!( "B)C", "C)D", "COM)B", "D)E", "B)G", "G)H", "D)I", "E)F", "E)J", "J)K", "K)L")
        //.iter().map(|s| s.to_string()).collect();

   //let input: Vec<String> = vec!(
    //"COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H",
    //"D)I", "E)J", "J)K", "K)L", "K)YOU", "I)SAN")
    //.iter().map(|s| s.to_string()).collect();

    let input = get_string_rows("input_data/day_6");
    let orbit_count = count_orbits(input.clone());
    println!("part 1: {}", orbit_count);

    let transfer_orbit_count = transfer_orbits(&input);
    println!("part 2: {}", transfer_orbit_count);

}
