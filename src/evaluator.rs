#[cfg(test)]
mod evaluator_test;

use std::fmt;
use crate::ast::{Program, Statement, BlockStatement, Expression};
use crate::object::{Object, Environment, get_built_in};
use crate::token::Token;

pub enum EvalError {
    UnknownError,
    UnknownPrefixOperator(Token),
    UnknownInfixOperator(Token),
    UnknownIdentifier(String),
    InfixTypeMismatch(Object, Token, Object),
    PrefixTypeMismatch(Token, Object),
    WrongNumberOfArguments(u32, u32),
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EvalError::UnknownPrefixOperator(token) => {
                write!(f, "EvalError: Unknown prefix operator `{}`", token)
            },
            EvalError::UnknownInfixOperator(token) => {
                write!(f, "EvalError: Unknown infix operator `{}`", token)
            },
            EvalError::InfixTypeMismatch(_, token, _) => {
                write!(f, "EvalError: Type mismatch for infix operator `{}`", token)
            },
            EvalError::PrefixTypeMismatch(token, _) => {
                write!(f, "EvalError: Type mismatch for prefix operator `{}`", token)
            },
            EvalError::UnknownIdentifier(name) => {
                write!(f, "EvalError: Unknown identifier `{}`", name)
            },
            EvalError::WrongNumberOfArguments(got, want) => {
                write!(f, "EvalError: Wrong number of parameters (got: {}, want: {}",
                got, want)
            }
            EvalError::UnknownError => write!(f, "EvalError: UnknownError!"),
        }
    }
}

pub fn eval(p: &Program, env: &mut Environment) -> Result<Object, EvalError> {
    let mut result = Object::Null;
    for statement in &p.statements {
        result = eval_statement(statement, env)?;
        if let Object::Return(value) = result {
            return Ok(*value);
        }
    }
    return Ok(result);
}

pub fn eval_block_statement(
    bs: &BlockStatement, env: &mut Environment) -> Result<Object, EvalError> {
    let mut result = Object::Null;
    for statement in &bs.statements {
        result = eval_statement(statement, env)?;
        if let Object::Return(_) = result {
            return Ok(result);
        }
    }
    return Ok(result);
}

fn eval_statement(s: &Statement, env: &mut Environment) -> Result<Object, EvalError> {
    match s {
        Statement::Expression(expr) => eval_expression(&expr, env),
        Statement::Return(expr) => {
            Ok(Object::Return(Box::new(eval_expression(&expr, env)?)))
        },
        Statement::Let(ident, expr) => {
            let result = eval_expression(&expr, env);
            match result {
                Err(_) => result,
                Ok(object) => {
                    env.set(ident, object);
                    Ok(Object::Null)
                }
            }
        },
    }
}

fn eval_expressions(
    exprs: &[Expression], env: &mut Environment) -> Result<Vec<Object>, EvalError> {
    let mut results = vec![];
    for expr in exprs {
        results.push(eval_expression(expr, env)?);
    }
    Ok(results)
}

fn eval_expression(e: &Expression, env: &mut Environment) -> Result<Object, EvalError> {
    match e {
        Expression::IntegerLiteral(value) => Ok(Object::Integer(*value)),
        Expression::StringLiteral(value) => Ok(Object::Str(value.clone())),
        Expression::BooleanLiteral(value) => Ok(Object::Boolean(*value)),
        Expression::Prefix(operator, expr) => {
            eval_prefix_expression(operator, expr, env)
        },
        Expression::Infix(left, operator, right) => {
            eval_infix_expression(left, operator, right, env)
        },
        Expression::If(condition, consequence, alternative) => {
            eval_if_expression(condition, consequence, alternative, env)
        },
        Expression::Ident(name) => eval_identifier(name, env),
        Expression::FunctionLiteral(parameters, body) => {
            Ok(Object::Function(parameters.clone(), body.clone(), env.clone()))
        },
        Expression::Call(expr, arguments) => {
            let function = eval_expression(&**expr, env)?;
            let args = eval_expressions(arguments, env)?;
            apply_function(&function, &args)
        },
        Expression::ArrayLiteral(items) => {
            let elements = eval_expressions(items, env)?;
            Ok(Object::Array(elements))
        },
        Expression::Index(left, right) => {
            let arr = eval_expression(&**left, env)?;
            let idx = eval_expression(&**right, env)?;
            eval_index_expression(&arr, &idx)
        },
    }
}

