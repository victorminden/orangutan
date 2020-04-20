#[cfg(test)]
mod compiler_test;

use crate::code::{Instructions, Constant, Bytecode, OpCode};
use crate::ast::{Program, Statement, Expression};
use crate::object::Object;
use crate::token::Token;

pub struct Compiler {
    instructions: Instructions,
    constants: Vec<Constant>,
} 

#[derive(Debug)]
pub enum CompileError {
    UnknownError,
    UnknownOperator,
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
            Expression::Infix(left, infix, right) => {
                self.compile_expression(left)?;
                self.compile_expression(right)?;
                match infix {
                    Token::Plus => { self.emit(OpCode::Add.make()); },
                    _ => return Err(CompileError::UnknownOperator),
                }
            },
            Expression::IntegerLiteral(int) => {
                let int = Object::Integer(*int);
                let instructions = OpCode::Constant.make_u16(self.add_constant(int));
                self.emit(instructions);
            }
            _ => return Err(CompileError::UnknownError)
        }    
        Ok(())
    }

    fn add_constant(&mut self, constant: Constant) -> u16 {
        self.constants.push(constant);
        return (self.constants.len() - 1) as u16;
    }

    // TODO: Determine if this function can be removed entirely.
    fn add_instruction(&mut self, ins: Instructions) -> usize {
        let pos_new_instruction = self.instructions.len();
        self.instructions.extend(ins);
        return pos_new_instruction;
    }

    fn emit(&mut self, ins: Instructions) -> usize {
        return self.add_instruction(ins);
    }

}
