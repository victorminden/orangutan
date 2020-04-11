mod environment;

use std::fmt;
pub use self::environment::*;
use crate::ast::BlockStatement;
use crate::evaluator::EvalError;

pub type BuiltInFunction = fn(Vec<Object>) -> Result<Object, EvalError>;
#[derive(Debug, Clone)]
pub enum Object {
    Null,
    Integer(i64),
    Boolean(bool),
    Str(String),
    Return(Box<Object>),
    Function(Vec<String>, BlockStatement, Environment),
    BuiltIn(BuiltInFunction),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Null => write!(f, "null"),
            Object::Str(value) => write!(f, "\"{}\"", value),
            Object::Integer(value) => write!(f, "{}", value),
            Object::Boolean(value) => write!(f, "{}", value),
            Object::Return(boxed_object) => write!(f, "{}", **boxed_object),
            Object::Function(parameters, body, _) => {
                write!(f, "fn({}) {}", parameters.join(", "), body)
            },
            Object::BuiltIn(_) => write!(f, "Built-In function"),
        }
    }
}

impl Object {
    pub fn is_truthy(self) -> bool {
        match self {
            Object::Boolean(value) => value,
            Object::Null => false,
            _ => true,
        }
    }
}
