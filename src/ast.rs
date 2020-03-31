//use std::fmt;
//use crate::token::Token;

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Let(String, Expression),
    Return(Expression),
}

#[derive(Debug)]
pub enum Expression {
    Null,
    Identifier(String),
}