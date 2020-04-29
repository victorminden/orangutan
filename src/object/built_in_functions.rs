//! BuiltInFunctions
//!
//! `built_in_functions` contains the implementation of functions built-in to the Monkey language.
use crate::evaluator::EvalError;
use crate::object::Object;

// TODO: Document.

pub enum BuiltIn {
    Len,
    First,
    Last,
    Rest,
    Push,
    Puts,
    MagicNumber,
}

impl BuiltIn {
    pub fn all() -> Vec<BuiltIn> {
        vec![
            BuiltIn::Len,
            BuiltIn::First,
            BuiltIn::Last,
            BuiltIn::Rest,
            BuiltIn::Push,
            BuiltIn::Puts,
            BuiltIn::MagicNumber,
        ]
    }

    pub fn name(&self) -> String {
        let raw = match self {
            BuiltIn::Len => "len",
            BuiltIn::First => "first",
            BuiltIn::Last => "last",
            BuiltIn::Rest => "rest",
            BuiltIn::Push => "push",
            BuiltIn::Puts => "puts",
            BuiltIn::MagicNumber => "magic_number",
        };
        String::from(raw)
    }

    pub fn func(&self) -> Object {
        let f = match self {
            BuiltIn::Len => len,
            BuiltIn::First => first,
            BuiltIn::Last => last,
            BuiltIn::Rest => rest,
            BuiltIn::Push => push,
            BuiltIn::Puts => puts,
            BuiltIn::MagicNumber => magic_number,
        };
        Object::BuiltIn(f)
    }
}

pub fn get_built_in(name: &str) -> Option<Object> {
    for b in &BuiltIn::all() {
        if name == b.name() {
            return Some(b.func());
        }
    }
    return None;
}

fn magic_number(_: Vec<Object>) -> Result<Object, EvalError> {
    // Doesn't care about parameters, just returns 42.
    Ok(Object::Integer(42))
}

fn puts(params: Vec<Object>) -> Result<Object, EvalError> {
    for param in &params {
        match param {
            // We do a silly match on the string to remove quotes from result.
            Object::Str(string) => {
                println!("{}", string);
            }
            _ => {
                println!("{}", param);
            }
        };
    }
    Ok(Object::Null)
}

fn len(params: Vec<Object>) -> Result<Object, EvalError> {
    if params.len() != 1 {
        return Err(EvalError::WrongNumberOfArguments(params.len() as u32, 1));
    }
    match &params[0] {
        Object::Str(string) => Ok(Object::Integer(string.len() as i64)),
        Object::Array(arr) => Ok(Object::Integer(arr.len() as i64)),
        _ => Err(EvalError::UnsupportedInputToBuiltIn),
    }
}

fn first(params: Vec<Object>) -> Result<Object, EvalError> {
    if params.len() != 1 {
        return Err(EvalError::WrongNumberOfArguments(params.len() as u32, 1));
    }
    match &params[0] {
        Object::Array(arr) => {
            if arr.len() > 0 {
                Ok(arr[0].clone())
            } else {
                Ok(Object::Null)
            }
        }
        _ => Err(EvalError::UnsupportedInputToBuiltIn),
    }
}

fn last(params: Vec<Object>) -> Result<Object, EvalError> {
    if params.len() != 1 {
        return Err(EvalError::WrongNumberOfArguments(params.len() as u32, 1));
    }
    match &params[0] {
        Object::Array(arr) => {
            let ell = arr.len();
            if ell > 0 {
                Ok(arr[ell - 1].clone())
            } else {
                Ok(Object::Null)
            }
        }
        _ => Err(EvalError::UnsupportedInputToBuiltIn),
    }
}

fn rest(params: Vec<Object>) -> Result<Object, EvalError> {
    if params.len() != 1 {
        return Err(EvalError::WrongNumberOfArguments(params.len() as u32, 1));
    }
    match &params[0] {
        Object::Array(arr) => {
            let ell = arr.len();
            if ell > 0 {
                let mut out = arr.clone();
                out.remove(0);
                Ok(Object::Array(out))
            } else {
                Ok(Object::Null)
            }
        }
        _ => Err(EvalError::UnsupportedInputToBuiltIn),
    }
}

fn push(params: Vec<Object>) -> Result<Object, EvalError> {
    if params.len() != 2 {
        return Err(EvalError::WrongNumberOfArguments(params.len() as u32, 2));
    }
    match &params[0] {
        Object::Array(arr) => {
            let mut new_arr = arr.clone();
            new_arr.push(params[1].clone());
            Ok(Object::Array(new_arr))
        }
        _ => Err(EvalError::UnsupportedInputToBuiltIn),
    }
}
