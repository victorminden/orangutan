//! Code
//! 
//! `code` contains functionality relating to bytecode for the Monkey language.
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryFrom;

pub type Instructions = Vec<u8>;

pub struct Definition {
    pub name: String,
    pub widths: Vec<usize>,
}

pub enum MakeError {
    WrongNumberOfArgs(usize),
    WrongArgSize(usize),
}

#[derive(IntoPrimitive, TryFromPrimitive, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Opcode {
    Constant,
}

impl Opcode {
    pub fn definition(&self) -> Definition {
        match self {
            Opcode::Constant => Definition {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn opcode_test() {
        let tests = vec![
            (0u8, Opcode::Constant),
        ];

        assert_eq!(size_of::<Opcode>(), 1);
        for (input, want) in tests {
            let got = Opcode::try_from(input);
            assert_eq!(got, Ok(want));
        }
    }

    #[test]
    fn make_u16_test() {
        // Op, Operands, Expected
        let tests = vec![
            (Opcode::Constant, 65534u16, vec![Opcode::Constant.into(), 255u8, 254u8]),
        ];

        for (op, operand, want) in tests {
            match op.make_u16(operand) {
                Ok(got) => assert_eq!(got, want),
                _ => panic!("Got error!")
            }
        }
    }
}