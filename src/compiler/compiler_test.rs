use super::*;

use crate::ast::Program;
use crate::code::{disassemble, CompiledFunction, Constant, OpCode};
use crate::lexer::Lexer;
use crate::parser::Parser;

struct TestCase<'a> {
    input: &'a str,
    expected_constants: Vec<Constant>,
    expected_instructions: Vec<Instructions>,
}

fn parse(input: &str) -> Program {
    let mut p = Parser::new(Lexer::new(input));
    p.parse_program().unwrap()
}

fn test_compile(test_case: TestCase) {
    let program = parse(test_case.input);
    let mut compiler = Compiler::new();
    let bytecode = match compiler.compile(&program) {
        Ok(code) => code,
        Err(_) => panic!("Compilation error!"),
    };

    test_constants(test_case.expected_constants, bytecode.constants);
    test_instructions(test_case.expected_instructions, bytecode.instructions);
}

fn test_instructions(want: Vec<Instructions>, got: Instructions) {
    let mut catted_want = vec![];
    for inst in want {
        catted_want.extend(inst);
    }

    for (w, g) in catted_want.iter().zip(got.iter()) {
        assert_eq!(
            w,
            g,
            "\n\nwant: \n{}, \n\ngot: \n{}",
            disassemble(&catted_want),
            disassemble(&got)
        );
    }
    assert_eq!(
        catted_want.len(),
        got.len(),
        "\n\nwant: \n{}, \n\ngot: \n{}",
        disassemble(&catted_want),
        disassemble(&got)
    );
}

fn test_constants(want: Vec<Constant>, got: Vec<Constant>) {
    for (w, g) in want.iter().zip(got.iter()) {
        match (w, g) {
            (Constant::Integer(want), Constant::Integer(got)) => {
                assert_eq!(want, got, "Bad integer constant!");
            }
            (Constant::Str(want), Constant::Str(got)) => {
                assert_eq!(want, got, "Bad string constant!");
            }
            (Constant::CompiledFunction(want), Constant::CompiledFunction(got)) => {
                assert_eq!(
                    want,
                    got,
                    "\n\nwanted: \n{}\n\n got: \n{}\n",
                    disassemble(&want.instructions[..]),
                    disassemble(&got.instructions[..]),
                );
            }
            _ => panic!(
                "Unexpected constants! \n\nwant: \n{:?}, \n\ngot: \n{:?}",
                want, got
            ),
        }
    }
}

#[test]
fn integer_arithmetic_test() {
    let tests = vec![
        TestCase {
            input: "1+2",
            expected_constants: vec![Constant::Integer(1), Constant::Integer(2)],
            expected_instructions: vec![
                OpCode::Constant.make_u16(0),
                OpCode::Constant.make_u16(1),
                OpCode::Add.make(),
                OpCode::Pop.make(),
            ],
        },
        TestCase {
            input: "1; 2",
            expected_constants: vec![Constant::Integer(1), Constant::Integer(2)],
            expected_instructions: vec![
                OpCode::Constant.make_u16(0),
                OpCode::Pop.make(),
                OpCode::Constant.make_u16(1),
                OpCode::Pop.make(),
            ],
        },
        TestCase {
            input: "2-1",
            expected_constants: vec![Constant::Integer(2), Constant::Integer(1)],
            expected_instructions: vec![
                OpCode::Constant.make_u16(0),
                OpCode::Constant.make_u16(1),
                OpCode::Sub.make(),
                OpCode::Pop.make(),
            ],
        },
        TestCase {
            input: "1*2",
            expected_constants: vec![Constant::Integer(1), Constant::Integer(2)],
            expected_instructions: vec![
                OpCode::Constant.make_u16(0),
                OpCode::Constant.make_u16(1),
                OpCode::Mul.make(),
                OpCode::Pop.make(),
            ],
        },
        TestCase {
            input: "2/1",
            expected_constants: vec![Constant::Integer(2), Constant::Integer(1)],
            expected_instructions: vec![
                OpCode::Constant.make_u16(0),
                OpCode::Constant.make_u16(1),
                OpCode::Div.make(),
                OpCode::Pop.make(),
            ],
        },
        TestCase {
            input: "-1",
            expected_constants: vec![Constant::Integer(1)],
            expected_instructions: vec![
                OpCode::Constant.make_u16(0),
                OpCode::Minus.make(),
                OpCode::Pop.make(),
            ],
        },
    ];
    for test in tests {
        test_compile(test);
    }
}

