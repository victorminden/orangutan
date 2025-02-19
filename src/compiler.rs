#[cfg(test)]
mod compiler_test;
mod symbol_table;

pub use self::symbol_table::*;
use crate::ast::{BlockStatement, Expression, Program, Statement};
use crate::code::{Bytecode, CompiledFunction, Constant, Instructions, OpCode};
use crate::object::Object;
use crate::token::Token;

use std::cell::RefCell;
use std::convert::TryFrom;
use std::mem;
use std::rc::Rc;

pub struct CompilationScope {
    instructions: Instructions,
    last_instruction: Option<EmittedInstruction>,
    previous_instruction: Option<EmittedInstruction>,
}

impl CompilationScope {
    pub fn new() -> Self {
        CompilationScope {
            instructions: vec![],
            last_instruction: None,
            previous_instruction: None,
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct EmittedInstruction {
    pub opcode: OpCode,
    pub position: usize,
}

pub struct Compiler {
    constants: Rc<RefCell<Vec<Constant>>>,
    symbol_table: Rc<RefCell<SymbolTable>>,
    scopes: Vec<CompilationScope>,
    scope_index: usize,
}

#[derive(Debug)]
pub enum CompileError {
    UnknownError,
    UnknownOperator,
    SymbolNotFound,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler::new_with_state(
            Rc::new(RefCell::new(SymbolTable::new_with_builtins())),
            Rc::new(RefCell::new(Vec::new())),
        )
    }

    pub fn new_with_state(
        symbol_table: Rc<RefCell<SymbolTable>>,
        constants: Rc<RefCell<Vec<Constant>>>,
    ) -> Self {
        Compiler {
            constants,
            symbol_table,
            scopes: vec![CompilationScope::new()],
            scope_index: 0,
        }
    }

    pub fn current_instructions(&self) -> &Instructions {
        &self.scopes[self.scope_index].instructions
    }

    // TODO: Determine if bytecode can return a reference / take ownership.
    pub fn bytecode(&self) -> Bytecode {
        Bytecode::new(
            self.current_instructions().clone(),
            self.constants.borrow().clone(),
        )
    }

    fn enter_scope(&mut self) {
        self.scopes.push(CompilationScope::new());
        self.symbol_table.borrow_mut().enter_scope();
        self.scope_index += 1;
    }

    fn leave_scope(&mut self) -> Result<Instructions, CompileError> {
        self.scope_index -= 1;
        if let Some(value) = self.scopes.pop() {
            self.symbol_table.borrow_mut().leave_scope();
            Ok(value.instructions)
        } else {
            Err(CompileError::UnknownError)
        }
    }

    fn load_symbol(&self, symbol: &Symbol) -> Instructions {
        match symbol.scope {
            SymbolScope::Global => OpCode::GetGlobal.make_u16(symbol.index),
            SymbolScope::Local => OpCode::GetLocal.make_u8(symbol.index as u8),
            SymbolScope::BuiltIn => OpCode::GetBuiltin.make_u8(symbol.index as u8),
            SymbolScope::Free => OpCode::GetFree.make_u8(symbol.index as u8),
            SymbolScope::Function => OpCode::CurrentClosure.make(),
        }
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
            }
            Statement::Let(name, expr) => {
                let symbol = self.symbol_table.borrow_mut().define(name).clone();
                self.compile_expression(expr)?;
                let insts = match symbol.scope {
                    SymbolScope::Global => OpCode::SetGlobal.make_u16(symbol.index),
                    SymbolScope::Local => OpCode::SetLocal.make_u8(symbol.index as u8),
                    _ => return Err(CompileError::UnknownError),
                };
                self.emit(insts);
            }
            Statement::Return(value) => {
                self.compile_expression(value)?;
                self.emit(OpCode::ReturnValue.make());
            }
        }
        Ok(())
    }

