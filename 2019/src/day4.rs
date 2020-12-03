extern crate advent_of_code;
use std::env;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

fn get_digit(number: u32, position: u32) -> u32 {
    number / 10u32.pow(position - 1) % 10
}

pub fn find_codes(start: u32, stop: u32, counter_part_1: Arc<AtomicUsize>, counter_part_2: Arc<AtomicUsize>) {
    for number in start..stop {
        let mut found_adjacent = false;
        let mut found_pair = false;
        let mut adjacent_count = 1;
        let mut valid = true;

        let mut previous_digit = get_digit(number, 1);
        for i in 2..7 {
            let current_digit = get_digit(number, i);
            if current_digit > previous_digit {
                valid = false;
                break;
            }
            else if current_digit == previous_digit {
                found_adjacent = true;
                adjacent_count += 1;
            }
            else {
                if adjacent_count == 2 {
                    found_pair = true;
                }
                adjacent_count = 1;
            }
            previous_digit = current_digit
        }
        if adjacent_count == 2 {
            found_pair = true;
        }

        if valid && found_adjacent {
            counter_part_1.fetch_add(1, Ordering::SeqCst);
        }

        if valid && found_pair {
            counter_part_2.fetch_add(1, Ordering::SeqCst);
        }
    }
}


pub fn main() {

    let mut args = env::args().skip(1);
    let start: u32 = args.next().unwrap().parse::<u32>().unwrap();
    let stop: u32 = args.next().unwrap().parse::<u32>().unwrap();

    let range = stop - start;
    const THREADS: u32 = 4;
    let thread_range = range / THREADS;

    let mut valid_codes_part_1 = Arc::new(AtomicUsize::new(0));
    let mut valid_codes_part_2 = Arc::new(AtomicUsize::new(0));

    let mut handles = vec![];
    for i in 0..THREADS {
        let p1 = valid_codes_part_1.clone();
        let p2 = valid_codes_part_2.clone();

        handles.push(
            thread::spawn(move||
                find_codes(start + i * thread_range, start + (i + 1) * thread_range, p1, p2))
        );
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("number {} valid codes part 1", valid_codes_part_1.load(Ordering::SeqCst));
    println!("number {} valid codes part 2", valid_codes_part_2.load(Ordering::SeqCst));
}
