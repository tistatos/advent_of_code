extern crate advent_of_code;
use self::advent_of_code::get_csv;

#[derive(PartialEq, Debug)]
enum Op {
    Add = 1,
    Mul = 2,
    Halt = 99,
}

impl Op {
    fn exec(self, lhs: i32, rhs: i32) -> i32 {
        match self {
            Op::Add => lhs + rhs,
            Op::Mul => lhs * rhs,
            _ => panic!("unrecognized op")
        }
    }
}

impl std::convert::From<i32> for Op {
    fn from(val: i32) -> Op {
        match val {
            1 => Op::Add,
            2 => Op::Mul,
            99 => Op::Halt,
            err => panic!("unrecognized opcode: {}", err)
        }
    }
}

fn patch_input(input: &mut Vec<i32>, noun: i32, verb: i32) {
    input[1] = noun;
    input[2] = verb;
}

fn execute_program(mut input: Vec<i32>) -> i32 {
    for i in (0..input.len()).step_by(4) {
        let op = Op::from(input[i]);
        if op == Op::Halt {
            break;
        }

        let lhs = input[input[i+1] as usize];
        let rhs = input[input[i+2] as usize];
        let dest = input[i+3];

        input[dest as usize] = op.exec(lhs,rhs);
    }

    input[0]
}

fn main() {
    let original_input = get_csv("input_data/day_2", ",");
    {
        let mut input = original_input.clone();
        patch_input(&mut input, 12, 2);
        let result = execute_program(input);
        println!("part 1 answer: {}", result);
    }

    const DESIRED_RESULT: i32 = 19690720;

    for noun in 0..100 {
        for verb in 0..100 {
            let mut input = original_input.clone();
            patch_input(&mut input, noun, verb);
            let result = execute_program(input);
            if result == DESIRED_RESULT {
                println!("part 2 answer: {}", 100 * noun + verb);
                break;
            }
        }
    }

}
