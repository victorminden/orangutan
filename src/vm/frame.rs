use crate::code::{Closure, Instructions};

pub struct Frame {
    pub cl: Closure,
    pub ip: usize,
    pub bp: usize,
}

impl Frame {
    pub fn new(cl: Closure, base_pointer: usize) -> Self {
        Frame {
            cl,
            ip: 0,
            bp: base_pointer,
        }
    }

    pub fn instructions(&self) -> &Instructions {
        &self.cl.compiled_function.instructions
    }
}
