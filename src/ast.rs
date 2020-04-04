use std::fmt;
use crate::token::Token;

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Program:")?;
        for stmt in &self.statements {
            write!(f, "{}", stmt)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Statement {
    Let(String, Expression),
    Return(Expression),
    Expression(Expression),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::Let(ident, expr) => write!(f, "let {} = {};", ident, expr),
            Statement::Return(expr) => write!(f, "return {};", expr),
            Statement::Expression(expr) => write!(f, "{};", expr),
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Null,
    Ident(String),
    IntegerLiteral(i32),
    BooleanLiteral(bool),
    Prefix(Token, Box<Expression>),
    Infix(Box<Expression>, Token, Box<Expression>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Null => write!(f, "<null>"),
            Expression::Ident(ident) => write!(f, "{}", ident),
            Expression::IntegerLiteral(i) => write!(f, "{}", i),
            Expression::BooleanLiteral(b) => write!(f, "{}", b),
            Expression::Prefix(token, expr) => write!(f, "({}{})", token, **expr),
            Expression::Infix(left, token, right) => {
                write!(f, "({} {} {})", **left, token, **right)
            }
        }
    }
}