#[test]
fn boolean_test() {
    let tests = vec![
        TestCase {
            input: "true",
            expected_constants: vec![],
            expected_instructions: vec![OpCode::True.make(), OpCode::Pop.make()],
        },
        TestCase {
            input: "!true",
            expected_constants: vec![],
            expected_instructions: vec![
                OpCode::True.make(),
                OpCode::Bang.make(),
                OpCode::Pop.make(),
            ],
        },
        TestCase {
            input: "false",
            expected_constants: vec![],
            expected_instructions: vec![OpCode::False.make(), OpCode::Pop.make()],
        },
        TestCase {
            input: "1 > 2",
            expected_constants: vec![Constant::Integer(1), Constant::Integer(2)],
            expected_instructions: vec![
                OpCode::Constant.make_u16(0),
                OpCode::Constant.make_u16(1),
                OpCode::GreaterThan.make(),
                OpCode::Pop.make(),
            ],
        },
        TestCase {
            input: "1 < 2",
            expected_constants: vec![Constant::Integer(2), Constant::Integer(1)],
            expected_instructions: vec![
                OpCode::Constant.make_u16(0),
                OpCode::Constant.make_u16(1),
                OpCode::GreaterThan.make(),
                OpCode::Pop.make(),
            ],
        },
        TestCase {
            input: "1 == 2",
            expected_constants: vec![Constant::Integer(1), Constant::Integer(2)],
            expected_instructions: vec![
                OpCode::Constant.make_u16(0),
                OpCode::Constant.make_u16(1),
                OpCode::Equal.make(),
                OpCode::Pop.make(),
            ],
        },
        TestCase {
            input: "1 != 2",
            expected_constants: vec![Constant::Integer(1), Constant::Integer(2)],
            expected_instructions: vec![
                OpCode::Constant.make_u16(0),
                OpCode::Constant.make_u16(1),
                OpCode::NotEqual.make(),
                OpCode::Pop.make(),
            ],
        },
        TestCase {
            input: "true == false",
            expected_constants: vec![],
            expected_instructions: vec![
                OpCode::True.make(),
                OpCode::False.make(),
                OpCode::Equal.make(),
                OpCode::Pop.make(),
            ],
        },
    ];
    for test in tests {
        test_compile(test);
    }
}

#[test]
fn conditionals_test() {
    let tests = vec![
        TestCase {
            input: "if (true) { 10 }; 3333;",
            expected_constants: vec![Constant::Integer(10), Constant::Integer(3333)],
            expected_instructions: vec![
                OpCode::True.make(),
                OpCode::JumpNotTruthy.make_u16(10),
                OpCode::Constant.make_u16(0),
                OpCode::Jump.make_u16(11),
                OpCode::Null.make(),
                OpCode::Pop.make(),
                OpCode::Constant.make_u16(1),
                OpCode::Pop.make(),
            ],
        },
        TestCase {
            input: "if (true) { 10 } else { 20 };",
            expected_constants: vec![Constant::Integer(10), Constant::Integer(20)],
            expected_instructions: vec![
                OpCode::True.make(),
                OpCode::JumpNotTruthy.make_u16(10),
                OpCode::Constant.make_u16(0),
                OpCode::Jump.make_u16(13),
                OpCode::Constant.make_u16(1),
                OpCode::Pop.make(),
            ],
        },
    ];
    for test in tests {
        test_compile(test);
    }
}

