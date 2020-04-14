use crate::object::Object;
use crate::evaluator::EvalError;

pub fn get_built_in(name: &str) -> Option<&Object> {
    match name {
        "len" => Some(&Object::BuiltIn(len)),
        "first" => Some(&Object::BuiltIn(first)),
        "last" => Some(&Object::BuiltIn(last)),
        "rest" => Some(&Object::BuiltIn(rest)),
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
        },
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
                Ok(arr[ell-1].clone())
            } else {
                Ok(Object::Null)
            }
        },
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
        },
        _ => Err(EvalError::UnsupportedInputToBuiltIn),
    }
}