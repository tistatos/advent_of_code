mod aoc;
use self::aoc::get_csv;
use std::collections::HashSet;

enum Mode {
    Position,
    Immediate
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
    Add,
    Mul,
    Input,
    Output,
    JNZ,
    JZ,
    LE,
    EQ,
    Halt,
}
impl std::convert::From<i32> for Op {
    fn from(val: i32) -> Op {
        match val {
            1 => Op::Add, //3 parameters
            2 => Op::Mul, //3 parameters
            3 => Op::Input, //1 parameter
            4 => Op::Output, //1 parameter
            5 => Op::JNZ, //2 parameter
            6 => Op::JZ, //2 parameter
            7 => Op::LE, //3 parameter
            8 => Op::EQ, //3 parameter
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
        Op::Add | Op::Mul | Op::LE | Op::EQ
            | Op::JNZ | Op::JZ => {
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

fn execute_program(mut program: Vec<i32>, mut input: Vec<i32>) -> i32 {
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
                let inp = input.pop().unwrap();
                program[dest as usize] = inp;
                i += 2;
            },
            Op::Output => {
                let src = match lhs_mode.unwrap() {
                    Mode::Position => program[program[i+1] as usize],
                    Mode::Immediate => program[i+1]
                };
                output = src;
                i += 2;
            },
            Op::JNZ => {
                let operand = match lhs_mode.unwrap() {
                    Mode::Position => program[program[i+1] as usize],
                    Mode::Immediate => program[i+1]
                };
                let dest = match rhs_mode.unwrap() {
                    Mode::Position => program[program[i+2] as usize],
                    Mode::Immediate => program[i+2]
                };
                if operand != 0 {
                    i = dest as usize;
                }
                else {
                    i += 3;
                }
            },
            Op::JZ => {
                let operand = match lhs_mode.unwrap() {
                    Mode::Position => program[program[i+1] as usize],
                    Mode::Immediate => program[i+1]
                };
                let dest = match rhs_mode.unwrap() {
                    Mode::Position => program[program[i+2] as usize],
                    Mode::Immediate => program[i+2]
                };
                if operand == 0 {
                    i = dest as usize;
                }
                else {
                    i += 3;
                }
            },
            Op::LE => {
                let lhs = match lhs_mode.unwrap() {
                    Mode::Position => program[program[i+1] as usize],
                    Mode::Immediate => program[i+1]
                };
                let rhs = match rhs_mode.unwrap() {
                    Mode::Position => program[program[i+2] as usize],
                    Mode::Immediate => program[i+2]
                };
                let dest = program[i+3]; //always position?
                program[dest as usize] = (lhs < rhs) as i32;
                i += 4;
            },
            Op::EQ => {
                let lhs = match lhs_mode.unwrap() {
                    Mode::Position => program[program[i+1] as usize],
                    Mode::Immediate => program[i+1]
                };
                let rhs = match rhs_mode.unwrap() {
                    Mode::Position => program[program[i+2] as usize],
                    Mode::Immediate => program[i+2]
                };
                let dest = program[i+3]; //always position?
                program[dest as usize] = (lhs == rhs) as i32;
                i += 4;
            },
            _ => panic!("Unexpected instruction")
        }
    }

    output
}

fn main() {
    let mut largest_result = 0;
    //let program = vec!(3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0);
    let program = get_csv("input_data/day_7", ",");
    for a in 0..5 {
        for b in 0..5 {
            for c in 0..5 {
                for d in 0..5 {
                    for e in 0..5 {
                        let mut all_unique = HashSet::new();
                        all_unique.insert(a);
                        all_unique.insert(b);
                        all_unique.insert(c);
                        all_unique.insert(d);
                        all_unique.insert(e);
                        if all_unique.len() != 5 {
                            continue;
                        }

                        let result = execute_program(program.clone(), vec!(0, a)); //A
                        let result = execute_program(program.clone(), vec!(result, b)); //B
                        let result = execute_program(program.clone(), vec!(result, c)); //C
                        let result = execute_program(program.clone(), vec!(result, d)); //D
                        let result = execute_program(program.clone(), vec!(result, e)); //E
                        if result > largest_result {
                            largest_result = result;
                        }
                    }
                }
            }
        }
    }

    println!("day 7 part 1: {}", largest_result );
}
