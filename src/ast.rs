use crate::token::Token;

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Let(String, Expression),
    Return(Expression),
    Expression(Expression),
}

#[derive(Debug)]
pub enum Expression {
    Null,
    Ident(String),
    IntegerLiteral(i32),
    Prefix(Token, Box<Expression>),
    Infix(Box<Expression>, Token, Box<Expression>),
}
