use crate::ast::Program;
use crate::compiler;
use crate::evaluator;
use crate::lexer;
use crate::object::Environment;
use crate::parser;
use crate::vm;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

pub fn start(compile: bool) {
    let input = "let fibonacci = fn(x) {
        if (x == 0) {
            0
        } else {
            if (x == 1) {
                1
            } else {
                fibonacci(x - 1) + fibonacci(x - 2)
            }
        }
    };
    fibonacci(35);";

    let mut p = parser::Parser::new(lexer::Lexer::new(&input));
    let program = p.parse_program().unwrap();

    if compile {
        benchmark_with_compiler(&program);
    } else {
        benchmark_with_interpreter(&program);
    }
}

fn benchmark_with_interpreter(program: &Program) {
    let env = Rc::new(RefCell::new(Environment::new()));
    let start = Instant::now();
    let result = evaluator::eval(&program, Rc::clone(&env)).unwrap();
    let elapsed = start.elapsed();
    println!(
        "{} seconds {} nanoseconds, result: {}",
        elapsed.as_secs(),
        elapsed.subsec_nanos(),
        result
    );
}

fn benchmark_with_compiler(program: &Program) {
    let mut compiler = compiler::Compiler::new();
    let bytecode = compiler.compile(&program).unwrap();

    let mut vm = vm::Vm::new(&bytecode);
    let start = Instant::now();
    let result = vm.run().unwrap();
    let elapsed = start.elapsed();
    println!(
        "{} seconds {} nanoseconds, result: {}",
        elapsed.as_secs(),
        elapsed.subsec_nanos(),
        result
    );
}
