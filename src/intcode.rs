use std::convert::TryInto;

#[repr(u32)]
#[derive(PartialEq)]
enum Opcode {
    ADD = 1,
    MUL = 2,
    IN = 3,
    OUT = 4,
    JNEZ = 5,
    JEZ = 6,
    LT = 7,
    EQ = 8,
    REL = 9,
    HALT = 99,
}

impl From<u32> for Opcode {
    fn from(n: u32) -> Self {
        let opcode = n % 100;
        use Opcode::*;
        match opcode {
            1 => ADD,
            2 => MUL,
            3 => IN,
            4 => OUT,
            5 => JNEZ,
            6 => JEZ,
            7 => LT,
            8 => EQ,
            9 => REL,
            99 => HALT,
            _ => panic!("Unknown opcode {}", opcode),
        }
    }
}

#[repr(u32)]
#[derive(PartialEq)]
enum Mode {
    Position = 0,
    Immediate = 1,
    Relative = 2,
}

impl From<u32> for Mode {
    fn from(n: u32) -> Self {
        use Mode::*;
        match n {
            0 => Position,
            1 => Immediate,
            2 => Relative,
            _ => panic!("Invalid mode {}", n),
        }
    }
}

fn get_modes_from_instruction(instruction: u32) -> (Mode, Mode, Mode) {
    use Mode::*;
    match instruction {
        0..=99 => (Position, Position, Position),
        100..=999 => (Mode::from((instruction / 100) % 10), Position, Position),
        1000..=9999 => (
            Mode::from((instruction / 100) % 10),
            Mode::from((instruction / 1000) % 10),
            Position,
        ),
        10000..=99999 => (
            Mode::from((instruction / 100) % 10),
            Mode::from((instruction / 1000) % 10),
            Mode::from((instruction / 10000) % 10),
        ),
        _ => panic!("Invalid instruction {}", instruction),
    }
}

pub fn run_program(
    prog: &mut [i64],
    pc: &mut usize,
    relative_base: &mut i64,
    inputs: &[i64],
) -> Option<i64> {
    let mut current_input = 0;
    loop {
        let instruction = prog[*pc] as u32;
        let (lhs_mode, rhs_mode, dest_mode) = get_modes_from_instruction(instruction);

        let opcode = Opcode::from(instruction);

        use {Mode::*, Opcode::*};
        let get_index = |mode: Mode, offset: usize| match mode {
            Immediate => *pc + offset,
            Position => prog[*pc + offset].try_into().unwrap(),
            Relative => (*relative_base + prog[*pc + offset]).try_into().unwrap(),
        };

        match opcode {
            ADD | MUL | LT | EQ => {
                let lhs = prog[get_index(lhs_mode, 1)];
                let rhs = prog[get_index(rhs_mode, 2)];
                assert!(dest_mode != Immediate);

                prog[get_index(dest_mode, 3)] = match opcode {
                    ADD => lhs + rhs,
                    MUL => lhs * rhs,
                    LT => {
                        if lhs < rhs {
                            1
                        } else {
                            0
                        }
                    }
                    EQ => {
                        if lhs == rhs {
                            1
                        } else {
                            0
                        }
                    }
                    _ => unreachable!(),
                };

                *pc += 4;
            }
            JNEZ | JEZ => {
                let lhs = prog[get_index(lhs_mode, 1)];

                let should_jump = if opcode == JNEZ { lhs != 0 } else { lhs == 0 };
                if should_jump {
                    let rhs = prog[get_index(rhs_mode, 2)];
                    *pc = rhs.try_into().unwrap();
                    continue;
                }

                *pc += 3;
            }
            REL => {
                assert!(rhs_mode != Immediate);

                let offset = prog[get_index(lhs_mode, 1)];
                *relative_base += offset;
                assert!(*relative_base > 0);

                *pc += 2;
            }
            IN => {
                assert!(rhs_mode != Immediate);
                assert!(lhs_mode != Immediate);

                prog[get_index(lhs_mode, 1)] = inputs[current_input];
                current_input += 1;

                *pc += 2;
            }
            OUT => {
                assert!(rhs_mode != Immediate);

                let value = prog[get_index(lhs_mode, 1)];

                *pc += 2;

                return Some(value);
            }
            HALT => return None,
        };

        assert!(*pc < prog.len());
    }
}

pub struct Program {
    prog: Vec<i64>,
    inputs: Vec<i64>,
    pc: usize,
    relative_base: i64,
}

impl Program {
    pub fn new(prog: Vec<i64>, inputs: Vec<i64>) -> Self {
        Self {
            prog,
            inputs,
            pc: 0,
            relative_base: 0,
        }
    }
}

impl Iterator for Program {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        run_program(
            &mut self.prog,
            &mut self.pc,
            &mut self.relative_base,
            &self.inputs,
        )
    }
}
