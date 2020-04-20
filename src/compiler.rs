#[cfg(test)]
mod compiler_test;

use crate::code::{Instructions, Constant, Bytecode};
use crate::ast::Program;

pub struct Compiler {
    instructions: Instructions,
    constants: Vec<Constant>,
} 

#[derive(Debug)]
pub enum CompileError {
    UnknownError,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler { instructions: Instructions::new(), constants: vec![] }
    }

    // TODO: Determine if bytecode can return a reference / take ownership.
    pub fn bytecode(&self) -> Bytecode {
        Bytecode::new(
            self.instructions.clone(), 
            self.constants.clone(),
        )
    }

    pub fn compile(&mut self, program: &Program) -> Result<Bytecode, CompileError> {
        Err(CompileError::UnknownError)
    }
}
