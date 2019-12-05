mod aoc;
use self::aoc::get_csv;

enum Mode {
    Position = 0,
    Immediate = 1
}
impl std::convert::From<i32> for Mode {
    fn from(val: i32) -> Mode {
        match val {
            0 => Mode::Position,
            1 => Mode::Immediate,
            err => panic!("unrecognized Mode: {}", err)
        }
    }
}

fn get_digit(number: i32, position: u32) -> i32 {
    number / 10i32.pow(position - 1) % 10
}

#[derive(PartialEq, Debug)]
enum Op {
    Add = 1,
    Mul = 2,
    Input = 3,
    Output = 4,
    Halt = 99,
}
impl std::convert::From<i32> for Op {
    fn from(val: i32) -> Op {
        match val {
            1 => Op::Add, //3 parameters
            2 => Op::Mul, //3 parameters
            3 => Op::Input, //1 parameter
            4 => Op::Output, //1 parameter
            99 => Op::Halt, //0 parameters
            err => panic!("unrecognized opcode: {}", err)
        }
    }
}

fn parse_op(code: i32) -> (Op, Option<Mode>, Option<Mode>) {
    let op =
        Op::from(
            get_digit(code, 2) * 10 + get_digit(code, 1));
    let (lhs, rhs) = match op {
        Op::Add | Op::Mul => {
            (Some(Mode::from(get_digit(code,3))),
            Some(Mode::from(get_digit(code,4))))
        }
        Op::Output => {
            (Some(Mode::from(get_digit(code,3))), None)
        }
        Op::Input | Op::Halt => {
            (None, None)
        }
    };
    (op, lhs, rhs)
}

fn execute_program(mut program: Vec<i32>, input: i32) -> Vec<i32> {
    let mut i = 0;
    let mut output = 0;
    loop {
        let (op, lhs_mode, rhs_mode) = parse_op(program[i]);

        if op == Op::Halt {
            break;
        }

        match op {
            Op::Add => {
                let lhs = match lhs_mode.unwrap() {
                    Mode::Position => program[program[i+1] as usize],
                    Mode::Immediate => program[i+1]
                };
                let rhs = match rhs_mode.unwrap() {
                    Mode::Position => program[program[i+2] as usize],
                    Mode::Immediate => program[i+2]
                };

                let dest = program[i+3]; //always position?
                program[dest as usize] = lhs + rhs;
                i += 4;
            },
            Op::Mul => {
                let lhs = match lhs_mode.unwrap() {
                    Mode::Position => program[program[i+1] as usize],
                    Mode::Immediate => program[i+1]
                };
                let rhs = match rhs_mode.unwrap() {
                    Mode::Position => program[program[i+2] as usize],
                    Mode::Immediate => program[i+2]
                };

                let dest = program[i+3]; //always position?
                program[dest as usize] = lhs * rhs;
                i += 4;
            },
            Op::Input => {
                let dest = program[i+1]; //always position?
                program[dest as usize] = input;
                i += 2;
            },
            Op::Output => {
                let src = program[i+1]; //always position?
                output = program[src as usize];
                i += 2;
            },
            _ => panic!("Unexpected instruction")
        }
    }

    println!("output is {}", output);
    program
}

fn main() {
    let program = get_csv("input_data/day_5", ",");
    {
        let program = program.clone();
        let result = execute_program(program, 1);
    }
}
