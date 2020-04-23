#[cfg(test)]
mod compiler_test;
mod symbol_table;

use crate::code::{Instructions, Constant, Bytecode, OpCode};
use crate::ast::{Program, Statement, Expression, BlockStatement};
use crate::object::Object;
use crate::token::Token;
pub use self::symbol_table::*;

use std::convert::TryFrom;
use std::mem;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(PartialEq, Eq)]
pub struct EmittedInstruction {
    pub opcode: OpCode,
    pub position: usize,
}

pub struct Compiler {
    instructions: Instructions,
    constants: Rc<RefCell<Vec<Constant>>>,
    last_instruction: Option<EmittedInstruction>,
    previous_instruction: Option<EmittedInstruction>,
    symbol_table: Rc<RefCell<SymbolTable>>,
} 

#[derive(Debug)]
pub enum CompileError {
    UnknownError,
    UnknownOperator,
    SymbolNotFound,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler { 
            instructions: Instructions::new(), 
            constants: Rc::new(RefCell::new(Vec::new())),
            last_instruction: None,
            previous_instruction: None,
            symbol_table: Rc::new(RefCell::new(SymbolTable::new())),
        }
    }

    pub fn new_with_state(symbol_table: Rc<RefCell<SymbolTable>>, constants: Rc<RefCell<Vec<Constant>>>) -> Self {
        Compiler {
            instructions: Instructions::new(),
            constants,
            last_instruction: None,
            previous_instruction: None,
            symbol_table,
        }
    }

    // TODO: Determine if bytecode can return a reference / take ownership.
    pub fn bytecode(&self) -> Bytecode {
        Bytecode::new(
            self.instructions.clone(), 
            self.constants.borrow().clone(),
        )
    }

    pub fn compile(&mut self, p: &Program) -> Result<Bytecode, CompileError> {
        for statement in &p.statements {
            self.compile_statement(statement)?;
        }
        Ok(self.bytecode())
    }

    pub fn compile_block_statement(&mut self, bs: &BlockStatement) -> Result<(), CompileError> {
        for statement in &bs.statements {
            self.compile_statement(statement)?;
        }
        Ok(())
    }

    fn compile_statement(&mut self, statement: &Statement) -> Result<(), CompileError> {
        match statement {
            Statement::Expression(expr) => {
                self.compile_expression(expr)?;
                self.emit(OpCode::Pop.make());
            },
            Statement::Let(name, expr) => {
                self.compile_expression(expr)?;
                let symbol = *self.symbol_table.borrow_mut().define(name);
                self.emit(OpCode::SetGlobal.make_u16(symbol.index));
            },
            _ => return Err(CompileError::UnknownError),
        }
        Ok(())
    }

    fn compile_expression(&mut self, expression: &Expression) -> Result<(), CompileError> {
        match expression {
            Expression::Ident(name) => {
                // Use a separate statement to catch the result so that we can unborrow the symbol_table.
                let symbol_result = self.symbol_table.borrow().resolve(name);
                match symbol_result {
                    Ok(symbol) => { self.emit(OpCode::GetGlobal.make_u16(symbol.index)); },
                    Err(_) => return Err(CompileError::SymbolNotFound),
                }
            },
            Expression::If(conditional, consequence, alternative) => {
                self.compile_expression(conditional)?;
                let jump_not_truthy_pos = self.emit(OpCode::JumpNotTruthy.make_u16(9999));
                self.compile_block_statement(&consequence)?;
                self.remove_last_pop();
                let jump_pos = self.emit(OpCode::Jump.make_u16(9999));
                self.replace_instructions(
                    jump_not_truthy_pos, 
                    OpCode::JumpNotTruthy.make_u16(self.instructions.len() as u16),
                );
                match alternative {
                    None => {
                        self.emit(OpCode::Null.make());
                    }
                    Some(alt) => {
                        self.compile_block_statement(&alt)?;
                        self.remove_last_pop();
                    },
                }
                self.replace_instructions(
                    jump_pos, 
                    OpCode::Jump.make_u16(self.instructions.len() as u16),
                );
                
            },
            Expression::Prefix(prefix, expr) => {
                self.compile_expression(expr)?;
                let opcode = match prefix {
                    Token::Bang => OpCode::Bang,
                    Token::Minus => OpCode::Minus,
                    _ => return Err(CompileError::UnknownOperator)
                };
                self.emit(opcode.make());
            },
            Expression::Infix(left, infix, right) => {
                match infix {
                    Token::LessThan => {
                        // Optimization to flip args and re-use GreaterThan.
                        self.compile_expression(right)?;
                        self.compile_expression(left)?;
                    }
                    _ => {
                        self.compile_expression(left)?;
                        self.compile_expression(right)?;
                    }
                }
                
                let opcode = match infix {
                    Token::Plus => OpCode::Add,
                    Token::Minus => OpCode::Sub,
                    Token::Asterisk => OpCode::Mul,
                    Token::Slash => OpCode::Div,
                    Token::Equal => OpCode::Equal,
                    Token::NotEqual => OpCode::NotEqual,
                    Token::GreaterThan | Token::LessThan => OpCode::GreaterThan,
                    _ => return Err(CompileError::UnknownOperator),
                };
                self.emit(opcode.make());
            },
            Expression::IntegerLiteral(int) => {
                let int = Object::Integer(*int);
                let instructions = OpCode::Constant.make_u16(self.add_constant(int));
                self.emit(instructions);
            },
            Expression::StringLiteral(str) => {
                let str = Object::Str(str.clone());
                let instructions = OpCode::Constant.make_u16(self.add_constant(str));
                self.emit(instructions);
            },
            Expression::BooleanLiteral(bool) => {
                let opcode = if *bool {OpCode::True} else {OpCode::False};
                self.emit(opcode.make());
            }
            _ => return Err(CompileError::UnknownError)
        }    
        Ok(())
    }

    fn add_constant(&mut self, constant: Constant) -> u16 {
        self.constants.borrow_mut().push(constant);
        return (self.constants.borrow().len() - 1) as u16;
    }

    // TODO: Determine if this function can be removed entirely.
    fn add_instruction(&mut self, ins: Instructions) -> usize {
        let pos_new_instruction = self.instructions.len();
        self.instructions.extend(ins);
        return pos_new_instruction;
    }

    fn emit(&mut self, ins: Instructions) -> usize {
        // TODO: Unwrap is Unsafe.
        let opcode = OpCode::try_from(ins[0]).unwrap();
        let pos = self.add_instruction(ins);
        self.set_last_instruction(opcode, pos);
        pos
    }

    fn set_last_instruction(&mut self, opcode: OpCode, position: usize) {
        self.previous_instruction = mem::replace(
            &mut self.last_instruction,
            Some(EmittedInstruction { opcode, position }),
        );
    }

    fn remove_last_pop(&mut self) {
        if let Some(inst) = &self.last_instruction {
            if inst.opcode != OpCode::Pop { return }
            self.last_instruction  = mem::replace(&mut self.previous_instruction, None);
            self.instructions.truncate(self.instructions.len()-1);
        }
    }

    fn replace_instructions(&mut self, pos: usize, new_instructions: Instructions) {
        // TODO: not safe.
        for (i, inst) in new_instructions.iter().enumerate() {
            self.instructions[pos + i] = *inst;
        }
    }

}
