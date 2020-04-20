#[cfg(test)]
mod compiler_test;

use crate::code::{Instructions, Constant, Bytecode};
use crate::ast::{Program, Statement, Expression};
use crate::object::Object;

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

    pub fn compile(&mut self, p: &Program) -> Result<Bytecode, CompileError> {
        for statement in &p.statements {
            self.compile_statement(statement)?;
        }
        Ok(self.bytecode())
    }

    fn compile_statement(&mut self, statement: &Statement) -> Result<(), CompileError> {
        match statement {
            Statement::Expression(expr) => {
                self.compile_expression(expr)?;
            },
            _ => return Err(CompileError::UnknownError),
        }
        Ok(())
    }

    fn compile_expression(&mut self, expression: &Expression) -> Result<(), CompileError> {
        match expression {
            Expression::Infix(left, _, right) => {
                self.compile_expression(left)?;
                // TODO: Why are we not yet treating the infix operator?
                self.compile_expression(right)?;
            },
            Expression::IntegerLiteral(int) => {
                let int = Object::Integer(*int);
            }
            _ => return Err(CompileError::UnknownError)
        }    
        Ok(())
    }

    fn add_constant(&mut self, constant: Constant) -> usize {
        self.constants.push(constant);
        return self.constants.len() - 1;
    }

    fn add_instruction(&mut self, ins: Instructions) -> usize {
        let pos_new_instruction = self.instructions.len() - 1;
        self.instructions.extend(ins);
        return pos_new_instruction;
    }

}
