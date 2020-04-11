use super::*;

use crate::parser::Parser;
use crate::lexer::Lexer;
use crate::object::Environment;

fn eval_test(input: &str) -> Result<Object, EvalError> {
    let mut parser = Parser::new(Lexer::new(input));
    let mut env = Environment::new();
    match parser.parse_program() {
        Ok(program) => eval(&program, &mut env),
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
#[test]
fn errors_test() {
    let tests = vec![
        ("5 + true;", "EvalError: Type mismatch for infix operator `+`"),
        ("5 + true; 5", "EvalError: Type mismatch for infix operator `+`"),
        ("-true;", "EvalError: Type mismatch for prefix operator `-`"),
    ];

    for (input, want) in tests {
        let evaluated = eval_test(input);
        match evaluated {
            Err(got) => assert_eq!(got.to_string(), want),
            _ => panic!("Did not get EvalError!"),
        }
    }
}

#[test]
fn let_statements_test() {
    let tests = vec![
        ("let a = 5; a;", 5),
        ("let a = 5 * 5; a;", 25),
        ("let a = 5; let b = a; b;", 5),
        ("let a = 5; let b = a; let c = a + b + 5; c;", 15),
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
fn function_test() {
    let tests = vec![
        ("fn(x) {x+2;}", 1, "x", "{ (x + 2); }"),
    ];

    for (input, want_len, want_parameters, want_body) in tests {
        let evaluated = eval_test(input);
        match evaluated {
            Ok(Object::Function(parameters, body, _)) => {
                assert_eq!(parameters.len(), want_len);
                assert_eq!(parameters.join(", "), want_parameters);
                assert_eq!(body.to_string(), want_body);
            },
            _ => panic!("Did not get Object::Function!"),
        }
    }
}

#[test]
fn function_application_test() {
    let tests = vec![
        ("let identity = fn(x) { x; }; identity(5);", 5),
        ("let identity = fn(x) { return x; }; identity(5);", 5),
        ("let double = fn(x) { x * 2; }; double(5);", 10),
        ("let add = fn(x, y) { x + y; }; add(5, 5);", 10),
        ("let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));", 20),
        ("fn(x) { x; }(5)", 5),
    ];

    for (input, want) in tests {
        let evaluated = eval_test(input);
        match evaluated {
            Ok(Object::Integer(got)) => assert_eq!(got, want),
            _ => panic!("Did not get Object::Integer!"),
        }
    }
}

