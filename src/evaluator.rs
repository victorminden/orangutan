use crate::ast::{Program, Statement, Expression};
use crate::object::Object;
use crate::token::Token;

pub enum EvalError {
    UnknownError,
    UnknownPrefixOperator(Token, Object),
}

pub fn eval(p: Program) -> Result<Object, EvalError> {
    let mut result = Object::Null;
    for statement in p.statements {
        result = eval_statement(statement)?;
        if let Object::Return(value) = result {
            return Ok(*value);
        }
    }
    return Ok(result);
}

fn eval_statement(s: Statement) -> Result<Object, EvalError> {
    match s {
        Statement::Expression(expr) => eval_expression(&expr),
        _ => Err(EvalError::UnknownError),
    }
}

fn eval_expression(e: &Expression) -> Result<Object, EvalError> {
    match e {
        Expression::IntegerLiteral(value) => Ok(Object::Integer(*value)),
        Expression::BooleanLiteral(value) => Ok(Object::Boolean(*value)),
        Expression::Prefix(operator, expr) => {
            eval_prefix_expression(operator, expr)
        },
        _ => Err(EvalError::UnknownError),
    }
}

fn eval_prefix_expression(
    prefix: &Token, right: &Expression) -> Result<Object, EvalError> {
    let obj = eval_expression(right)?;
    match prefix {
        Token::Bang => Ok(Object::Boolean(!obj.is_truthy())),
        Token::Minus => {
            // Optional: Could choose to return Null for non-integral type.
            match obj {
                Object::Integer(value) => Ok(Object::Integer(-value)),
                other => Err(EvalError::UnknownPrefixOperator(Token::Minus, other)),
            }
        }
        other => Err(EvalError::UnknownPrefixOperator(other.clone(), obj)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;
    use crate::lexer::Lexer;

    fn eval_test(input: &str) -> Result<Object, EvalError> {
        let mut parser = Parser::new(Lexer::new(input));
        
        match parser.parse_program() {
            Ok(program) => eval(program),
            _ => panic!("Input could not be parsed!"),
        }
    }

    #[test]
    fn eval_integer_expression_test() {
        let tests = vec![
            ("5", 5),
            ("10", 10),
            ("-5", -5),
            ("-10", -10),
        ];
    
        for (input, want) in tests {
            let evaluated = eval_test(input);
            match evaluated {
                Ok(Object::Integer(got)) => assert_eq!(got, want),
                _ => panic!("Did not get Object::Integer!"),
            }
        }
    }

    #[test]
    fn eval_boolean_expression_test() {
        let tests = vec![
            ("true", true),
            ("false", false),
        ];
    
        for (input, want) in tests {
            let evaluated = eval_test(input);
            match evaluated {
                Ok(Object::Boolean(got)) => assert_eq!(got, want),
                _ => panic!("Did not get Object::Boolean!"),
            }
        }
    }

    #[test]
    fn bang_operator_test() {
        let tests = vec![
            ("!true", false),
            ("!false", true),
            ("!!true", true),
            ("!!false", false),
            ("!5", false),
        ];
    
        for (input, want) in tests {
            let evaluated = eval_test(input);
            match evaluated {
                Ok(Object::Boolean(got)) => assert_eq!(got, want),
                _ => panic!("Did not get Object::Boolean!"),
            }
        }
    }
}