#[test]
fn global_let_statement_test() {
    let tests = vec![
        TestCase {
            input: "let one = 1;
            let two = 2;",
            expected_constants: vec![Constant::Integer(1), Constant::Integer(2)],
            expected_instructions: vec![
                OpCode::Constant.make_u16(0),
                OpCode::SetGlobal.make_u16(0),
                OpCode::Constant.make_u16(1),
                OpCode::SetGlobal.make_u16(1),
            ],
        },
        TestCase {
            input: "let one = 1;
            one;",
            expected_constants: vec![Constant::Integer(1)],
            expected_instructions: vec![
                OpCode::Constant.make_u16(0),
                OpCode::SetGlobal.make_u16(0),
                OpCode::GetGlobal.make_u16(0),
                OpCode::Pop.make(),
            ],
        },
        TestCase {
            input: "let one = 1;
            let two = one;
            two;",
            expected_constants: vec![Constant::Integer(1)],
            expected_instructions: vec![
                OpCode::Constant.make_u16(0),
                OpCode::SetGlobal.make_u16(0),
                OpCode::GetGlobal.make_u16(0),
                OpCode::SetGlobal.make_u16(1),
                OpCode::GetGlobal.make_u16(1),
                OpCode::Pop.make(),
            ],
        },
    ];
    for test in tests {
        test_compile(test);
    }
}

#[test]
fn string_expression_test() {
    let tests = vec![
        TestCase {
            input: "\"monkey\"",
            expected_constants: vec![Constant::Str(String::from("monkey"))],
            expected_instructions: vec![OpCode::Constant.make_u16(0), OpCode::Pop.make()],
        },
        TestCase {
            input: "\"mon\" + \"key\"",
            expected_constants: vec![
                Constant::Str(String::from("mon")),
                Constant::Str(String::from("key")),
            ],
            expected_instructions: vec![
                OpCode::Constant.make_u16(0),
                OpCode::Constant.make_u16(1),
                OpCode::Add.make(),
                OpCode::Pop.make(),
            ],
        },
    ];
    for test in tests {
        test_compile(test);
    }
}

#[test]
fn array_literal_test() {
    let tests = vec![
        TestCase {
            input: "[]",
            expected_constants: vec![],
            expected_instructions: vec![OpCode::Array.make_u16(0), OpCode::Pop.make()],
        },
        TestCase {
            input: "[1, 2, 3]",
            expected_constants: vec![
                Constant::Integer(1),
                Constant::Integer(2),
                Constant::Integer(3),
            ],
            expected_instructions: vec![
                OpCode::Constant.make_u16(0),
                OpCode::Constant.make_u16(1),
                OpCode::Constant.make_u16(2),
                OpCode::Array.make_u16(3),
                OpCode::Pop.make(),
            ],
        },
        TestCase {
            input: "[1+2, 3-4, 5*6]",
            expected_constants: vec![
                Constant::Integer(1),
                Constant::Integer(2),
                Constant::Integer(3),
                Constant::Integer(4),
                Constant::Integer(5),
                Constant::Integer(6),
            ],
            expected_instructions: vec![
                OpCode::Constant.make_u16(0),
                OpCode::Constant.make_u16(1),
                OpCode::Add.make(),
                OpCode::Constant.make_u16(2),
                OpCode::Constant.make_u16(3),
                OpCode::Sub.make(),
                OpCode::Constant.make_u16(4),
                OpCode::Constant.make_u16(5),
                OpCode::Mul.make(),
                OpCode::Array.make_u16(3),
                OpCode::Pop.make(),
            ],
        },
    ];
    for test in tests {
        test_compile(test);
    }
}

