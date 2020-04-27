#[cfg(test)]
mod vm_test;
mod frame;

use crate::vm::frame::Frame;
use crate::object::Object;
use crate::code::{Bytecode, Constant, Instructions, OpCode, read_uint16, CompiledFunction};
use std::convert::TryFrom;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

const STACK_SIZE: usize = 2048;
const MAX_FRAMES: usize = 1024;
const GLOBALS_SIZE: usize = 65536;

#[derive(Debug)]
pub enum VmError {
    UnknownError,
    BadOpCode,
    StackOverflow,
    StackUnderflow,
    UnsupportedOperands,
    CallingNonFunction,
}

pub struct Vm {
    constants: Vec<Rc<Constant>>,
    globals: Rc<RefCell<Vec<Rc<Object>>>>,
    stack: Vec<Rc<Object>>, // TODO: Check type
    sp: usize,
    frames: Vec<Frame>,
    frames_index: usize,
    // TODO: Determine a better way to have these constants.
    true_obj: Rc<Object>,
    false_obj: Rc<Object>,
    null_obj: Rc<Object>,
}

impl Vm {
    pub fn new(bytecode: &Bytecode) -> Self {
        // TODO: Would be nice to make this the same reference as in new_with_globals_store.
        let null_ref = Rc::new(Object::Null);
        Vm::new_with_globals_store(bytecode, 
            Rc::new(RefCell::new(vec![null_ref.clone(); GLOBALS_SIZE])))
    }

    fn current_frame(&mut self) -> &mut Frame {
        &mut self.frames[self.frames_index-1]
    }

    fn push_frame(&mut self, frame: Frame) {
        self.frames_index += 1;
        self.frames.push(frame);
    }

    fn pop_frame(&mut self) -> Result<Frame, VmError> {
        self.frames_index -= 1;
        match self.frames.pop() {
            None => Err(VmError::UnknownError),
            Some(other) => Ok(other),
        }
    }

    pub fn new_with_globals_store(bytecode: &Bytecode, store: Rc<RefCell<Vec<Rc<Object>>>>) -> Self {
        let mut ref_counted_constants = vec![];
        for constant in &bytecode.constants {
            ref_counted_constants.push(Rc::new(constant.clone()));
        }
        let main_function = CompiledFunction { instructions: bytecode.instructions.clone(), num_locals: 0 };
        let null_ref = Rc::new(Object::Null);
        let mut frames = Vec::with_capacity(MAX_FRAMES);
        frames.push(Frame::new(main_function, 0));
        Vm {
            constants: ref_counted_constants,
            globals: store,
            stack: vec![null_ref.clone(); STACK_SIZE],
            sp: 0,
            frames,
            frames_index: 1,
            true_obj: Rc::new(Object::Boolean(true)),
            false_obj: Rc::new(Object::Boolean(false)),
            null_obj: null_ref.clone(),
        }
    }

    fn increment_ip(&mut self, val: usize) {
        self.current_frame().ip += val;
    }
    
    fn set_ip(&mut self, val: usize) {
        self.current_frame().ip = val;
    }

