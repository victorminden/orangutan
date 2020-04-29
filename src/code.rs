//! Code
//!
//! `code` contains functionality relating to bytecode for the Monkey language.
use crate::object::Object;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryFrom;
use std::fmt;
use std::rc::Rc;

pub type Instructions = Vec<u8>;
pub type ReadOnlyInstructions = [u8];
// TODO: Determine a space-efficient way of representing constants.
pub type Constant = Object;

#[derive(Debug, Clone)]
pub struct Closure {
    pub compiled_function: CompiledFunction,
    pub free: Vec<Rc<Object>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledFunction {
    pub instructions: Instructions,
    pub num_locals: usize,
    pub num_parameters: usize,
}

impl fmt::Display for CompiledFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CompiledFunction[{}]", disassemble(&self.instructions))
    }
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

#[derive(IntoPrimitive, TryFromPrimitive, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum OpCode {
    Null,
    Constant,
    Call,
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
    Minus,
    Bang,
    Jump,
    JumpNotTruthy,
    GetGlobal,
    SetGlobal,
    GetLocal,
    SetLocal,
    GetBuiltin,
    Array,
    Hash,
    Index,
    ReturnValue,
    Return,
    Closure,
}

impl OpCode {
    pub fn definition(&self) -> Definition {
        match self {
            OpCode::Closure => Definition {
                name: String::from("OpClosure"),
                widths: vec![2, 1],
            },
            OpCode::GetBuiltin => Definition {
                name: String::from("OpGetBuiltin"),
                widths: vec![1],
            },
            OpCode::Return => Definition {
                name: String::from("OpReturn"),
                widths: vec![],
            },
            OpCode::ReturnValue => Definition {
                name: String::from("OpReturnValue"),
                widths: vec![],
            },
            OpCode::Call => Definition {
                name: String::from("OpCall"),
                widths: vec![1],
            },
            OpCode::Index => Definition {
                name: String::from("OpIndex"),
                widths: vec![],
            },
            OpCode::Hash => Definition {
                name: String::from("OpHash"),
                widths: vec![2],
            },
            OpCode::Array => Definition {
                name: String::from("OpArray"),
                widths: vec![2],
            },
            OpCode::GetGlobal => Definition {
                name: String::from("OpGetGlobal"),
                widths: vec![2],
            },
            OpCode::SetGlobal => Definition {
                name: String::from("OpSetGlobal"),
                widths: vec![2],
            },
            OpCode::GetLocal => Definition {
                name: String::from("OpGetLocal"),
                widths: vec![1],
            },
            OpCode::SetLocal => Definition {
                name: String::from("OpSetLocal"),
                widths: vec![1],
            },
            OpCode::Constant => Definition {
                name: String::from("OpConstant"),
                widths: vec![2],
            },
            OpCode::Jump => Definition {
                name: String::from("OpJump"),
                widths: vec![2],
            },
            OpCode::JumpNotTruthy => Definition {
                name: String::from("OpJumpNotTruthy"),
                widths: vec![2],
            },
            OpCode::Null => Definition {
                name: String::from("OpNull"),
                widths: vec![],
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
            OpCode::Equal => Definition {
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
            OpCode::Minus => Definition {
                name: String::from("OpMinus"),
                widths: vec![],
            },
            OpCode::Bang => Definition {
                name: String::from("OpBang"),
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

    pub fn make_u8(self, operand: u8) -> Instructions {
        vec![self.into(), operand]
    }

    pub fn make_u16_u8(self, operand16: u16, operand8: u8) -> Instructions {
        let b = u16::to_be_bytes(operand16);
        vec![self.into(), b[0], b[1], operand8]
    }
}

pub fn read_operands(def: &Definition, instructions: &ReadOnlyInstructions) -> (Vec<u16>, usize) {
    let mut operands = Vec::with_capacity(def.widths.len());
    let mut offset = 0;
    for w in &def.widths {
        match w {
            2 => {
                operands.push(read_uint16(instructions[offset], instructions[offset + 1]));
            }
            1 => {
                // Even though the operand is 8-bit, we convert to 16 for read-out for ease of implementation.
                operands.push(instructions[offset] as u16)
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
            }
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
        let tests = vec![(1u8, OpCode::Constant)];

        assert_eq!(size_of::<OpCode>(), 1);
        for (input, want) in tests {
            let got = OpCode::try_from(input);
            assert_eq!(got, Ok(want));
        }
    }

    #[test]
    fn make_u16_test() {
        // Op, Operands, Expected
        let tests = vec![(
            OpCode::Constant,
            65534u16,
            vec![OpCode::Constant.into(), 255u8, 254u8],
        )];

        for (op, operand, want) in tests {
            let got = op.make_u16(operand);
            assert_eq!(got, want);
        }
    }

    #[test]
    fn make_u8_test() {
        // Op, Operands, Expected
        let tests = vec![(
            OpCode::Constant,
            255u8,
            vec![OpCode::Constant.into(), 255u8],
        )];

        for (op, operand, want) in tests {
            let got = op.make_u8(operand);
            assert_eq!(got, want);
        }
    }

    #[test]
    fn read_operands_test() {
        let tests = vec![(
            OpCode::Constant.make_u16(65535),
            OpCode::Constant.definition(),
            vec![65535],
            2,
        )];
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
            OpCode::Closure.make_u16_u8(65535, 255),
        ]
        .concat();
        let expected =
            "0000 OpAdd\n0001 OpConstant 2\n0004 OpConstant 65535\n0007 OpClosure 65535 255";
        assert_eq!(disassemble(&instructions), expected);
    }
}
