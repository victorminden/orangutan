mod environment;
mod built_in_functions;

use std::fmt;
use std::cell::RefCell;
use std::rc::Rc;
pub use self::environment::*;
pub use self::built_in_functions::*;
use crate::ast::BlockStatement;
use crate::evaluator::EvalError;

pub type BuiltInFunction = fn(Vec<Object>) -> Result<Object, EvalError>;
pub type SharedEnvironment = Rc<RefCell<Environment>>;

#[derive(Debug, Clone)]
pub enum Object {
    Null,
    Integer(i64),
    Boolean(bool),
    Str(String),
    Return(Box<Object>),
    Function(Vec<String>, BlockStatement, SharedEnvironment),
    BuiltIn(BuiltInFunction),
    Array(Vec<Object>),
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
            Object::Array(items) => {
                write!(f, "[{}]",
                    items.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", "))
            }
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