    pub fn run(&mut self) -> Result<Object, VmError> {
        while self.current_frame().ip < self.current_frame().instructions().len() {
            let ip = self.current_frame().ip;
            let ins = self.current_frame().instructions();
            let op = match OpCode::try_from(ins[ip]) {
                Ok(op) => op,
                _ => return Err(VmError::BadOpCode),
            };
            match op {
                OpCode::Return => {
                    let frame = self.pop_frame()?;
                    self.sp = frame.bp-1;
                    self.push(self.null_obj.clone())?;
                },
                OpCode::ReturnValue => {
                    let return_value = self.pop()?;
                    let frame = self.pop_frame()?;
                    self.sp = frame.bp-1;
                    self.push(return_value)?;
                },
                OpCode::Call => {
                    self.increment_ip(1);
                    let func = (*self.stack[self.sp-1]).clone();
                    match func {
                        Object::CompiledFunction(func) => {
                            let num_locals = func.num_locals;
                            self.push_frame(Frame::new(func, self.sp));
                            self.sp += num_locals;
                            continue;
                        },
                        _ => return Err(VmError::CallingNonFunction)
                    }
                },
                OpCode::Index => {
                    let index = self.pop()?;
                    let left = self.pop()?;
                    self.index_expression(left, index)?;
                },
                OpCode::Hash => {
                    let num_elements = read_uint16(ins[ip+1], ins[ip+2]);
                    self.increment_ip(2);
                    let mut hash_map = HashMap::new();
                    for _ in 0..num_elements/2 {
                        // TODO: Stop the cloning...
                        let value = (*self.pop()?).clone();
                        if let Ok(key) = (*self.pop()?).clone().to_hashable_object() {
                            hash_map.insert(key, value);
                        } else {
                            return Err(VmError::UnsupportedOperands);
                        }
                    }
                    let hash = Rc::new(Object::Hash(hash_map));
                    self.push(hash)?;
                },
                OpCode::Array => {
                    let num_elements = read_uint16(ins[ip+1], ins[ip+2]);
                    self.increment_ip(2);
                    let mut elements = Vec::with_capacity(num_elements as usize);
                    for _ in 0..num_elements {
                        // TODO: If we modify the array class to hold Rc elements, we don't have to clone here.
                        elements.push((*self.pop()?).clone());
                    }
                    elements.reverse();
                    let array = Rc::new(Object::Array(elements));
                    self.push(array)?;
                },
                OpCode::SetGlobal => {
                    let global_idx = read_uint16(ins[ip+1], ins[ip+2]);
                    self.increment_ip(2);
                    let element = self.pop()?;
                    // TODO: Insert moves things to the right!  This is probably not what we want to do!
                    self.globals.borrow_mut()[global_idx as usize] = element;
                },
                OpCode::GetGlobal => {
                    let global_idx = read_uint16(ins[ip+1], ins[ip+2]);
                    self.increment_ip(2);
                    let element = match self.globals.borrow().get(global_idx as usize) {
                        Some(elem) => elem.clone(),
                        _ => return Err(VmError::UnknownError),
                    };
                    self.push(element)?;
                },
                OpCode::SetLocal => {
                    let local_idx = ins[ip+1];
                    self.increment_ip(1);
                    let element = self.pop()?;
                    let idx = self.current_frame().bp + local_idx as usize;
                    self.stack[idx] = element;
                },
                OpCode::GetLocal => {
                    let local_idx = ins[ip+1];
                    self.increment_ip(1);
                    let idx = self.current_frame().bp + local_idx as usize;
                    let element = self.stack[idx].clone();
                    self.push(element)?;
                },
                OpCode::True => self.push(self.true_obj.clone())?,
                OpCode::False => self.push(self.false_obj.clone())?,
                OpCode::Null => self.push(self.null_obj.clone())?,
                OpCode::Pop => { self.pop()?; },
                OpCode::Constant => {
                    let const_idx = read_uint16(ins[ip+1], ins[ip+2]);
                    self.increment_ip(2);
                    self.push(self.constants[const_idx as usize].clone())?;
                },
                OpCode::Bang => {
                    let result = match &*self.pop()? {
                        Object::Boolean(false) | Object::Null => true,
                        _ => false,
                    };
                    if result {
                        self.push(self.true_obj.clone())?;
                    } else {
                        self.push(self.false_obj.clone())?;
                    }
                    
                },
                OpCode::Add | OpCode::Sub | OpCode::Mul | OpCode::Div => self.binary_op(op)?,
                OpCode::Equal | OpCode::NotEqual | OpCode::GreaterThan => self.comparison_op(op)?,
                OpCode::Minus => {
                    let value = match &*self.pop()? {
                        Object::Integer(val) => *val,
                        _ => return Err(VmError::UnsupportedOperands),
                    };
                    self.push(Rc::new(Object::Integer(-value)))?;
                },
                OpCode::Jump => {
                    let jump_pos = read_uint16(ins[ip+1], ins[ip+2]);
                    self.set_ip((jump_pos - 1) as usize);
                },
                OpCode::JumpNotTruthy => {
                    let jump_pos = read_uint16(ins[ip+1], ins[ip+2]);
                    self.increment_ip(2);
                    let value = &*self.pop()?;
                    if !value.is_truthy() {
                        self.set_ip((jump_pos - 1) as usize);
                    }
                },
                _ => {},
            }
            self.increment_ip(1);
        }
        let result = &*self.last_top();
        Ok(result.clone())
    }

