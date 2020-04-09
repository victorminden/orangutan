use crate::ast::{Program, Statement, BlockStatement, Expression};
use crate::object::Object;
use crate::token::Token;

pub enum EvalError {
    UnknownError,
    UnknownPrefixOperator(Token),
    UnknownInfixOperator(Token),
    InfixTypeMismatch(Object, Token, Object),
    PrefixTypeMismatch(Token, Object),
}

pub fn eval(p: &Program) -> Result<Object, EvalError> {
    let mut result = Object::Null;
    for statement in &p.statements {
        result = eval_statement(statement)?;
        if let Object::Return(value) = result {
            return Ok(*value);
        }
    }
    return Ok(result);
}

pub fn eval_block_statement(bs: &BlockStatement) -> Result<Object, EvalError> {
    let mut result = Object::Null;
    for statement in &bs.statements {
        result = eval_statement(statement)?;
        if let Object::Return(_) = result {
            return Ok(result);
        }
    }
    return Ok(result);
}

fn eval_statement(s: &Statement) -> Result<Object, EvalError> {
    match s {
        Statement::Expression(expr) => eval_expression(&expr),
        Statement::Return(expr) => {
            Ok(Object::Return(Box::new(eval_expression(&expr)?)))
        },
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
        Expression::Infix(left, operator, right) => {
            eval_infix_expression(left, operator, right)
        },
        Expression::If(condition, consequence, alternative) => {
            eval_if_expression(condition, consequence, alternative)
        },
        _ => Err(EvalError::UnknownError),
    }
}

fn eval_if_expression(
    condition: &Expression, 
    consequence: &BlockStatement, 
    alternative: &Option<BlockStatement>) -> Result<Object, EvalError> {
        if eval_expression(condition)?.is_truthy() {
            return eval_block_statement(consequence);
        }
        if let Some(bs) = alternative {
            return eval_block_statement(bs);
        }
        return Ok(Object::Null);
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
                other => Err(EvalError::PrefixTypeMismatch(Token::Minus, other)),
            }
        }
        other => Err(EvalError::UnknownPrefixOperator(other.clone())),
    }
}

fn eval_infix_expression(
    left: &Expression, op: &Token, right: &Expression) -> Result<Object, EvalError> {
    let left_obj = eval_expression(left)?;
    let right_obj = eval_expression(right)?;

    match (left_obj, right_obj) {
        (Object::Integer(left), Object::Integer(right)) => {
            eval_integer_infix_expression(left, op, right)
        },
        (Object::Boolean(left), Object::Boolean(right)) => {
            eval_boolean_infix_expression(left, op, right)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;
    use crate::lexer::Lexer;

    fn eval_test(input: &str) -> Result<Object, EvalError> {
        let mut parser = Parser::new(Lexer::new(input));
        
        match parser.parse_program() {
            Ok(program) => eval(&program),
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
            ("5 + 5 + 5 + 5 - 10", 10),
            ("2 * 2 * 2 * 2 * 2", 32),
            ("-50 + 100 + -50", 0),
            ("5 * 2 + 10", 20),
            ("5 + 2 * 10", 25),
            ("20 + 2 * -10", 0),
            ("50 / 2 * 2 + 10", 60),
            ("2 * (5 + 10)", 30),
            ("3 * 3 * 3 + 10", 37),
            ("3 * (3 * 3) + 10", 37),
            ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
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
            ("true == true", true),
            ("true == false", false),
            ("true != true", false),
            ("true != false", true),
            ("(1<2) == true", true),
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
            ("5 < 3", false),
            ("5 == 5", true),
            ("1 > 2", false),
            ("1 != 1", false),
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
    fn if_else_expression_test() {
        // Use -1 as a placeholder to indicate a Null return.
        let tests = vec![
            ("if (true) { 10 }", 10),
            ("if (false) { 10 }", -1),
            ("if (1) { 10 }", 10),
            ("if (1 < 2) { 10 }", 10),
            ("if (1 > 2) { 10 }", -1),
            ("if (1 > 2) { 10 } else { 20 }", 20),
            ("if (1 < 2) { 10 } else { 20 }", 10),
        ];
    
        for (input, want) in tests {
            let evaluated = eval_test(input);
            match evaluated {
                Ok(Object::Integer(got)) => assert_eq!(got, want),
                Ok(Object::Null) => assert_eq!(want, -1),
                _ => panic!("Did not get Object::Integer or Object::Null!"),
            }
        }
    }

    #[test]
    fn return_test() {
        let tests = vec![
            ("return 10;", 10),
            ("return 10; 9;", 10),
            ("return 2 * 5; 9;", 10),
            ("9; return 2 * 5; 9;", 10),
            ("if (10 > 1) {
                if (10 > 1) {
                  return 10;
                }
              return 1;
              }", 10),
        ];
    
        for (input, want) in tests {
            let evaluated = eval_test(input);
            match evaluated {
                Ok(Object::Integer(got)) => assert_eq!(got, want),
                _ => panic!("Did not get Object::Integer!"),
            }
        }
    }
}