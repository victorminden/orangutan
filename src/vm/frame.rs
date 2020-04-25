use crate::code::{CompiledFunction, Instructions};

pub struct Frame {
    pub func: CompiledFunction,
    pub ip: usize,
}

impl Frame {
    pub fn new(func: CompiledFunction) -> Self {
        Frame { 
            func,
            ip: 0,
        }
    }

    pub fn instructions(&self) -> &Instructions {
        &self.func.instructions
    }
}