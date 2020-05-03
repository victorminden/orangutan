//! Evaluator
//!
//! `evaluator` contains functions for evaluating parsed expressions in the Monkey language.
//! The public interface is simply the `eval` function.
mod eval_error;
#[cfg(test)]
mod evaluator_test;
pub use self::eval_error::EvalError;
use crate::ast::{BlockStatement, Expression, Program, Statement};
use crate::object::{get_built_in, Object, SharedEnvironment};
use crate::token::Token;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Returns the result of evaluating the input program.
///
/// The input `p` is the primary input consisting of the abstract syntax tree of a Monkey program.
/// The input `env` contains any saved state (environment variables) to be used, and may be modified.
pub fn eval(p: &Program, env: SharedEnvironment) -> Result<Object, EvalError> {
    let mut result = Object::Null;
    for statement in &p.statements {
        result = eval_statement(statement, Rc::clone(&env))?;
        if let Object::Return(value) = result {
            // We *do* unwrap the returned object from its `Return`.
            return Ok(*value);
        }
    }
    return Ok(result);
}

// TODO: This function could be merged with `eval` if we merge the `BlockStatement` and `Program` types.
fn eval_block_statement(bs: &BlockStatement, env: SharedEnvironment) -> Result<Object, EvalError> {
    let mut result = Object::Null;
    for statement in &bs.statements {
        result = eval_statement(statement, Rc::clone(&env))?;
        if let Object::Return(_) = result {
            // We do *not* unwrap the returned object from its `Return`.
            return Ok(result);
        }
    }
    return Ok(result);
}

fn eval_statement(s: &Statement, env: SharedEnvironment) -> Result<Object, EvalError> {
    match s {
        Statement::Expression(expr) => eval_expression(&expr, env),
        Statement::Return(expr) => Ok(Object::Return(Box::new(eval_expression(&expr, env)?))),
        Statement::Let(ident, expr) => {
            let result = eval_expression(&expr, Rc::clone(&env));
            match result {
                Err(_) => result,
                Ok(object) => {
                    // Ugly, unsafe Rust, what to do?
                    env.borrow_mut().set(ident, object);
                    Ok(Object::Null)
                }
            }
        }
    }
}

fn eval_expressions(
    exprs: &[Expression],
    env: SharedEnvironment,
) -> Result<Vec<Object>, EvalError> {
    let mut results = vec![];
    for expr in exprs {
        results.push(eval_expression(expr, Rc::clone(&env))?);
    }
    Ok(results)
}

fn eval_expression(e: &Expression, env: SharedEnvironment) -> Result<Object, EvalError> {
    match e {
        Expression::IntegerLiteral(value) => Ok(Object::Integer(*value)),
        Expression::StringLiteral(value) => Ok(Object::Str(value.clone())),
        Expression::BooleanLiteral(value) => Ok(Object::Boolean(*value)),
        Expression::Prefix(operator, expr) => eval_prefix_expression(operator, expr, env),
        Expression::Infix(left, operator, right) => {
            eval_infix_expression(left, operator, right, env)
        }
        Expression::If(condition, consequence, alternative) => {
            eval_if_expression(condition, consequence, alternative, env)
        }
        Expression::Ident(name) => eval_identifier(name, env),
        Expression::FunctionLiteral(parameters, body, _) => Ok(Object::Function(
            parameters.clone(),
            body.clone(),
            env.clone(),
        )),
        Expression::Call(expr, arguments) => {
            let function = eval_expression(&**expr, Rc::clone(&env))?;
            let args = eval_expressions(arguments, env)?;
            apply_function(&function, &args)
        }
        Expression::ArrayLiteral(items) => {
            let elements = eval_expressions(items, env)?;
            Ok(Object::Array(elements))
        }
        Expression::Index(left, right) => {
            let obj = eval_expression(&**left, Rc::clone(&env))?;
            let idx = eval_expression(&**right, env)?;
            eval_index_expression(&obj, &idx)
        }
        Expression::HashLiteral(items) => {
            let mut hash = HashMap::new();
            for (key, value) in items.iter() {
                let evaluated_key = eval_expression(&key, Rc::clone(&env))?;
                let evaluated_value = eval_expression(&value, Rc::clone(&env))?;
                hash.insert(evaluated_key.to_hashable_object()?, evaluated_value);
            }
            Ok(Object::Hash(hash))
        }
    }
}