#[test]
fn hash_literal_test() {
    let tests = vec![
        TestCase {
            input: "{}",
            expected_constants: vec![],
            expected_instructions: vec![OpCode::Hash.make_u16(0), OpCode::Pop.make()],
        },
        TestCase {
            input: "{1: 2, 3: 4}",
            expected_constants: vec![
                Constant::Integer(1),
                Constant::Integer(2),
                Constant::Integer(3),
                Constant::Integer(4),
            ],
            expected_instructions: vec![
                OpCode::Constant.make_u16(0),
                OpCode::Constant.make_u16(1),
                OpCode::Constant.make_u16(2),
                OpCode::Constant.make_u16(3),
                OpCode::Hash.make_u16(4),
                OpCode::Pop.make(),
            ],
        },
        TestCase {
            input: "{1: 2 + 3, 4: 5 * 6}",
            expected_constants: vec![
                Constant::Integer(1),
                Constant::Integer(2),
                Constant::Integer(3),
                Constant::Integer(4),
                Constant::Integer(5),
                Constant::Integer(6),
            ],
            expected_instructions: vec![
                OpCode::Constant.make_u16(0),
                OpCode::Constant.make_u16(1),
                OpCode::Constant.make_u16(2),
                OpCode::Add.make(),
                OpCode::Constant.make_u16(3),
                OpCode::Constant.make_u16(4),
                OpCode::Constant.make_u16(5),
                OpCode::Mul.make(),
                OpCode::Hash.make_u16(4),
                OpCode::Pop.make(),
            ],
        },
    ];
    for test in tests {
        test_compile(test);
    }
}

#[test]
fn index_expression_test() {
    let tests = vec![
        TestCase {
            input: "[1,2,3][1+1]",
            expected_constants: vec![
                Constant::Integer(1),
                Constant::Integer(2),
                Constant::Integer(3),
                Constant::Integer(1),
                Constant::Integer(1),
            ],
            expected_instructions: vec![
                OpCode::Constant.make_u16(0),
                OpCode::Constant.make_u16(1),
                OpCode::Constant.make_u16(2),
                OpCode::Array.make_u16(3),
                OpCode::Constant.make_u16(3),
                OpCode::Constant.make_u16(4),
                OpCode::Add.make(),
                OpCode::Index.make(),
                OpCode::Pop.make(),
            ],
        },
        TestCase {
            input: "{1: 2}[2-1]",
            expected_constants: vec![
                Constant::Integer(1),
                Constant::Integer(2),
                Constant::Integer(2),
                Constant::Integer(1),
            ],
            expected_instructions: vec![
                OpCode::Constant.make_u16(0),
                OpCode::Constant.make_u16(1),
                OpCode::Hash.make_u16(2),
                OpCode::Constant.make_u16(2),
                OpCode::Constant.make_u16(3),
                OpCode::Sub.make(),
                OpCode::Index.make(),
                OpCode::Pop.make(),
            ],
        },
    ];
    for test in tests {
        test_compile(test);
    }
}

#[test]
fn function_test() {
    let tests = vec![
        TestCase {
            input: "fn() { return 5 + 10; }",
            expected_constants: vec![
                Constant::Integer(5),
                Constant::Integer(10),
                compiled_function(
                    vec![
                        OpCode::Constant.make_u16(0),
                        OpCode::Constant.make_u16(1),
                        OpCode::Add.make(),
                        OpCode::ReturnValue.make(),
                    ],
                    0,
                    0,
                ),
            ],
            expected_instructions: vec![OpCode::Closure.make_u16_u8(2, 0), OpCode::Pop.make()],
        },
        TestCase {
            input: "fn() { 5 + 10; }",
            expected_constants: vec![
                Constant::Integer(5),
                Constant::Integer(10),
                compiled_function(
                    vec![
                        OpCode::Constant.make_u16(0),
                        OpCode::Constant.make_u16(1),
                        OpCode::Add.make(),
                        OpCode::ReturnValue.make(),
                    ],
                    0,
                    0,
                ),
            ],
            expected_instructions: vec![OpCode::Closure.make_u16_u8(2, 0), OpCode::Pop.make()],
        },
        TestCase {
            input: "fn() { 1; 2 }",
            expected_constants: vec![
                Constant::Integer(1),
                Constant::Integer(2),
                compiled_function(
                    vec![
                        OpCode::Constant.make_u16(0),
                        OpCode::Pop.make(),
                        OpCode::Constant.make_u16(1),
                        OpCode::ReturnValue.make(),
                    ],
                    0,
                    0,
                ),
            ],
            expected_instructions: vec![OpCode::Closure.make_u16_u8(2, 0), OpCode::Pop.make()],
        },
        TestCase {
            input: "fn() {}",
            expected_constants: vec![compiled_function(vec![OpCode::Return.make()], 0, 0)],
            expected_instructions: vec![OpCode::Closure.make_u16_u8(0, 0), OpCode::Pop.make()],
        },
    ];
    for test in tests {
        test_compile(test);
    }
}

