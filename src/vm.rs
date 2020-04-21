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
                OpCode::Constant => {
                    let const_idx = read_uint16(self.instructions[ip+1], self.instructions[ip+2]);
                    ip += 2;
                    self.push(self.constants[const_idx as usize].clone())?;
                },
                OpCode::Add => {
                    let right = self.pop()?;
                    let left = self.pop()?;
                    match (&*left, &*right) {
                        (Object::Integer(a), Object::Integer(b)) => { self.push(Rc::new(Object::Integer(a+b)))?; },
                        _ => return Err(VmError::UnsupportedOperands)
                    }
                }
                _ => {},
            }
            ip += 1
        }

        let result = &*self.top()?;
        Ok(result.clone())
    }

    fn top(&self) -> Result<Rc<Object>, VmError> {
        match self.sp {
            0 => Err(VmError::EmptyStack),
            _ => Ok(self.stack[self.sp-1].clone())
        }

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