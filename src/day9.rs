mod aoc;

use self::aoc::intcode::{IntCodeMachine};
use self::aoc::{get_csv};

fn main() {
    //let input = vec!(109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99);
    let input = get_csv("input_data/day_9", ",");

    let mut machine = IntCodeMachine::new(input);
    while !machine.will_halt() {
        if let Some(result) = machine.execute_program(Some(vec!(2))) {
            print!("{} ", result);
        }
    }
    println!("");
}