#[test]
fn function_call_test() {
    let tests = vec![
        TestCase {
            input: "fn() { 24 }()",
            expected_constants: vec![
                Constant::Integer(24),
                compiled_function(
                    vec![OpCode::Constant.make_u16(0), OpCode::ReturnValue.make()],
                    0,
                    0,
                ),
            ],
            expected_instructions: vec![
                OpCode::Closure.make_u16_u8(1, 0),
                OpCode::Call.make_u8(0),
                OpCode::Pop.make(),
            ],
        },
        TestCase {
            input: "let noargs = fn() { 24 };
            noargs()",
            expected_constants: vec![
                Constant::Integer(24),
                compiled_function(
                    vec![OpCode::Constant.make_u16(0), OpCode::ReturnValue.make()],
                    0,
                    0,
                ),
            ],
            expected_instructions: vec![
                OpCode::Closure.make_u16_u8(1, 0),
                OpCode::SetGlobal.make_u16(0),
                OpCode::GetGlobal.make_u16(0),
                OpCode::Call.make_u8(0),
                OpCode::Pop.make(),
            ],
        },
        TestCase {
            input: "let onearg = fn(a) { a };
            onearg(24)",
            expected_constants: vec![
                compiled_function(
                    vec![OpCode::GetLocal.make_u8(0), OpCode::ReturnValue.make()],
                    1,
                    1,
                ),
                Constant::Integer(24),
            ],
            expected_instructions: vec![
                OpCode::Closure.make_u16_u8(0, 0),
                OpCode::SetGlobal.make_u16(0),
                OpCode::GetGlobal.make_u16(0),
                OpCode::Constant.make_u16(1),
                OpCode::Call.make_u8(1),
                OpCode::Pop.make(),
            ],
        },
        TestCase {
            input: "let manyarg = fn(a,b,c) { a; b; c };
            manyarg(24,25,26)",
            expected_constants: vec![
                compiled_function(
                    vec![
                        OpCode::GetLocal.make_u8(0),
                        OpCode::Pop.make(),
                        OpCode::GetLocal.make_u8(1),
                        OpCode::Pop.make(),
                        OpCode::GetLocal.make_u8(2),
                        OpCode::ReturnValue.make(),
                    ],
                    3,
                    3,
                ),
                Constant::Integer(24),
                Constant::Integer(25),
                Constant::Integer(26),
            ],
            expected_instructions: vec![
                OpCode::Closure.make_u16_u8(0, 0),
                OpCode::SetGlobal.make_u16(0),
                OpCode::GetGlobal.make_u16(0),
                OpCode::Constant.make_u16(1),
                OpCode::Constant.make_u16(2),
                OpCode::Constant.make_u16(3),
                OpCode::Call.make_u8(3),
                OpCode::Pop.make(),
            ],
        },
    ];
    for test in tests {
        test_compile(test);
    }
}

