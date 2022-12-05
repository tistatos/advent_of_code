use advent_of_code::get_row_input;
use advent_of_code::get_string_rows;

pub fn main() {
    //let sections: Vec<String> = get_row_input!("4_test");
    let sections: Vec<String> = get_row_input!("4");
    let sections: Vec<((u32, u32), (u32, u32))> = sections
        .iter()
        .map(|s| {
            let mut sec = s.split(',');
            let mut first = sec.next().unwrap().split('-');
            let first_start = first.next().unwrap().parse::<u32>().unwrap();
            let first_end = first.next().unwrap().parse::<u32>().unwrap();
            let mut second = sec.next().unwrap().split('-');
            let second_start = second.next().unwrap().parse::<u32>().unwrap();
            let second_end = second.next().unwrap().parse::<u32>().unwrap();
            ((first_start, first_end), (second_start, second_end))
        })
        .collect();
    let overlapping_pairs: usize = sections
        .iter()
        .filter(|(s0, s1)| s1.0 >= s0.0 && s1.1 <= s0.1 || s0.0 >= s1.0 && s0.1 <= s1.1)
        .count();

    let overlapping_sections: usize = sections
        .iter()
        .filter(|(s1, s2)| {
            println!("{:?} {:?}", s1, s2);
            if s1.0 <= s2.0 && s1.1 >= s2.0 || s2.0 <= s1.0 && s2.1 >= s1.0 {
                println!("true");
                true
            } else {
                false
            }
        })
        .count();

    println!("Solution 1: {:?}", overlapping_pairs);
    println!("Solution 2: {:?}", overlapping_sections);
}
