use std::collections::HashMap;
use std::collections::VecDeque;

pub fn main() {
    let desc = include_str!("../input_data/day_5");
    let (crates, instructions) = desc.split_at(desc.find("\n\n").unwrap());
    let instructions = instructions
        .split('\n')
        .filter_map(|l| {
            if l == "" {
                None
            } else {
                let l_s: Vec<&str> = l.split(" ").collect();
                let cnt = l_s.get(1).unwrap().parse::<usize>().unwrap();
                let from = l_s.get(3).unwrap().parse::<usize>().unwrap();
                let to = l_s.get(5).unwrap().parse::<usize>().unwrap();
                Some((from, to, cnt))
            }
        })
        .collect::<Vec<(usize, usize, usize)>>();
    let container_stacks = crates
        .split('\n')
        .flat_map(|r| {
            r.chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .enumerate()
                .filter_map(|(i, c)| {
                    if c.iter().collect::<String>().trim().is_empty() {
                        None
                    } else {
                        Some((i + 1, c[1]))
                    }
                })
                .collect::<Vec<(usize, char)>>()
        })
        .fold(HashMap::<usize, VecDeque<char>>::new(), |mut acc, c| {
            if !c.1.is_ascii_digit() {
                acc.entry(c.0).or_default().push_back(c.1);
            }
            acc
        });

    let mut first_stack = container_stacks.clone();
    let mut second_stack = container_stacks.clone();

    for (f, t, c) in instructions.iter() {
        let mut crane_arm = VecDeque::new();
        for _ in 0..*c {
            let container = first_stack.entry(*f).or_default().pop_front().unwrap();
            first_stack.entry(*t).or_default().push_front(container);

            let container = second_stack.entry(*f).or_default().pop_front().unwrap();
            crane_arm.push_back(container);
        }
        while !crane_arm.is_empty() {
            second_stack
                .entry(*t)
                .or_default()
                .push_front(crane_arm.pop_back().unwrap());
        }
    }

    let stack_count = container_stacks.len();
    let mut first_res = String::new();
    let mut second_res = String::new();
    for i in 0..stack_count {
        let top_c = first_stack.get(&(i + 1)).unwrap().get(0).unwrap();
        first_res.push(*top_c);
        let top_c = second_stack.get(&(i + 1)).unwrap().get(0).unwrap();
        second_res.push(*top_c);
    }
    println!("Solution 1: {}", first_res);
    println!("Solution 2: {}", second_res);
}
