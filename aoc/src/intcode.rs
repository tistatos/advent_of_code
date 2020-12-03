#[derive(PartialEq, Debug)]
enum Mode {
    Position,
    Immediate,
    Relative
}

impl std::convert::From<i64> for Mode {
    fn from(val: i64) -> Mode {
        match val {
            0 => Mode::Position,
            1 => Mode::Immediate,
            2 => Mode::Relative,
            err => panic!("unrecognized Mode: {}", err)
        }
    }
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
    ARB, //Adjust relative base
    Halt,
}
impl std::convert::From<i64> for Op {
    fn from(val: i64) -> Op {
        match val {
            1 => Op::Add, //3 parameters
            2 => Op::Mul, //3 parameters
            3 => Op::Input, //1 parameter
            4 => Op::Output, //1 parameter
            5 => Op::JNZ, //2 parameter
            6 => Op::JZ, //2 parameter
            7 => Op::LE, //3 parameter
            8 => Op::EQ, //3 parameter
            9 => Op::ARB, //1 parameter
            99 => Op::Halt, //0 parameters
            err => panic!("unrecognized opcode: {}", err)
        }
    }
}

fn get_digit(number: i64, position: u32) -> i64 {
    number / 10i64.pow(position - 1) % 10
}

pub struct IntCodeMachine {
    ip: usize,
    relative_base: usize,
    program: Vec<i64>,
    halted: bool
}

impl IntCodeMachine {
    pub fn new(mut program: Vec<i64>) -> Self {
        program.resize_with(2048, Default::default);
        Self {
            ip: 0,
            relative_base: 0,
            program,
            halted: false
        }
    }

    fn parse_op(code: i64) -> (Op, Option<Mode>, Option<Mode>, Option<Mode>) {
        let op =
            Op::from(
                get_digit(code, 2) * 10 + get_digit(code, 1));
        let (lhs, rhs, dst) = match op {
            Op::Add | Op::Mul | Op::LE | Op::EQ => {
                (Some(Mode::from(get_digit(code,3))),
                Some(Mode::from(get_digit(code,4))),
                Some(Mode::from(get_digit(code,5))),
                )
            }
            Op::JNZ | Op::JZ => {
                (Some(Mode::from(get_digit(code,3))),
                Some(Mode::from(get_digit(code,4))),
                None)
            }
            Op::Input | Op::Output | Op::ARB => {
                (Some(Mode::from(get_digit(code,3))), None, None)
            }
            Op::Halt => {
                (None, None, None)
            }
        };
        (op, lhs, rhs, dst)
    }

