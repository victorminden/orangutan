#[cfg(test)]
mod vm_test;

use crate::object::Object;
use crate::code::{Bytecode, Constant, Instructions, OpCode, read_uint16};
use std::convert::TryFrom;
use std::rc::Rc;

const STACK_SIZE: usize = 2048;

#[derive(Debug)]
pub enum VmError {
    UnknownError,
    BadOpCode,
    EmptyStack,
    StackOverflow,
    StackUnderflow,
    UnsupportedOperands,
}

pub struct Vm {
    constants: Vec<Rc<Constant>>,
    instructions: Instructions, 
    stack: Vec<Rc<Object>>, // TODO: Check type
    sp: usize,
}

impl Vm {

    pub fn new(bytecode: &Bytecode) -> Self {
        let mut ref_counted_constants = vec![];
        for constant in &bytecode.constants {
            ref_counted_constants.push(Rc::new(constant.clone()));
        }
        let null_ref = Rc::new(Object::Null);
        Vm {
            constants: ref_counted_constants,
            instructions: bytecode.instructions.clone(),
            stack: vec![null_ref.clone(); STACK_SIZE],
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
                OpCode::Pop => {
                    self.pop()?;
                },
                OpCode::Constant => {
                    let const_idx = read_uint16(self.instructions[ip+1], self.instructions[ip+2]);
                    ip += 2;
                    self.push(self.constants[const_idx as usize].clone())?;
                },
                OpCode::Add | OpCode::Sub | OpCode::Mul | OpCode::Div => {
                    self.binary_op(op)?;
                }
                _ => {},
            }
            ip += 1
        }

        let result = &*self.last_top();
        Ok(result.clone())
    }

    fn binary_op(&mut self, op: OpCode) -> Result<(), VmError> {
        let right = self.pop()?;
        let left = self.pop()?;
        match (&*left, &*right) {
            (Object::Integer(left), Object::Integer(right)) => { 
              self.binary_integer_op(*left, op, *right)?;
            },
            _ => return Err(VmError::UnsupportedOperands)
        }
        Ok(())
    }

    fn binary_integer_op(&mut self, left: i64, op: OpCode, right: i64) -> Result<(), VmError> {
        let result = match op {
            OpCode::Add => left + right,
            OpCode::Sub => left - right,
            OpCode::Mul => left * right,
            OpCode::Div => left / right,
            _ => return Err(VmError::BadOpCode)
        };
        self.push(Rc::new(Object::Integer(result)))?;
        Ok(())
    }

    fn last_top(&self) -> Rc<Object> {
        self.stack[self.sp].clone()
    }

    fn push(&mut self, obj: Rc<Object>) -> Result<(), VmError> {
        if self.sp >= STACK_SIZE {
            return Err(VmError::StackOverflow);
        }
        self.stack[self.sp] = obj;
        self.sp += 1;
        Ok(())
    }

    fn pop(&mut self) -> Result<Rc<Object>, VmError> {
        if self.sp == 0 {
            return Err(VmError::StackUnderflow);
        }
        // TODO: Remove slow clones.
        let obj = self.stack[self.sp - 1].clone();
        self.sp -= 1;
        Ok(obj)
    }
}