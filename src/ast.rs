//! AST
//!
//! `ast` contains types representing the (A)bstract (S)yntax (T)ree of expressions in the Monkey language.
//! These parsed expressions may then be interpreted / compiled / otherwise processed.
use crate::token::Token;
use std::fmt;

/// Represents a full parsed program of Monkey statements.
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

/// Represents a statement in the Monkey language.
///
/// There are only a small number of distinct variants due to the simplicity of the language.
#[derive(Debug, Clone)]
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

/// Represents a grouped sequence of statements in the Monkey language.
// TODO: BlockStatement type is essentially just Program -- remove?
#[derive(Debug, Clone)]
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

/// Represents a parsed expression in the Monkey language.
#[derive(Debug, Clone)]
pub enum Expression {
    Ident(String),
    IntegerLiteral(i64),
    BooleanLiteral(bool),
    StringLiteral(String),
    Prefix(Token, Box<Expression>),
    Infix(Box<Expression>, Token, Box<Expression>),
    If(Box<Expression>, BlockStatement, Option<BlockStatement>),
    FunctionLiteral(Vec<String>, BlockStatement, Option<String>),
    Call(Box<Expression>, Vec<Expression>),
    ArrayLiteral(Vec<Expression>),
    Index(Box<Expression>, Box<Expression>),
    HashLiteral(Vec<(Expression, Expression)>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Ident(ident) => write!(f, "{}", ident),
            Expression::IntegerLiteral(i) => write!(f, "{}", i),
            Expression::BooleanLiteral(b) => write!(f, "{}", b),
            Expression::StringLiteral(s) => write!(f, "\"{}\"", s),
            Expression::Prefix(token, expr) => write!(f, "({}{})", token, **expr),
            Expression::Infix(left, token, right) => {
                write!(f, "({} {} {})", **left, token, **right)
            }
            Expression::If(condition, consequence, alternative) => {
                if let Some(alt_bs) = alternative {
                    write!(f, "if {} {} else {}", condition, consequence, alt_bs)
                } else {
                    write!(f, "if {} {}", condition, consequence)
                }
            }
            Expression::FunctionLiteral(parameters, body, _) => {
                write!(f, "fn({}) {}", parameters.join(", "), body)
            }
            Expression::Call(function, arguments) => {
                // Map the vector of expressions to a vector of strings so we can join them with comma.
                write!(
                    f,
                    "{}({})",
                    function,
                    arguments
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            Expression::ArrayLiteral(elements) => write!(
                f,
                "[{}]",
                elements
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Expression::HashLiteral(elements) => write!(
                f,
                "{{{}}}",
                elements
                    .iter()
                    .map(|(x, y)| format!("{}: {}", x.to_string(), y.to_string()))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Expression::Index(arr, idx) => write!(f, "({}[{}])", arr, idx),
        }
    }
}