fn eval_index_expression(array: &Object, index: &Object) -> Result<Object, EvalError> {
    match (&array, &index) {
        (Object::Array(arr), Object::Integer(idx)) => {
            Ok(arr[*idx as usize].clone())
        },
        _ => Err(EvalError::UnknownError),
    }
}

fn eval_identifier(
    name: &String, env: &mut Environment) -> Result<Object, EvalError> {
    if let Some(obj) = env.get(name) {
        return Ok(obj.clone());
    } 
    if let Some(obj) = get_built_in(name) {
        return Ok(obj.clone())
    }
    else {
        Err(EvalError::UnknownIdentifier(name.clone()))
    }
}

fn eval_if_expression(
    condition: &Expression, 
    consequence: &BlockStatement, 
    alternative: &Option<BlockStatement>,
    env: &mut Environment) -> Result<Object, EvalError> {
        if eval_expression(condition, env)?.is_truthy() {
            return eval_block_statement(consequence, env);
        }
        if let Some(bs) = alternative {
            return eval_block_statement(bs, env);
        }
        return Ok(Object::Null);
    }

fn eval_prefix_expression(
    prefix: &Token, right: &Expression, env: &mut Environment) -> Result<Object, EvalError> {
    let obj = eval_expression(right, env)?;
    match prefix {
        Token::Bang => Ok(Object::Boolean(!obj.is_truthy())),
        Token::Minus => {
            // Optional: Could choose to return Null for non-integral type.
            match obj {
                Object::Integer(value) => Ok(Object::Integer(-value)),
                other => Err(EvalError::PrefixTypeMismatch(Token::Minus, other)),
            }
        }
        other => Err(EvalError::UnknownPrefixOperator(other.clone())),
    }
}

fn eval_infix_expression(
    left: &Expression, op: &Token, right: &Expression, env: &mut Environment) -> Result<Object, EvalError> {
    let left_obj = eval_expression(left, env)?;
    let right_obj = eval_expression(right, env)?;

    match (left_obj, right_obj) {
        (Object::Integer(left), Object::Integer(right)) => {
            eval_integer_infix_expression(left, op, right)
        },
        (Object::Boolean(left), Object::Boolean(right)) => {
            eval_boolean_infix_expression(left, op, right)
        },
        (Object::Str(left), Object::Str(right)) => {
            if *op != Token::Plus {
                Err(EvalError::UnknownInfixOperator(op.clone()))
            } else {
                Ok(Object::Str(format!("{}{}", left, right)))
            }
        },
        (a, b)  => Err(EvalError::InfixTypeMismatch(a, op.clone(), b)),
    }
}

fn eval_boolean_infix_expression(
    left: bool, op: &Token, right: bool) -> Result<Object, EvalError> {
        let obj = match op {
            Token::Equal => Object::Boolean(left == right),
            Token::NotEqual => Object::Boolean(left != right),
            other => {
                return Err(EvalError::UnknownInfixOperator(other.clone()));
            },
        };
        Ok(obj)
}

fn eval_integer_infix_expression(
    left: i64, op: &Token, right: i64) -> Result <Object, EvalError> {
    let obj = match op {
        Token::Equal => Object::Boolean(left == right),
        Token::NotEqual => Object::Boolean(left != right),
        Token::LessThan => Object::Boolean(left < right),
        Token::GreaterThan => Object::Boolean(left > right),
        Token::Plus => Object::Integer(left + right),
        Token::Minus => Object::Integer(left - right),
        Token::Asterisk => Object::Integer(left * right),
        Token::Slash => Object::Integer(left / right),
        other => {
            return Err(EvalError::UnknownInfixOperator(other.clone()));
        },
    };
    Ok(obj)
}

fn apply_function(
    function: &Object, args: &Vec<Object>) -> Result<Object, EvalError> {
    match function {
        Object::Function(parameters, body, env) => {
            if parameters.len() != args.len() {
                return Err(EvalError::WrongNumberOfArguments(
                    parameters.len() as u32, args.len() as u32));
            }
            // Build environment for function.
            let mut extended_env = env.clone();
            for (p, a) in parameters.iter().zip(args) {
                extended_env.set(p, a.clone())
            }
            // Evaluate the function with this environment.
            match eval_block_statement(body, &mut extended_env) {
                Ok(Object::Return(value)) => Ok(*value),
                other => other,
            }
        },
        Object::BuiltIn(built_in_function) => {
            // TODO: Remove this clone and figure out references here.
            built_in_function(args.clone())
        },
        // TODO: Make this a more specific error.
        _ => Err(EvalError::UnknownError),
    }
}