    pub fn execute_program(&mut self, mut input: Option<Vec<i64>>) -> Option<i64> {
        loop {
            let (op, lhs_mode, rhs_mode, dst_mode) = IntCodeMachine::parse_op(self.program[self.ip]);

            //println!("{}:\t {}={:?}({:?}, {:?}, {:?})", self.ip, self.program[self.ip],
                //op, lhs_mode, rhs_mode, dst_mode,);
            match op {
                Op::Add => {
                    let lhs = match lhs_mode.unwrap() {
                        Mode::Position => self.program[self.program[self.ip+1] as usize],
                        Mode::Immediate => self.program[self.ip+1],
                        Mode::Relative => self.program[(self.relative_base as i64 + self.program[self.ip + 1]) as usize]
                    };
                    let rhs = match rhs_mode.unwrap() {
                        Mode::Position => self.program[self.program[self.ip+2] as usize],
                        Mode::Immediate => self.program[self.ip+2],
                        Mode::Relative => self.program[(self.relative_base as i64 + self.program[self.ip + 2]) as usize]
                    };
                    let dest = match dst_mode.unwrap() {
                        Mode::Position => self.program[self.ip+3] as usize,
                        Mode::Immediate => self.ip+3,
                        Mode::Relative => (self.relative_base as i64 + self.program[self.ip + 3]) as usize
                    };

                    //println!("{}, {}, {}", lhs, rhs, dest);
                    self.program[dest as usize] = lhs + rhs;
                    self.ip += 4;
                },
                Op::Mul => {
                    let lhs = match lhs_mode.unwrap() {
                        Mode::Position => self.program[self.program[self.ip+1] as usize],
                        Mode::Immediate => self.program[self.ip+1],
                        Mode::Relative => self.program[(self.relative_base as i64 + self.program[self.ip + 1]) as usize]
                    };
                    let rhs = match rhs_mode.unwrap() {
                        Mode::Position => self.program[self.program[self.ip+2] as usize],
                        Mode::Immediate => self.program[self.ip+2],
                        Mode::Relative => self.program[(self.relative_base as i64 + self.program[self.ip + 2]) as usize]
                    };
                    let dest = match dst_mode.unwrap() {
                        Mode::Position => self.program[self.ip+3] as usize,
                        Mode::Immediate => self.ip+3,
                        Mode::Relative => (self.relative_base as i64 + self.program[self.ip + 3]) as usize
                    };
                    //println!("{}, {}, {}", lhs, rhs, dest);
                    self.program[dest as usize] = lhs * rhs;
                    self.ip += 4;
                },
                Op::Input => {
                    if let Some(input_values) = &mut input {
                        let dest = match lhs_mode.unwrap() {
                            Mode::Position => self.program[self.ip+1] as usize,
                            Mode::Immediate => self.ip+1,
                            Mode::Relative => (self.relative_base as i64 + self.program[self.ip + 1]) as usize
                        };
                        let inp = input_values.pop().unwrap();
                        self.program[dest as usize] = inp;
                        self.ip += 2;
                    }
                    else {
                        panic!("Program was expecting input but got none");
                    }
                },
                Op::Output => {
                    let src = match lhs_mode.unwrap() {
                        Mode::Position => self.program[self.program[self.ip+1] as usize],
                        Mode::Immediate => self.program[self.ip+1],
                        Mode::Relative => self.program[(self.relative_base as i64 + self.program[self.ip + 1]) as usize]
                    };
                    //println!("{}", src);
                    self.ip += 2;
                    return Some(src);
                },
                Op::JNZ => {
                    let operand = match lhs_mode.unwrap() {
                        Mode::Position => self.program[self.program[self.ip+1] as usize],
                        Mode::Immediate => self.program[self.ip+1],
                        Mode::Relative => self.program[(self.relative_base as i64 + self.program[self.ip + 1]) as usize]
                    };
                    let dest = match rhs_mode.unwrap() {
                        Mode::Position => self.program[self.program[self.ip+2] as usize],
                        Mode::Immediate => self.program[self.ip+2],
                        Mode::Relative => self.program[(self.relative_base as i64 + self.program[self.ip + 2]) as usize]
                    };
                    //println!("{}, {}", operand, dest);
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
                        Mode::Immediate => self.program[self.ip+1],
                        Mode::Relative => self.program[(self.relative_base as i64 + self.program[self.ip + 1]) as usize]
                    };
                    let dest = match rhs_mode.unwrap() {
                        Mode::Position => self.program[self.program[self.ip+2] as usize],
                        Mode::Immediate => self.program[self.ip+2],
                        Mode::Relative => self.program[(self.relative_base as i64 + self.program[self.ip + 2]) as usize]
                    };
                    //println!("{}, {}", operand, dest);
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
                        Mode::Immediate => self.program[self.ip+1],
                        Mode::Relative => self.program[(self.relative_base as i64 + self.program[self.ip + 1]) as usize]
                    };
                    let rhs = match rhs_mode.unwrap() {
                        Mode::Position => self.program[self.program[self.ip+2] as usize],
                        Mode::Immediate => self.program[self.ip+2],
                        Mode::Relative => self.program[(self.relative_base as i64 + self.program[self.ip + 2]) as usize]
                    };
                    let dest = match dst_mode.unwrap() {
                        Mode::Position => self.program[self.ip+3] as usize,
                        Mode::Immediate => self.ip+3,
                        Mode::Relative => (self.relative_base as i64 + self.program[self.ip + 3]) as usize
                    };
                    //println!("{}, {}, {}", lhs, rhs, dest);
                    self.program[dest as usize] = (lhs < rhs) as i64;
                    self.ip += 4;
                },
                Op::EQ => {
                    let lhs = match lhs_mode.unwrap() {
                        Mode::Position => self.program[self.program[self.ip+1] as usize],
                        Mode::Immediate => self.program[self.ip+1],
                        Mode::Relative => self.program[(self.relative_base as i64 + self.program[self.ip + 1]) as usize]
                    };
                    let rhs = match rhs_mode.unwrap() {
                        Mode::Position => self.program[self.program[self.ip+2] as usize],
                        Mode::Immediate => self.program[self.ip+2],
                        Mode::Relative => self.program[(self.relative_base as i64 + self.program[self.ip + 2]) as usize]
                    };
                    let dest = match dst_mode.unwrap() {
                        Mode::Position => self.program[self.ip+3] as usize,
                        Mode::Immediate => self.ip+3,
                        Mode::Relative => (self.relative_base as i64 + self.program[self.ip + 3]) as usize
                    };
                    //println!("{}, {}, {}", lhs, rhs, dest);
                    self.program[dest as usize] = (lhs == rhs) as i64;
                    self.ip += 4;
                },

                Op::ARB => {
                    let lhs = match lhs_mode.unwrap() {
                        Mode::Position => self.program[self.program[self.ip+1] as usize],
                        Mode::Immediate => self.program[self.ip+1],
                        Mode::Relative => self.program[(self.relative_base as i64 + self.program[self.ip + 1]) as usize]
                    };
                    //println!("{}", lhs);
                    self.relative_base = (self.relative_base as i64 + lhs) as usize;
                    self.ip += 2;
                }
                Op::Halt => {
                    self.halted = true;
                    break;
                }
            }
        }
        None
    }

    pub fn will_halt(&self) -> bool {
        let (op, _, _, _) = IntCodeMachine::parse_op(self.program[self.ip]);
        op == Op::Halt
    }
}
