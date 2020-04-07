use std::fmt;
use crate::token::Token;

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Program:")?;
        for statement in &self.statements {
            write!(f, "{}", statement)?;
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

// TODO: BlockStatement type is essentially just Program -- remove?
#[derive(Debug)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}

impl fmt::Display for BlockStatement {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{{ ")?;
    for statement in &self.statements {
        write!(f, "{}", statement)?;
    }
    write!(f, " }}")
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
    If(Box<Expression>, BlockStatement, Option<BlockStatement>),
    FunctionLiteral(Vec<String>, BlockStatement),
    Call(Box<Expression>, Vec<Expression>),
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
            },
            Expression::If(condition, consequence, alternative) => {
                if let Some(alt_bs) = alternative {
                    write!(f, "if {} {} else {}", condition, consequence, alt_bs)
                } else {
                    write!(f, "if {} {}", condition, consequence)
                }
            },
            Expression::FunctionLiteral(parameters, body) => {
                write!(f, "fn({}) {}", parameters.join(", "), body)
            },
            Expression::Call(function, arguments) => {
                // Map the vector of expressions to a vector of strings so we can join them with comma.
                write!(f, "{}({})", function, 
                    arguments.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", "))
            },
        }
    }
}
