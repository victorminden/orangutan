//! Object
//! 
//! `object` contains types representing evaluated objects from a Monkey program.
//! These types are used while interpreting Monkey programs.
mod environment;
mod built_in_functions;

use std::fmt;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
pub use self::environment::*;
pub use self::built_in_functions::*;
use crate::ast::BlockStatement;
use crate::evaluator::EvalError;
use crate::code::CompiledFunction;

pub type BuiltInFunction = fn(Vec<Object>) -> Result<Object, EvalError>;
pub type SharedEnvironment = Rc<RefCell<Environment>>;

// Represents an object that is of a hashable type.
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum HashableObject {
    Integer(i64),
    Boolean(bool),
    Str(String),
}

impl fmt::Display for HashableObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HashableObject::Str(value) => write!(f, "\"{}\"", value),
            HashableObject::Integer(value) => write!(f, "{}", value),
            HashableObject::Boolean(value) => write!(f, "{}", value),
        }
    }
}

/// Represents any object in the Monkey language after evaluation.
/// These types are specific to the interpreter implementation.
#[derive(Clone)]
pub enum Object {
    Null,
    Integer(i64),
    Boolean(bool),
    Str(String),
    Return(Box<Object>),
    Function(Vec<String>, BlockStatement, SharedEnvironment),
    BuiltIn(BuiltInFunction),
    Array(Vec<Object>),
    Hash(HashMap<HashableObject, Object>),
    CompiledFunction(CompiledFunction),
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
            },
            Object::Hash(elements) => {
                let mut formatted_elements = elements.iter().map(|(x, y)| format!("{}: {}", x.to_string(), y.to_string()))
                    .collect::<Vec<String>>();
                formatted_elements.sort();
                write!(f, "{{{}}}", formatted_elements.join(", "))
                
            },
            Object::CompiledFunction(func) => write!(f, "{}", func),
        }
    }
}

impl Object {
    pub fn is_truthy(&self) -> bool {
        match self {
            Object::Boolean(value) => *value,
            Object::Null => false,
            _ => true,
        }
    }

    pub fn to_hashable_object(self) -> Result<HashableObject, EvalError> {
        match self {
            Object::Boolean(value) => Ok(HashableObject::Boolean(value)),
            Object::Str(value) => Ok(HashableObject::Str(value)),
            Object::Integer(value) => Ok(HashableObject::Integer(value)),
            other => Err(EvalError::HashError(other)),
        }
    }
}
