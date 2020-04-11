use std::collections::HashMap;
use crate::object::Object;
use crate::evaluator::EvalError;

#[derive(Debug, Default, Clone)]
pub struct Environment {
    store: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get(&self, name: &str) -> Option<&Object> {
        self.store.get(name)
    }

    pub fn set(&mut self, name: &str, val: Object) {
        self.store.insert(name.to_string(), val);
    }
}

pub fn get_built_in(name: &str) -> Option<&Object> {
    match name {
        "len" => Some(&Object::BuiltIn(len)),
        "magic_number" => Some(&Object::BuiltIn(magic_number)),
        _ => None,
    }
}

fn magic_number(_: Vec<Object>) -> Result<Object, EvalError> {
    // Doesn't care about parameters, just returns 42.
    Ok(Object::Integer(42))
}

fn len(params: Vec<Object>) -> Result<Object, EvalError> {
    if params.len() != 1 {
        return Err(EvalError::WrongNumberOfArguments(params.len() as u32, 1));
    }
    match &params[0] {
        Object::Str(string) => Ok(Object::Integer(string.len() as i64)),
        _ => Err(EvalError::UnknownError),
    }
}