    fn compile_expression(&mut self, expression: &Expression) -> Result<(), CompileError> {
        match expression {
            Expression::Call(func, args) => {
                self.compile_expression(func)?;
                for expr in args {
                    self.compile_expression(expr)?;
                }
                self.emit(OpCode::Call.make_u8(args.len() as u8));
            }
            Expression::FunctionLiteral(parameters, block_statement, maybe_name) => {
                self.enter_scope();
                if let Some(name) = maybe_name {
                    self.symbol_table.borrow_mut().define_function_name(name);
                };
                for parameter in parameters {
                    self.symbol_table.borrow_mut().define(parameter);
                }
                self.compile_block_statement(block_statement)?;
                self.replace_last_pop_with_return();
                if !self.last_instruction_is(OpCode::ReturnValue) {
                    self.emit(OpCode::Return.make());
                }
                let free_symbols = self.symbol_table.borrow().free_symbols().clone();
                let num_locals = self.symbol_table.borrow().num_definitions();
                let instructions = self.leave_scope()?;
                for symbol in &free_symbols {
                    self.emit(self.load_symbol(symbol));
                }
                let compiled_function = CompiledFunction {
                    instructions,
                    num_locals,
                    num_parameters: parameters.len(),
                };
                let idx = self.add_constant(Constant::CompiledFunction(compiled_function));
                self.emit(OpCode::Closure.make_u16_u8(idx, free_symbols.len() as u8));
            }
            Expression::Ident(name) => {
                // Use a separate statement to catch the result so that we can unborrow the symbol_table.
                let symbol_result = self.symbol_table.borrow_mut().resolve(name);
                match symbol_result {
                    Ok(symbol) => {
                        let insts = self.load_symbol(&symbol);
                        self.emit(insts);
                    }
                    Err(_) => return Err(CompileError::SymbolNotFound),
                }
            }
            Expression::If(conditional, consequence, alternative) => {
                self.compile_expression(conditional)?;
                let jump_not_truthy_pos = self.emit(OpCode::JumpNotTruthy.make_u16(9999));
                self.compile_block_statement(&consequence)?;
                self.remove_last_pop();
                let jump_pos = self.emit(OpCode::Jump.make_u16(9999));
                self.replace_instructions(
                    jump_not_truthy_pos,
                    OpCode::JumpNotTruthy.make_u16(self.current_instructions().len() as u16),
                );
                match alternative {
                    None => {
                        self.emit(OpCode::Null.make());
                    }
                    Some(alt) => {
                        self.compile_block_statement(&alt)?;
                        self.remove_last_pop();
                    }
                }
                self.replace_instructions(
                    jump_pos,
                    OpCode::Jump.make_u16(self.current_instructions().len() as u16),
                );
            }
            Expression::Prefix(prefix, expr) => {
                self.compile_expression(expr)?;
                let opcode = match prefix {
                    Token::Bang => OpCode::Bang,
                    Token::Minus => OpCode::Minus,
                    _ => return Err(CompileError::UnknownOperator),
                };
                self.emit(opcode.make());
            }
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
            }
            Expression::IntegerLiteral(int) => {
                let int = Object::Integer(*int);
                let instructions = OpCode::Constant.make_u16(self.add_constant(int));
                self.emit(instructions);
            }
            Expression::StringLiteral(str) => {
                let str = Object::Str(str.clone());
                let instructions = OpCode::Constant.make_u16(self.add_constant(str));
                self.emit(instructions);
            }
            Expression::BooleanLiteral(bool) => {
                let opcode = if *bool { OpCode::True } else { OpCode::False };
                self.emit(opcode.make());
            }
            Expression::ArrayLiteral(elements) => {
                for expr in elements {
                    self.compile_expression(expr)?;
                }
                self.emit(OpCode::Array.make_u16(elements.len() as u16));
            }
            Expression::HashLiteral(keys_and_values) => {
                for (key, value) in keys_and_values {
                    self.compile_expression(key)?;
                    self.compile_expression(value)?;
                }
                self.emit(OpCode::Hash.make_u16(2 * keys_and_values.len() as u16));
            }
            Expression::Index(left, right) => {
                self.compile_expression(&left)?;
                self.compile_expression(&right)?;
                self.emit(OpCode::Index.make());
            }
        }
        Ok(())
    }

    fn add_constant(&mut self, constant: Constant) -> u16 {
        self.constants.borrow_mut().push(constant);
        return (self.constants.borrow().len() - 1) as u16;
    }

    pub fn emit(&mut self, ins: Instructions) -> usize {
        self.scopes[self.scope_index].emit(ins)
    }

    fn remove_last_pop(&mut self) {
        self.scopes[self.scope_index].remove_last_pop()
    }

    fn replace_instructions(&mut self, pos: usize, new_instructions: Instructions) {
        self.scopes[self.scope_index].replace_instructions(pos, new_instructions)
    }

    fn replace_last_pop_with_return(&mut self) {
        self.scopes[self.scope_index].replace_last_pop_with_return()
    }

    fn last_instruction_is(&self, op: OpCode) -> bool {
        self.scopes[self.scope_index].last_instruction_is(op)
    }
}

impl CompilationScope {
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
        if !self.last_instruction_is(OpCode::Pop) {
            return;
        }
        self.last_instruction = mem::replace(&mut self.previous_instruction, None);
        self.instructions.truncate(self.instructions.len() - 1);
    }

    fn replace_instructions(&mut self, pos: usize, new_instructions: Instructions) {
        // TODO: not safe.
        for (i, inst) in new_instructions.iter().enumerate() {
            self.instructions[pos + i] = *inst;
        }
    }

    fn last_instruction_is(&self, op: OpCode) -> bool {
        match &self.last_instruction {
            Some(inst) => inst.opcode == op,
            None => false,
        }
    }

    fn replace_last_pop_with_return(&mut self) {
        if !self.last_instruction_is(OpCode::Pop) {
            return;
        }
        let inst = match &mut self.last_instruction {
            Some(value) => value,
            _ => return,
        };
        inst.opcode = OpCode::ReturnValue;
        let last_pos = inst.position;
        self.replace_instructions(last_pos, OpCode::ReturnValue.make());
    }
}
