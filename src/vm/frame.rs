use crate::code::{CompiledFunction, Instructions};

pub struct Frame {
    pub func: CompiledFunction,
    pub ip: usize,
    pub bp: usize,
}

impl Frame {
    pub fn new(func: CompiledFunction, base_pointer: usize) -> Self {
        Frame {
            func,
            ip: 0,
            bp: base_pointer,
        }
    }

    pub fn instructions(&self) -> &Instructions {
        &self.func.instructions
    }
}
