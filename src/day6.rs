mod aoc;
use std::collections::{HashMap};
use std::collections::{VecDeque};

use self::aoc::get_string_rows;

pub fn count_orbits(input: Vec<String>) -> i32 {
    let mut jumps: HashMap<String, i32> = HashMap::new();

    jumps.insert("COM".to_string(), 0);

    let mut queue = VecDeque::from(input);
    while let Some(orb) = queue.pop_front() {
        let orb_split: Vec<String> = orb.split(")").map(|s| s.to_string()).collect();

        if let Some(body) = jumps.get(&orb_split[0]) {
            let origin_value = *body;
            let satellite = jumps.entry(orb_split[1].clone()).or_insert(origin_value);
                *satellite +=1;
        }
        else {
            queue.push_back(orb);
        }
    }

    jumps.iter_mut().map(|(_v, k)| *k).sum()
}

pub fn main() {
    //let input: Vec<String> = vec!( "B)C", "C)D", "COM)B", "D)E", "B)G", "G)H", "D)I", "E)F", "E)J", "J)K", "K)L")
        //.iter().map(|s| s.to_string()).collect();
    let input = get_string_rows("input_data/day_6");
    let orbit_count = count_orbits(input);
    println!("part 1: {}", orbit_count);

}