fn eval_index_expression(obj: &Object, index: &Object) -> Result<Object, EvalError> {
    match (&obj, &index) {
        (Object::Array(arr), Object::Integer(idx)) => match arr.get(*idx as usize) {
            Some(obj) => Ok(obj.clone()),
            None => Ok(Object::Null),
        },
        (Object::Hash(items), _) => {
            let key = index.clone().to_hashable_object()?;
            match items.get(&key) {
                Some(result) => Ok(result.clone()),
                None => Ok(Object::Null),
            }
        }
        _ => Err(EvalError::UnknownError),
    }
}

fn eval_identifier(name: &String, env: SharedEnvironment) -> Result<Object, EvalError> {
    if let Some(obj) = env.borrow().get(name) {
        return Ok(obj.clone());
    }
    if let Some(obj) = get_built_in(name) {
        return Ok(obj.clone());
    } else {
        Err(EvalError::UnknownIdentifier(name.clone()))
    }
}

fn eval_if_expression(
    condition: &Expression,
    consequence: &BlockStatement,
    alternative: &Option<BlockStatement>,
    env: SharedEnvironment,
) -> Result<Object, EvalError> {
    if eval_expression(condition, Rc::clone(&env))?.is_truthy() {
        return eval_block_statement(consequence, env);
    }
    if let Some(bs) = alternative {
        return eval_block_statement(bs, env);
    }
    return Ok(Object::Null);
}

fn eval_prefix_expression(
    prefix: &Token,
    right: &Expression,
    env: SharedEnvironment,
) -> Result<Object, EvalError> {
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
    left: &Expression,
    op: &Token,
    right: &Expression,
    env: SharedEnvironment,
) -> Result<Object, EvalError> {
    let left_obj = eval_expression(left, Rc::clone(&env))?;
    let right_obj = eval_expression(right, Rc::clone(&env))?;

    match (left_obj, right_obj) {
        (Object::Integer(left), Object::Integer(right)) => {
            eval_integer_infix_expression(left, op, right)
        }
        (Object::Boolean(left), Object::Boolean(right)) => {
            eval_boolean_infix_expression(left, op, right)
        }
        (Object::Str(left), Object::Str(right)) => {
            if *op != Token::Plus {
                Err(EvalError::UnknownInfixOperator(op.clone()))
            } else {
                Ok(Object::Str(format!("{}{}", left, right)))
            }
        }
        (a, b) => Err(EvalError::InfixTypeMismatch(a, op.clone(), b)),
    }
}

fn eval_boolean_infix_expression(left: bool, op: &Token, right: bool) -> Result<Object, EvalError> {
    let obj = match op {
        Token::Equal => Object::Boolean(left == right),
        Token::NotEqual => Object::Boolean(left != right),
        other => {
            return Err(EvalError::UnknownInfixOperator(other.clone()));
        }
    };
    Ok(obj)
}

fn eval_integer_infix_expression(left: i64, op: &Token, right: i64) -> Result<Object, EvalError> {
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
        }
    };
    Ok(obj)
}

fn apply_function(function: &Object, args: &Vec<Object>) -> Result<Object, EvalError> {
    match function {
        Object::Function(parameters, body, env) => {
            if parameters.len() != args.len() {
                return Err(EvalError::WrongNumberOfArguments(
                    parameters.len() as u32,
                    args.len() as u32,
                ));
            }
            // Build environment for function.
            let extended_env = Rc::new(RefCell::new(env.borrow().clone()));
            for (p, a) in parameters.iter().zip(args) {
                extended_env.borrow_mut().set(p, a.clone())
            }
            // Evaluate the function with this environment.
            match eval_block_statement(body, Rc::clone(&extended_env)) {
                Ok(Object::Return(value)) => Ok(*value),
                other => other,
            }
        }
        Object::BuiltIn(built_in_function) => {
            // TODO: Remove this clone and figure out references here.
            built_in_function(args.clone())
        }
        // TODO: Make this a more specific error.
        _ => Err(EvalError::UnknownError),
    }
}
