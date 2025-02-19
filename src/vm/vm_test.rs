use super::*;

use crate::compiler::Compiler;
use crate::lexer::Lexer;
use crate::object::Object;
use crate::parser::Parser;

fn run(input: &str) -> Result<Object, VmError> {
    let mut p = Parser::new(Lexer::new(input));
    let program = p.parse_program().unwrap();
    let mut compiler = Compiler::new();
    match compiler.compile(&program) {
        Ok(bytecode) => Vm::new(&bytecode).run(),
        _ => panic!("Compilation error of some sort!"),
    }
}

#[test]
fn integer_arithmetic_test() {
    let tests = vec![
        ("1", 1),
        ("2", 2),
        ("1 + 2", 3),
        ("1 - 2", -1),
        ("1 * 2", 2),
        ("4 / 2", 2),
        ("50 / 2 * 2 + 10 - 5", 55),
        ("5 + 5 + 5 + 5 - 10", 10),
        ("2 * 2 * 2 * 2 * 2", 32),
        ("5 * 2 + 10", 20),
        ("5 + 2 * 10", 25),
        ("5 * (2 + 10)", 60),
        ("-5", -5),
        ("-10", -10),
        ("-50 + 100 + -50", 0),
        ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
    ];
    for (test_input, expected) in tests {
        match run(test_input) {
            Ok(obj) => assert_eq!(obj.to_string(), expected.to_string()),
            _ => panic!("VM error!"),
        }
    }
}

#[test]
fn boolean_expression_test() {
    let tests = vec![
        ("true", true),
        ("false", false),
        ("1 < 2", true),
        ("1 > 2", false),
        ("1 < 1", false),
        ("1 > 1", false),
        ("1 == 1", true),
        ("1 != 1", false),
        ("1 == 2", false),
        ("1 != 2", true),
        ("true == true", true),
        ("false == false", true),
        ("true == false", false),
        ("true != false", true),
        ("false != true", true),
        ("(1 < 2) == true", true),
        ("(1 < 2) == false", false),
        ("(1 > 2) == true", false),
        ("(1 > 2) == false", true),
        ("!true", false),
        ("!false", true),
        ("!5", false),
        ("!!true", true),
        ("!!false", false),
        ("!!5", true),
        ("!(if (false) { 5; })", true),
    ];
    for (test_input, expected) in tests {
        match run(test_input) {
            Ok(obj) => assert_eq!(
                obj.to_string(),
                expected.to_string(),
                "Wrong output on input \"{}\"!",
                test_input
            ),
            _ => panic!(format!("VM error on input {}!", test_input)),
        }
    }
}

#[test]
fn conditional_test() {
    let tests = vec![
        ("if (true) { 10 }", 10),
        ("if (true) { 10 } else { 20 }", 10),
        ("if (false) { 10 } else { 20 } ", 20),
        ("if (1) { 10 }", 10),
        ("if (1 < 2) { 10 }", 10),
        ("if (1 < 2) { 10 } else { 20 }", 10),
        ("if (1 > 2) { 10 } else { 20 }", 20),
        ("if (1 > 2) { 10 }", -1),
        ("if (false) { 10 }", -1),
        ("if ((if (false) { 10 })) { 10 } else { 20 }", 20),
    ];
    for (test_input, expected) in tests {
        match run(test_input) {
            Ok(Object::Null) => {
                assert_eq!(-1, expected, "Wrong output on input \"{}\"!", test_input)
            }
            Ok(obj) => assert_eq!(
                obj.to_string(),
                expected.to_string(),
                "Wrong output on input \"{}\"!",
                test_input
            ),
            _ => panic!(format!("VM error on input \"{}\"!", test_input)),
        }
    }
}

#[test]
fn global_let_test() {
    let tests = vec![
        ("let one = 1; one", 1),
        ("let one = 1; let two = 2; one + two", 3),
        ("let one = 1; let two = one + one; one + two", 3),
    ];
    for (test_input, expected) in tests {
        match run(test_input) {
            Ok(Object::Null) => {
                assert_eq!(-1, expected, "Wrong output on input \"{}\"!", test_input)
            }
            Ok(obj) => assert_eq!(
                obj.to_string(),
                expected.to_string(),
                "Wrong output on input \"{}\"!",
                test_input
            ),
            _ => panic!(format!("VM error on input \"{}\"!", test_input)),
        }
    }
}

