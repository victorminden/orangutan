use super::*;

use crate::ast::Program;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::code::{OpCode, Constant};

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
        Err(_) => panic!("Compilation error!")
    };

    test_instructions(test_case.expected_instructions, bytecode.instructions);
    test_constants(test_case.expected_constants, bytecode.constants);
}

fn test_instructions(want: Vec<Instructions>, got: Instructions) {
    let mut catted_want = vec![];
    for inst in want {
        catted_want.extend(inst);
    }
    assert_eq!(catted_want.len(), got.len());
    for (w, g) in catted_want.iter().zip(got.iter()) {
        assert_eq!(w, g);
    }
}

fn test_constants(want: Vec<Constant>, got: Vec<Constant>) {
    for (w, g) in want.iter().zip(got.iter()) {
        match (w, g) {
            (Constant::Integer(want), Constant::Integer(got)) => {
                assert_eq!(want, got);
            }
            _ => panic!("Unexpected constants!")
        }
    }
}


#[test]
fn integer_arithmetic_test() {
    let tests = vec![
        TestCase {
            input: "1+2", 
            expected_constants: vec![
                Constant::Integer(1), 
                Constant::Integer(2),
                ], 
            expected_instructions :vec![
                OpCode::Constant.make_u16(0),
                OpCode::Constant.make_u16(1),
                OpCode::Add.make(),
                ],
        },
    ];
    for test in tests {
        test_compile(test);
    }
}