    fn comparison_op(&mut self, op: OpCode) -> Result<(), VmError> {
        let right = self.pop()?;
        let left = self.pop()?;
        match (&*left, &*right) {
            (Object::Boolean(left), Object::Boolean(right)) => {
                self.comparison_boolean_op(*left, op, *right)?;
            },
            (Object::Integer(left), Object::Integer(right)) => { 
              self.comparison_integer_op(*left, op, *right)?;
            },
            _ => return Err(VmError::UnsupportedOperands)
        }
        Ok(())
    }

    fn comparison_boolean_op(&mut self, left: bool, op: OpCode, right: bool) -> Result<(), VmError> {
        let result = match op {
            OpCode::Equal => left == right,
            OpCode::NotEqual => left != right,
            _ => return Err(VmError::BadOpCode)
        };
        if result {
            self.push(self.true_obj.clone())?;
        } else {
            self.push(self.false_obj.clone())?;
        }
        Ok(())
    }

    fn comparison_integer_op(&mut self, left: i64, op: OpCode, right: i64) -> Result<(), VmError> {
        let result = match op {
            OpCode::Equal => left == right,
            OpCode::NotEqual => left != right,
            OpCode::GreaterThan => left > right,
            _ => return Err(VmError::BadOpCode)
        };
        if result {
            self.push(self.true_obj.clone())?;
        } else {
            self.push(self.false_obj.clone())?;
        }
        Ok(())
    }

    fn binary_op(&mut self, op: OpCode) -> Result<(), VmError> {
        let right = self.pop()?;
        let left = self.pop()?;
        match (&*left, &*right) {
            (Object::Integer(left), Object::Integer(right)) => { 
              self.binary_integer_op(*left, op, *right)?;
            },
            (Object::Str(left), Object::Str(right)) => {
                self.binary_string_op(left, op, right)?;
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

    fn binary_string_op(&mut self, left: &String, op: OpCode, right: &String) -> Result<(), VmError> {
        let result = match op {
            OpCode::Add => format!("{}{}", left, right),
            _ => return Err(VmError::BadOpCode)
        };
        self.push(Rc::new(Object::Str(result)))?;
        Ok(())
    }


    fn index_expression(&mut self, left: Rc<Object>, index: Rc<Object>) -> Result<(), VmError> {
        match (&*left, &*index) {
            (Object::Array(elements), Object::Integer(idx)) => {
                match elements.get(*idx as usize) {
                    Some(thing) => {
                        self.push(Rc::new(thing.clone()))?;
                    },
                    None => { self.push(self.null_obj.clone())?; }
                }
            },
            (Object::Hash(keys_and_values), _) => {
                match (*index).clone().to_hashable_object() {
                    Ok(key) => {
                        let obj = match keys_and_values.get(&key) {
                            Some(elem) => Rc::new(elem.clone()),
                            _ => self.null_obj.clone(),
                        };
                        self.push(obj)?;
                    },
                    _ => return Err(VmError::UnsupportedOperands)
                }
            },
            _ => return Err(VmError::UnsupportedOperands)
        }
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
        let obj = self.stack[self.sp - 1].clone();
        self.sp -= 1;
        Ok(obj)
    }
}