#[test]
fn let_statement_scopes_test() {
    let tests = vec![
        TestCase {
            input: "let num = 55; fn() { num }",
            expected_constants: vec![
                Constant::Integer(55),
                compiled_function(
                    vec![OpCode::GetGlobal.make_u16(0), OpCode::ReturnValue.make()],
                    0,
                    0,
                ),
            ],
            expected_instructions: vec![
                OpCode::Constant.make_u16(0),
                OpCode::SetGlobal.make_u16(0),
                OpCode::Closure.make_u16_u8(1, 0),
                OpCode::Pop.make(),
            ],
        },
        TestCase {
            input: "fn() { let num = 55; num }",
            expected_constants: vec![
                Constant::Integer(55),
                compiled_function(
                    vec![
                        OpCode::Constant.make_u16(0),
                        OpCode::SetLocal.make_u8(0),
                        OpCode::GetLocal.make_u8(0),
                        OpCode::ReturnValue.make(),
                    ],
                    1,
                    0,
                ),
            ],
            expected_instructions: vec![OpCode::Closure.make_u16_u8(1, 0), OpCode::Pop.make()],
        },
        TestCase {
            input: "fn() {
                let a = 55;
                let b = 77;
                a + b
                }",
            expected_constants: vec![
                Constant::Integer(55),
                Constant::Integer(77),
                compiled_function(
                    vec![
                        OpCode::Constant.make_u16(0),
                        OpCode::SetLocal.make_u8(0),
                        OpCode::Constant.make_u16(1),
                        OpCode::SetLocal.make_u8(1),
                        OpCode::GetLocal.make_u8(0),
                        OpCode::GetLocal.make_u8(1),
                        OpCode::Add.make(),
                        OpCode::ReturnValue.make(),
                    ],
                    2,
                    0,
                ),
            ],
            expected_instructions: vec![OpCode::Closure.make_u16_u8(2, 0), OpCode::Pop.make()],
        },
        TestCase {
            input: " let a = 55;
            fn() {
                let b = 77;
                a + b
                }",
            expected_constants: vec![
                Constant::Integer(55),
                Constant::Integer(77),
                compiled_function(
                    vec![
                        OpCode::Constant.make_u16(1),
                        OpCode::SetLocal.make_u8(0),
                        OpCode::GetGlobal.make_u16(0),
                        OpCode::GetLocal.make_u8(0),
                        OpCode::Add.make(),
                        OpCode::ReturnValue.make(),
                    ],
                    1,
                    0,
                ),
            ],
            expected_instructions: vec![
                OpCode::Constant.make_u16(0),
                OpCode::SetGlobal.make_u16(0),
                OpCode::Closure.make_u16_u8(2, 0),
                OpCode::Pop.make(),
            ],
        },
    ];
    for test in tests {
        test_compile(test);
    }
}

#[test]
fn builtin_test() {
    let tests = vec![
        TestCase {
            input: "len([]); push([], 1);",
            expected_constants: vec![Constant::Integer(1)],
            expected_instructions: vec![
                OpCode::GetBuiltin.make_u8(0),
                OpCode::Array.make_u16(0),
                OpCode::Call.make_u8(1),
                OpCode::Pop.make(),
                OpCode::GetBuiltin.make_u8(4),
                OpCode::Array.make_u16(0),
                OpCode::Constant.make_u16(0),
                OpCode::Call.make_u8(2),
                OpCode::Pop.make(),
            ],
        },
        TestCase {
            input: "fn() { len([]); };",
            expected_constants: vec![compiled_function(
                vec![
                    OpCode::GetBuiltin.make_u8(0),
                    OpCode::Array.make_u16(0),
                    OpCode::Call.make_u8(1),
                    OpCode::ReturnValue.make(),
                ],
                0,
                0,
            )],
            expected_instructions: vec![OpCode::Closure.make_u16_u8(0, 0), OpCode::Pop.make()],
        },
    ];
    for test in tests {
        test_compile(test);
    }
}

