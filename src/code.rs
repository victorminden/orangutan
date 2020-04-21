//! Code
//! 
//! `code` contains functionality relating to bytecode for the Monkey language.
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryFrom;
use crate::object::Object;

pub type Instructions = Vec<u8>;
pub type ReadOnlyInstructions = [u8];
// TODO: Determine a space-efficient way of representing constants.
pub type Constant = Object;

pub struct Bytecode {
    pub instructions: Instructions,
    pub constants: Vec<Constant>,
}

impl Bytecode {
    pub fn new(instructions: Instructions, constants: Vec<Constant>) -> Self {
        Bytecode {
            instructions: instructions,
            constants: constants,
        }
    }
}

pub struct Definition {
    pub name: String,
    pub widths: Vec<usize>,
}

#[derive(IntoPrimitive, TryFromPrimitive, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum OpCode {
    Constant,
    Add,
    Sub,
    Mul,
    Div,
    Pop,
    True,
    False,
    Equal,
    NotEqual,
    GreaterThan,
}

impl OpCode {
    pub fn definition(&self) -> Definition {
        match self {
            OpCode::Constant => Definition {
                name: String::from("OpConstant"),
                widths: vec![2],
            },
            OpCode::Add => Definition {
                name: String::from("OpAdd"),
                widths: vec![],
            },
            OpCode::Sub => Definition {
                name: String::from("OpSub"),
                widths: vec![],
            },
            OpCode::Mul => Definition {
                name: String::from("OpMul"),
                widths: vec![],
            },
            OpCode::Div => Definition {
                name: String::from("OpDiv"),
                widths: vec![],
            },
            OpCode::Pop => Definition {
                name: String::from("OpPop"),
                widths: vec![],
            },
            OpCode::True => Definition {
                name: String::from("OpTrue"),
                widths: vec![],
            },
            OpCode::False => Definition {
                name: String::from("OpFalse"),
                widths: vec![],
            },
            OpCode::Equal=> Definition {
                name: String::from("OpEqual"),
                widths: vec![],
            },
            OpCode::NotEqual => Definition {
                name: String::from("OpNotEqual"),
                widths: vec![],
            },
            OpCode::GreaterThan => Definition {
                name: String::from("OpGreaterThan"),
                widths: vec![],
            },
        }
    }

    pub fn make(self) -> Instructions {
        vec![self.into()]
    }

    pub fn make_u16(self, operand: u16) -> Instructions {
        let b = u16::to_be_bytes(operand);
        vec![self.into(), b[0], b[1]]
    }
}

pub fn read_operands(def: &Definition, instructions: &ReadOnlyInstructions) -> (Vec<u16>, usize) {
    let mut operands = Vec::with_capacity(def.widths.len());
    let mut offset = 0;
    for w in &def.widths {
        match w {
            2 => {
                operands.push(read_uint16(instructions[offset], instructions[offset+1]));
            }
            _ => panic!("The requested operand size was invalid!"),
        }
        offset += w;
    }
    (operands, offset)
}

pub fn read_uint16(b0: u8, b1: u8) -> u16 {
    u16::from_be_bytes([b0, b1])
}

pub fn disassemble(instructions: &ReadOnlyInstructions) -> String {
    let mut all_instructions = vec![];
    let mut ip = 0;
    while ip < instructions.len() {
        let mut current_instruction = vec![];
        current_instruction.push(format!("{:04}", ip));
        let op = OpCode::try_from(instructions[ip]);
        ip += 1;
        match op {
            Err(_) => current_instruction.push(String::from("ERROR")),
            Ok(op) => {
                let def = op.definition();
                current_instruction.push(format!("{}", def.name));
                let (operands, n) = read_operands(&def, &instructions[ip..]);
                for o in operands {
                    current_instruction.push(format!("{}", o));
                }
                ip += n;
                all_instructions.push(current_instruction.join(" "));
            },
        }
    }
    all_instructions.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn opcode_test() {
        let tests = vec![
            (0u8, OpCode::Constant),
        ];

        assert_eq!(size_of::<OpCode>(), 1);
        for (input, want) in tests {
            let got = OpCode::try_from(input);
            assert_eq!(got, Ok(want));
        }
    }

    #[test]
    fn make_u16_test() {
        // Op, Operands, Expected
        let tests = vec![
            (OpCode::Constant, 65534u16, vec![OpCode::Constant.into(), 255u8, 254u8]),
        ];

        for (op, operand, want) in tests {
            let got = op.make_u16(operand);
            assert_eq!(got, want);
        }
    }

    #[test]
    fn read_operands_test() {
        let tests = vec![
            (OpCode::Constant.make_u16(65535), OpCode::Constant.definition(), vec![65535], 2),
        ];
        for (instructions, def, want_operands, want_n) in tests {
            let (operands, n) = read_operands(&def, &instructions[1..]);
            assert_eq!(n, want_n);
            for (i, operand) in want_operands.iter().enumerate() {
                assert_eq!(*operand as u16, operands[i]);
            }
        }
    }

    #[test]
    fn disassemble_test() {
        let instructions = vec![
            OpCode::Add.make(),
            OpCode::Constant.make_u16(2),
            OpCode::Constant.make_u16(65535),
        ].concat();
        let expected = "0000 OpAdd\n0001 OpConstant 2\n0004 OpConstant 65535";
        assert_eq!(disassemble(&instructions), expected);
    }
}