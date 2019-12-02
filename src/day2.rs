mod aoc;
use self::aoc::get_csv;

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

fn patch_input(input: &mut Vec<i32>) {
    input[1] = 12;
    input[2] = 2;
}

fn main() {
    let mut input = get_csv("input_data/day_2", ",");
    patch_input(&mut input);

    for i in (0..input.len()).step_by(4) {
        let op = Op::from(input[i]);
        if op == Op::Halt {
            break;
        }

        let lhs = input[input[i+1] as usize];
        let rhs = input[input[i+2] as usize];
        let dest = input[i+3];
        let result = Op::from(op).exec(lhs,rhs);
        input[dest as usize] = result;
    }

    println!("{}", input[0]);
}