#[test]
fn closure_test() {
    let tests = vec![
        TestCase {
            input: "fn(a) {
                fn(b) {
                a + b
                };
                };",
            expected_constants: vec![
                compiled_function(
                    vec![
                        OpCode::GetFree.make_u8(0),
                        OpCode::GetLocal.make_u8(0),
                        OpCode::Add.make(),
                        OpCode::ReturnValue.make(),
                    ],
                    1,
                    1,
                ),
                compiled_function(
                    vec![
                        OpCode::GetLocal.make_u8(0),
                        OpCode::Closure.make_u16_u8(0, 1),
                        OpCode::ReturnValue.make(),
                    ],
                    1,
                    1,
                ),
            ],
            expected_instructions: vec![OpCode::Closure.make_u16_u8(1, 0), OpCode::Pop.make()],
        },
        TestCase {
            input: "let global = 55;
                fn() {
                    let a = 66;
                    fn() {
                        let b = 77;
                        fn() {
                            let c = 88;
                            global + a + b + c;
                        }
                    }
                }",
            expected_constants: vec![
                Constant::Integer(55),
                Constant::Integer(66),
                Constant::Integer(77),
                Constant::Integer(88),
                compiled_function(
                    vec![
                        OpCode::Constant.make_u16(3),
                        OpCode::SetLocal.make_u8(0),
                        OpCode::GetGlobal.make_u16(0),
                        OpCode::GetFree.make_u8(0),
                        OpCode::Add.make(),
                        OpCode::GetFree.make_u8(1),
                        OpCode::Add.make(),
                        OpCode::GetLocal.make_u8(0),
                        OpCode::Add.make(),
                        OpCode::ReturnValue.make(),
                    ],
                    1,
                    0,
                ),
                compiled_function(
                    vec![
                        OpCode::Constant.make_u16(2),
                        OpCode::SetLocal.make_u8(0),
                        OpCode::GetFree.make_u8(0),
                        OpCode::GetLocal.make_u8(0),
                        OpCode::Closure.make_u16_u8(4, 2),
                        OpCode::ReturnValue.make(),
                    ],
                    1,
                    0,
                ),
                compiled_function(
                    vec![
                        OpCode::Constant.make_u16(1),
                        OpCode::SetLocal.make_u8(0),
                        OpCode::GetLocal.make_u8(0),
                        OpCode::Closure.make_u16_u8(5, 1),
                        OpCode::ReturnValue.make(),
                    ],
                    1,
                    0,
                ),
            ],
            expected_instructions: vec![
                OpCode::Constant.make_u16(0),
                OpCode::SetGlobal.make_u16(0),
                OpCode::Closure.make_u16_u8(6, 0),
                OpCode::Pop.make(),
            ],
        },
    ];
    for test in tests {
        test_compile(test);
    }
}

#[test]
fn recursive_test() {
    let tests = vec![TestCase {
        input: "let countDown = fn(x) { countDown(x - 1); };
            countDown(1);",
        expected_constants: vec![
            Constant::Integer(1),
            compiled_function(
                vec![
                    OpCode::CurrentClosure.make(),
                    OpCode::GetLocal.make_u8(0),
                    OpCode::Constant.make_u16(0),
                    OpCode::Sub.make(),
                    OpCode::Call.make_u8(1),
                    OpCode::ReturnValue.make(),
                ],
                1,
                1,
            ),
            Constant::Integer(1),
        ],
        expected_instructions: vec![
            OpCode::Closure.make_u16_u8(1, 0),
            OpCode::SetGlobal.make_u16(0),
            OpCode::GetGlobal.make_u16(0),
            OpCode::Constant.make_u16(2),
            OpCode::Call.make_u8(1),
            OpCode::Pop.make(),
        ],
    }];
    for test in tests {
        test_compile(test);
    }
}

fn compiled_function(
    instructions: Vec<Instructions>,
    num_locals: usize,
    num_parameters: usize,
) -> Constant {
    Constant::CompiledFunction(CompiledFunction {
        instructions: instructions.concat(),
        num_locals,
        num_parameters,
    })
}
