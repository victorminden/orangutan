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

// TODO: Determine if this leads to a 1-byte Opcode we can use.
#[derive(IntoPrimitive, TryFromPrimitive, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Opcode {
    Constant,
    OtherConstant,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opcode_test() {
        let tests = vec![
            (0u8, Opcode::Constant),
            (1u8, Opcode::OtherConstant),
        ];

        for (input, want) in tests {
            assert_eq!(Opcode::try_from(input), Ok(want));
        }
    }
}