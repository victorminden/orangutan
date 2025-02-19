use super::*;

use crate::lexer::Lexer;
use crate::object::Environment;
use crate::parser::Parser;
use std::cell::RefCell;
use std::rc::Rc;

fn eval_test(input: &str) -> Result<Object, EvalError> {
    let mut parser = Parser::new(Lexer::new(input));
    let env = Rc::new(RefCell::new(Environment::new()));
    match parser.parse_program() {
        Ok(program) => eval(&program, env),
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
fn eval_string_literal_test() {
    let tests = vec![("\"Hello, world!\"", "Hello, world!")];

    for (input, want) in tests {
        let evaluated = eval_test(input);
        match evaluated {
            Ok(Object::Str(got)) => assert_eq!(got, want),
            _ => panic!("Did not get Object::Str!"),
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
fn eval_string_expression_test() {
    let tests = vec![("\"foo\" + \"bar\"", "foobar")];

    for (input, want) in tests {
        let evaluated = eval_test(input);
        match evaluated {
            Ok(Object::Str(got)) => assert_eq!(got, want),
            _ => panic!("Did not get Object::Str!"),
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
        (
            "if (10 > 1) {
            if (10 > 1) {
                return 10;
            }
            return 1;
            }",
            10,
        ),
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
        (
            "5 + true;",
            "EvalError: Type mismatch for infix operator `+`",
        ),
        (
            "5 + true; 5",
            "EvalError: Type mismatch for infix operator `+`",
        ),
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
    let tests = vec![("fn(x) {x+2;}", 1, "x", "{ (x + 2); }")];

    for (input, want_len, want_parameters, want_body) in tests {
        let evaluated = eval_test(input);
        match evaluated {
            Ok(Object::Function(parameters, body, _)) => {
                assert_eq!(parameters.len(), want_len);
                assert_eq!(parameters.join(", "), want_parameters);
                assert_eq!(body.to_string(), want_body);
            }
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
        (
            "let twice = fn(x) { if (x>0) { twice(x-1) } else {12} }; twice(1)",
            12,
        ),
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

#[test]
fn builtin_function_test() {
    let tests = vec![
        ("len(\"\")", 0),
        ("len(\"four\")", 4),
        ("len(\"hello world\")", 11),
        ("len([1, 2, 3+3])", 3),
        ("magic_number(1,2,3)", 42),
        ("first([3, 2, 1])", 3),
        ("first([])", -1),
        ("last([3, 2, 1+5])", 6),
        ("last([])", -1),
    ];

    for (input, want) in tests {
        let evaluated = eval_test(input);
        match evaluated {
            Ok(Object::Integer(got)) => assert_eq!(got, want),
            Ok(Object::Null) => assert_eq!(want, -1),
            _ => panic!("Did not get Object::Integer!"),
        }
    }
}

#[test]
fn rest_test() {
    let tests = vec![("rest([1, 2, 3])", "[2, 3]"), ("rest([])", "")];

    for (input, want) in tests {
        let evaluated = eval_test(input);
        match evaluated {
            Ok(obj) => match obj {
                Object::Array(_) => assert_eq!(obj.to_string(), want),
                Object::Null => assert_eq!(want, ""),
                _ => panic!("Got unexpected object!"),
            },
            _ => panic!("Got error!"),
        }
    }
}

#[test]
fn push_test() {
    let tests = vec![
        ("push([1, 2, 3], 4)", "[1, 2, 3, 4]"),
        ("push([], [])", "[[]]"),
    ];

    for (input, want) in tests {
        let evaluated = eval_test(input);
        match evaluated {
            Ok(obj) => match obj {
                Object::Array(_) => assert_eq!(obj.to_string(), want),
                Object::Null => assert_eq!(want, ""),
                _ => panic!("Got unexpected object!"),
            },
            _ => panic!("Got error!"),
        }
    }
}

#[test]
fn array_test() {
    let tests = vec![("[1, 2*2, 3+3]", "[1, 4, 6]")];

    for (input, want) in tests {
        let evaluated = eval_test(input);
        match evaluated {
            Ok(Object::Array(_)) => {
                if let Ok(obj) = evaluated {
                    assert_eq!(obj.to_string(), want)
                }
            }
            _ => panic!("Did not get Object::Array!"),
        }
    }
}

#[test]
fn hash_test() {
    let tests = vec![("{1: 2*2, \"a\": len(\"bcd\")}", "{\"a\": 3, 1: 4}")];

    for (input, want) in tests {
        let evaluated = eval_test(input);
        match evaluated {
            Ok(Object::Hash(_)) => {
                if let Ok(obj) = evaluated {
                    assert_eq!(obj.to_string(), want)
                }
            }
            _ => panic!("Did not get Object::Hash!"),
        }
    }
}

#[test]
fn array_index_test() {
    let tests = vec![
        ("[1, 2, 3][0]", 1),
        ("[1, 2, 3][1]", 2),
        ("[1, 2, 3][2]", 3),
        ("let i = 0; [1][i]", 1),
        ("[1, 2, 3][1 + 1]", 3),
        (
            "let myArray = [1, 2, 3]; myArray[0] + myArray[1] + myArray[2];",
            6,
        ),
        ("let myArray = [1, 2, 3]; let i = myArray[0]; myArray[i]", 2),
        ("[1, 2, 3][-1]", -1),
    ];

    for (input, want) in tests {
        let evaluated = eval_test(input);
        match evaluated {
            Ok(Object::Integer(got)) => assert_eq!(got, want),
            Ok(Object::Null) => assert_eq!(want, -1),
            _ => panic!("Did not get Object::Integer!"),
        }
    }
}

#[test]
fn hash_index_test() {
    let tests = vec![
        ("{\"foo\": 5}[\"foo\"]", 5),
        ("{\"foo\": 5}[\"bar\"]", -1),
        ("let key = \"foo\"; {\"foo\": 5}[key]", 5),
        ("{}[\"foo\"]", -1),
    ];

    for (input, want) in tests {
        let evaluated = eval_test(input);
        match evaluated {
            Ok(Object::Integer(got)) => assert_eq!(got, want),
            Ok(Object::Null) => assert_eq!(want, -1),
            _ => panic!("Did not get Object::Integer!"),
        }
    }
}

#[test]
fn map_function_test() {
    let input = "
    let map = fn(arr, f) {
        let iter = fn(arr, accumulated) {
            if (len(arr) == 0) {
                return accumulated;
            } else {
                return iter(rest(arr), push(accumulated, f(first(arr))));
            }
        };
        return iter(arr, []);
    };
    let a = [1, 2, 3, 4];
    let double = fn(x) { x * 2 };
    map(a, double);";
    let evaluated = eval_test(input);
    match evaluated {
        Ok(Object::Array(_)) => {
            if let Ok(obj) = evaluated {
                assert_eq!(obj.to_string(), "[2, 4, 6, 8]")
            }
        }
        _ => panic!("Did not get Object::Array!"),
    }
}

#[test]
fn sum_function_test() {
    let input = "
    let reduce = fn(arr, initial, f) {
        let iter = fn(arr, result) {
            if (len(arr) == 0) {
                result
            } else {
                iter(rest(arr), f(result, first(arr)));
            }
        };
        iter(arr, initial);
    };
    let sum = fn(arr) {
        reduce(arr, 0, fn(initial, el) { initial + el });
    };
    sum([1, 2, 3, 4, 5]);";
    let evaluated = eval_test(input);
    match evaluated {
        Ok(Object::Integer(int)) => assert_eq!(int, 15),
        _ => panic!("Did not get Object::Integer!"),
    }
}
