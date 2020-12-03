extern crate advent_of_code;
use self::advent_of_code::get_csv;
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

struct Amplifier {
    ip: usize,
    program: Vec<i32>,
    halted: bool
}

impl Amplifier {
    fn execute_program(&mut self, mut input: Vec<i32>) -> i32 {
        let mut output = 0;
        loop {
            let (op, lhs_mode, rhs_mode) = parse_op(self.program[self.ip]);

            if op == Op::Halt {
                self.halted = true;
                break;
            }

            match op {
                Op::Add => {
                    let lhs = match lhs_mode.unwrap() {
                        Mode::Position => self.program[self.program[self.ip+1] as usize],
                        Mode::Immediate => self.program[self.ip+1]
                    };
                    let rhs = match rhs_mode.unwrap() {
                        Mode::Position => self.program[self.program[self.ip+2] as usize],
                        Mode::Immediate => self.program[self.ip+2]
                    };

                    let dest = self.program[self.ip+3]; //always position?
                    self.program[dest as usize] = lhs + rhs;
                    self.ip += 4;
                },
                Op::Mul => {
                    let lhs = match lhs_mode.unwrap() {
                        Mode::Position => self.program[self.program[self.ip+1] as usize],
                        Mode::Immediate => self.program[self.ip+1]
                    };
                    let rhs = match rhs_mode.unwrap() {
                        Mode::Position => self.program[self.program[self.ip+2] as usize],
                        Mode::Immediate => self.program[self.ip+2]
                    };

                    let dest = self.program[self.ip+3]; //always position?
                    self.program[dest as usize] = lhs * rhs;
                    self.ip += 4;
                },
                Op::Input => {
                    let dest = self.program[self.ip+1]; //always position?
                    let inp = input.pop().unwrap();
                    self.program[dest as usize] = inp;
                    self.ip += 2;
                },
                Op::Output => {
                    let src = match lhs_mode.unwrap() {
                        Mode::Position => self.program[self.program[self.ip+1] as usize],
                        Mode::Immediate => self.program[self.ip+1]
                    };
                    output = src;
                    self.ip += 2;
                    break;
                },
                Op::JNZ => {
                    let operand = match lhs_mode.unwrap() {
                        Mode::Position => self.program[self.program[self.ip+1] as usize],
                        Mode::Immediate => self.program[self.ip+1]
                    };
                    let dest = match rhs_mode.unwrap() {
                        Mode::Position => self.program[self.program[self.ip+2] as usize],
                        Mode::Immediate => self.program[self.ip+2]
                    };
                    if operand != 0 {
                        self.ip = dest as usize;
                    }
                    else {
                        self.ip += 3;
                    }
                },
                Op::JZ => {
                    let operand = match lhs_mode.unwrap() {
                        Mode::Position => self.program[self.program[self.ip+1] as usize],
                        Mode::Immediate => self.program[self.ip+1]
                    };
                    let dest = match rhs_mode.unwrap() {
                        Mode::Position => self.program[self.program[self.ip+2] as usize],
                        Mode::Immediate => self.program[self.ip+2]
                    };
                    if operand == 0 {
                        self.ip = dest as usize;
                    }
                    else {
                        self.ip += 3;
                    }
                },
                Op::LE => {
                    let lhs = match lhs_mode.unwrap() {
                        Mode::Position => self.program[self.program[self.ip+1] as usize],
                        Mode::Immediate => self.program[self.ip+1]
                    };
                    let rhs = match rhs_mode.unwrap() {
                        Mode::Position => self.program[self.program[self.ip+2] as usize],
                        Mode::Immediate => self.program[self.ip+2]
                    };
                    let dest = self.program[self.ip+3]; //always position?
                    self.program[dest as usize] = (lhs < rhs) as i32;
                    self.ip += 4;
                },
                Op::EQ => {
                    let lhs = match lhs_mode.unwrap() {
                        Mode::Position => self.program[self.program[self.ip+1] as usize],
                        Mode::Immediate => self.program[self.ip+1]
                    };
                    let rhs = match rhs_mode.unwrap() {
                        Mode::Position => self.program[self.program[self.ip+2] as usize],
                        Mode::Immediate => self.program[self.ip+2]
                    };
                    let dest = self.program[self.ip+3]; //always position?
                    self.program[dest as usize] = (lhs == rhs) as i32;
                    self.ip += 4;
                },
                _ => panic!("Unexpected instruction")
            }
        }
        output
    }
    fn will_halt(&self) -> bool {
        let (op, _, _) = parse_op(self.program[self.ip]);
        op == Op::Halt
    }
}

fn main() {
    let mut largest_result = 0;
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

                        let mut a_amp = Amplifier{ ip: 0, program: program.clone(), halted: false};
                        let mut b_amp = Amplifier{ ip: 0, program: program.clone(), halted: false};
                        let mut c_amp = Amplifier{ ip: 0, program: program.clone(), halted: false};
                        let mut d_amp = Amplifier{ ip: 0, program: program.clone(), halted: false};
                        let mut e_amp = Amplifier{ ip: 0, program: program.clone(), halted: false};

                        let result = a_amp.execute_program(vec!(0, a)); //A
                        let result = b_amp.execute_program(vec!(result, b)); //B
                        let result = c_amp.execute_program(vec!(result, c)); //C
                        let result = d_amp.execute_program(vec!(result, d)); //D
                        let result = e_amp.execute_program(vec!(result, e)); //E
                        if result > largest_result {
                            largest_result = result;
                        }
                    }
                }
            }
        }
    }
    println!("day 7 part 1: {}", largest_result );

    for a in 5..10 {
        for b in 5..10 {
            for c in 5..10 {
                for d in 5..10 {
                    for e in 5..10 {
                        let mut all_unique = HashSet::new();
                        all_unique.insert(a);
                        all_unique.insert(b);
                        all_unique.insert(c);
                        all_unique.insert(d);
                        all_unique.insert(e);
                        if all_unique.len() != 5 {
                            continue;
                        }

                        let mut a_amp = Amplifier{ ip: 0, program: program.clone(), halted: false};
                        let mut b_amp = Amplifier{ ip: 0, program: program.clone(), halted: false};
                        let mut c_amp = Amplifier{ ip: 0, program: program.clone(), halted: false};
                        let mut d_amp = Amplifier{ ip: 0, program: program.clone(), halted: false};
                        let mut e_amp = Amplifier{ ip: 0, program: program.clone(), halted: false};
                        let mut result = a_amp.execute_program(vec!(0, a)); //A
                        result = b_amp.execute_program(vec!(result, b)); //B
                        result = c_amp.execute_program(vec!(result, c)); //C
                        result = d_amp.execute_program(vec!(result, d)); //D
                        result = e_amp.execute_program(vec!(result, e)); //E
                        while !a_amp.will_halt() {
                            result = a_amp.execute_program(vec!(result)); //A
                            result = b_amp.execute_program(vec!(result)); //B
                            result = c_amp.execute_program(vec!(result)); //C
                            result = d_amp.execute_program(vec!(result)); //D
                            result = e_amp.execute_program(vec!(result)); //E
                        }

                        if result > largest_result {
                            largest_result = result;
                        }
                    }
                }
            }
        }
    }

    println!("day 7 part 2: {}", largest_result );
}
