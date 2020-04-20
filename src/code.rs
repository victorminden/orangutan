//! Code
//! 
//! `code` contains functionality relating to bytecode for the Monkey language.
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryFrom;
use crate::object::Object;

pub type Instructions = Vec<u8>;
pub type ReadOnlyInstructions = [u8];

// TODO: Determine a space-efficient way of representing constants.
#[derive(Clone)]
pub enum Constant {
    Integer(u16),
}

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

#[derive(Debug)]
pub enum MakeError {
    WrongNumberOfArgs(usize),
    WrongArgSize(usize),
}

#[derive(IntoPrimitive, TryFromPrimitive, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum OpCode {
    Constant,
}

impl OpCode {
    pub fn definition(&self) -> Definition {
        match self {
            OpCode::Constant => Definition {
                name: String::from("OpConstant"),
                widths: vec![2],
            },
        }
    }

    pub fn make(self) -> Result<Instructions, MakeError> {
        match self.definition().widths.len() {
            0 => Ok(vec![self.into()]),
            other => Err(MakeError::WrongNumberOfArgs(other))
        } 
    }

    pub fn make_u16(self, operand: u16) -> Result<Instructions, MakeError> {
        let w = &self.definition().widths;
        if w.len() != 1 {
            Err(MakeError::WrongNumberOfArgs(w.len()))
        } else if w[0] != 2 {
            Err(MakeError::WrongArgSize(w[0]))
        } else {
            let b = u16::to_be_bytes(operand);
            Ok(vec![self.into(), b[0], b[1]])
        }
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
            match op.make_u16(operand) {
                Ok(got) => assert_eq!(got, want),
                _ => panic!("Got error!")
            }
        }
    }

    #[test]
    fn read_operands_test() {
        let tests = vec![
            (OpCode::Constant.make_u16(65535).unwrap(), OpCode::Constant.definition(), vec![65535], 2),
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
            OpCode::Constant.make_u16(1).unwrap(),
            OpCode::Constant.make_u16(2).unwrap(),
            OpCode::Constant.make_u16(65535).unwrap(),
        ].concat();
        let expected = "0000 OpConstant 1\n0003 OpConstant 2\n0006 OpConstant 65535";
        assert_eq!(disassemble(&instructions), expected);
    }
}