#[test]
fn string_expression_test() {
    let tests = vec![
        ("\"monkey\"", "monkey"),
        ("\"mon\" + \"key\"", "monkey"),
        ("\"mon\" + \"key\" + \"banana\"", "monkeybanana"),
    ];
    for (test_input, expected) in tests {
        match run(test_input) {
            Ok(Object::Str(string)) => assert_eq!(string.to_string(), expected.to_string()),
            _ => panic!("VM error!"),
        }
    }
}

#[test]
fn array_literal_test() {
    let tests = vec![
        ("[]", "[]"),
        ("[1,2,3]", "[1, 2, 3]"),
        ("[1+2,3+4,5*6]", "[3, 7, 30]"),
    ];
    for (test_input, expected) in tests {
        if let Ok(obj) = run(test_input) {
            assert_eq!(obj.to_string(), expected.to_string())
        } else {
            panic!("VM error!");
        }
    }
}

#[test]
fn hash_literal_test() {
    let tests = vec![
        ("{}", "{}"),
        ("{1: 2, 3: 4}", "{1: 2, 3: 4}"),
        ("{1+1: 2+2, 3*3: 4*4}", "{2: 4, 9: 16}"),
    ];
    for (test_input, expected) in tests {
        if let Ok(obj) = run(test_input) {
            assert_eq!(obj.to_string(), expected.to_string())
        } else {
            panic!("VM error!");
        }
    }
}

#[test]
fn index_test() {
    let tests = vec![
        ("[1, 2, 3][1]", "2"),
        ("[1, 2, 3][0 + 2]", "3"),
        ("[[1, 1, 1]][0][0]", "1"),
        ("[][0]", "null"),
        ("[1, 2, 3][99]", "null"),
        ("[1][-1]", "null"),
        ("{1: 1, 2: 2}[1]", "1"),
        ("{1: 1, 2: 2}[2]", "2"),
        ("{1: 1}[0]", "null"),
        ("{}[0]", "null"),
    ];
    for (test_input, expected) in tests {
        match run(test_input) {
            Ok(obj) => assert_eq!(obj.to_string(), expected.to_string()),
            Err(error) => panic!("VM error! {:?}", error),
        }
    }
}

#[test]
fn no_args_function_call_test() {
    let tests = vec![
        ("fn() {5 + 11}()", "16"),
        (
            "let fivePlusTen = fn() { 5 + 10 };
        fivePlusTen();",
            "15",
        ),
        (
            "let noReturn = fn() { };
        noReturn();",
            "null",
        ),
        (
            "let noReturn = fn() { };
        let noReturnTwo = fn() { noReturn(); };
        noReturn();
        noReturnTwo();",
            "null",
        ),
        (
            "let returnsOne = fn() { 1; };
        let returnsOneReturner = fn() { returnsOne; };
        returnsOneReturner()();",
            "1",
        ),
        (
            "let returnsOneReturner = fn() {
            let returnsOne = fn() { 1; };
            returnsOne;
            };
            returnsOneReturner()();",
            "1",
        ),
    ];
    for (test_input, expected) in tests {
        match run(test_input) {
            Ok(obj) => assert_eq!(obj.to_string(), expected.to_string()),
            Err(error) => panic!("VM error on input {}! {:?}", test_input, error),
        }
    }
}

#[test]
fn calling_functions_with_bindings_test() {
    let tests = vec![
        ("let one = fn() { let one = 1; one }; one();", 1),
        (
            "let oneAndTwo = fn() { let one = 1; let two = 2; one + two; };
        oneAndTwo();",
            3,
        ),
        (
            "let oneAndTwo = fn() { let one = 1; let two = 2; one + two; };
        let threeAndFour = fn() { let three = 3; let four = 4; three + four; };
        oneAndTwo() + threeAndFour();",
            10,
        ),
        (
            "let firstFoobar = fn() { let foobar = 50; foobar; };
        let secondFoobar = fn() { let foobar = 100; foobar; };
        firstFoobar() + secondFoobar();",
            150,
        ),
        (
            "let globalSeed = 50;
        let minusOne = fn() {
        let num = 1;
        globalSeed - num;
        };
        let minusTwo = fn() {
        let num = 2;
        globalSeed - num;
        };
        minusOne() + minusTwo();",
            97,
        ),
    ];
    for (test_input, expected) in tests {
        match run(test_input) {
            Ok(obj) => assert_eq!(obj.to_string(), expected.to_string()),
            Err(error) => panic!("VM error! {:?}", error),
        }
    }
}

