#[cfg(test)]
mod vm_test;

use crate::object::Object;
use crate::code::{Bytecode, Constant, Instructions, OpCode, read_uint16};
use std::convert::TryFrom;

const STACK_SIZE: usize = 2048;

#[derive(Debug)]
pub enum VmError {
    UnknownError,
    BadOpCode,
    EmptyStack,
    StackOverflow,
}

pub struct Vm {
    constants: Vec<Constant>,
    instructions: Instructions, 
    stack: Vec<Object>, // TODO: Check type
    sp: usize,
}

impl Vm {

    pub fn new(bytecode: &Bytecode) -> Self {
        Vm {
            constants: bytecode.constants.clone(),
            instructions: bytecode.instructions.clone(),
            stack: vec![Object::Null; STACK_SIZE],
            sp: 0,
        }
    }

    pub fn run(&mut self) -> Result<Object, VmError> {
        let mut ip = 0;
        while ip < self.instructions.len() {
            let op = match OpCode::try_from(self.instructions[ip]) {
                Ok(op) => op,
                _ => return Err(VmError::BadOpCode),
            };
            match op {
                OpCode::Constant => {
                    let const_idx = read_uint16(self.instructions[ip+1], self.instructions[ip+2]);
                    ip += 2;
                    // TODO: Remove the super slow clones...
                    self.push(self.constants[const_idx as usize].clone())?;
                }
                _ => {},
            }
            ip += 1
        }
        self.stack_top()
    }

    fn stack_top(&self) -> Result<Object, VmError> {
        match self.sp {
            0 => Err(VmError::EmptyStack),
            _ => Ok(self.stack[self.sp-1].clone())
        }

    }

    fn push(&mut self, obj: Object) -> Result<(), VmError> {
        if self.sp >= STACK_SIZE {
            return Err(VmError::StackOverflow);
        }
        self.stack[self.sp] = obj;
        self.sp += 1;
        Ok(())
    }
}