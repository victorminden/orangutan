use super::*;

use crate::ast::Program;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::compiler::Compiler;
use crate::code::{OpCode, Constant};
use crate::object::Object;

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
    ];
    for (test_input, expected) in tests {
        match run(test_input) {
            Ok(obj) => assert_eq!(obj.to_string(), expected.to_string()),
            _ => panic!("VM error!"),
        }
    }
}