#[test]
fn calling_functions_with_arguments_and_bindings_test() {
    let tests = vec![
        (
            "let identity = fn(a) { a; };
        identity(4);",
            4,
        ),
        (
            "let sum = fn(a, b) { a + b; };
            sum(1, 2);",
            3,
        ),
    ];
    for (test_input, expected) in tests {
        match run(test_input) {
            Ok(obj) => assert_eq!(obj.to_string(), expected.to_string()),
            Err(error) => panic!("VM error! {:?}", error),
        }
    }
}

#[test]
fn builtin_functions_test() {
    let tests = vec![
        ("len(\"\")", 0),
        ("len(\"four\")", 4),
        ("let array = [1,2,3]; first(rest(array))", 2),
    ];
    for (test_input, expected) in tests {
        match run(test_input) {
            Ok(obj) => assert_eq!(obj.to_string(), expected.to_string()),
            Err(error) => panic!("VM error! {:?}", error),
        }
    }
}

#[test]
fn closures_test() {
    let tests = vec![
        (
            "let newClosure = fn(a) {
            fn() { a; };
            };
            let closure = newClosure(99);
            closure();",
            99,
        ),
        (
            "let newAdder = fn(a, b) {
        fn(c) { a + b + c };
        };
        let adder = newAdder(1, 2);
        adder(8);",
            11,
        ),
        (
            "let newAdder = fn(a, b) {
            let c = a + b;
            fn(d) { c + d };
            };
            let adder = newAdder(1, 2);
            adder(8);",
            11,
        ),
        (
            "let newAdderOuter = fn(a, b) {
            let c = a + b;
            fn(d) {
            let e = d + c;
            fn(f) { e + f; };
            };
            };
            let newAdderInner = newAdderOuter(1, 2);
            let adder = newAdderInner(3);
            adder(8);",
            14,
        ),
        (
            "let a = 1;
            let newAdderOuter = fn(b) {
            fn(c) {
            fn(d) { a + b + c + d };
            };
            };
            let newAdderInner = newAdderOuter(2);
            let adder = newAdderInner(3);
            adder(8);",
            14,
        ),
        (
            "let newClosure = fn(a, b) {
                let one = fn() { a; };
                let two = fn() { b; };
                fn() { one() + two(); };
                };
                let closure = newClosure(9, 90);
                closure();",
            99,
        ),
    ];
    for (test_input, expected) in tests {
        match run(test_input) {
            Ok(obj) => assert_eq!(obj.to_string(), expected.to_string()),
            Err(error) => panic!("VM error! {:?}", error),
        }
    }
}

#[test]
fn recursive_functions_test() {
    let tests = vec![
        (
            "let countDown = fn(x) {
            if (x == 0) {
            return 0;
            } else {
            countDown(x - 1);
            }
            };
            countDown(1);",
            0,
        ),
        (
            "let countDown = fn(x) {
        if (x == 0) {
        return 0;
        } else {
        countDown(x - 1);
        }
        };
        let wrapper = fn() {
        countDown(1);
        };
        wrapper();",
            0,
        ),
        (
            "let wrapper = fn() {
            let countDown = fn(x) {
            if (x == 0) {
            return 0;
            } else {
            countDown(x - 1);
            }
            };
            countDown(1);
            };
            wrapper();",
            0,
        ),
        (
            "let fibonacci = fn(x) {
            if (x == 0) {
            return 0;
            } else {
            if (x == 1) {
            return 1;
            } else {
            fibonacci(x - 1) + fibonacci(x - 2);
            }
            }
            };
            fibonacci(15);",
            610,
        ),
    ];
    for (test_input, expected) in tests {
        match run(test_input) {
            Ok(obj) => assert_eq!(obj.to_string(), expected.to_string()),
            Err(error) => panic!("VM error! {:?}", error),
        }
    }
}
