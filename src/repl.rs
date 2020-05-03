//! REPL
//!
//! `repl` implements a read-evaluate-print-loop for the Monkey language.
//! The interface is bare-bones, consisting only of reading lines of input from
//! standard in and evaluating them, line by line.
use crate::code::Constant;
use crate::compiler;
use crate::evaluator;
use crate::lexer;
use crate::object::Environment;
use crate::object::Object;
use crate::parser;
use crate::vm;
use std::cell::RefCell;
use std::io;
use std::io::Write;
use std::rc::Rc;

const PROMPT: &str = ">>";
const MONKEY_FACE: &str = "            __,__
   .--.  .-\"     \"-.  .--.
  / .. \\/  .-. .-.  \\/ .. \\
 | |  \'|  /   Y   \\  |\'  | |
 | \\   \\  \\ 0 | 0 /  /   / |
  \\ \'- ,\\.-\"\"\"\"\"\"\"-./, -\' /
   \'\'-\' /_   ^ ^   _\\ \'-\'\'
       |  \\._   _./  |
       \\   \\ \'~\' /   /
        \'._ \'-=-\' _.\'
           \'-----\'
";

/// Starts the REPL.
///
/// Input is read line-by-line in interactive form until the user terminates the process.
pub fn start(compile: bool) -> io::Result<()> {
    println!("Welcome to the Monkey programming language!");
    println!("{}", MONKEY_FACE);
    println!("Feel free to type in commands");

    if compile {
        println!("(REPL is running in compiled mode)");
        start_with_compiler()?;
    } else {
        println!("(REPL is running in interpreted mode)");
        start_with_interpreter()?;
    }
    Ok(())
}

fn start_with_interpreter() -> io::Result<()> {
    let env = Rc::new(RefCell::new(Environment::new()));
    loop {
        print!("{}", PROMPT);
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let mut p = parser::Parser::new(lexer::Lexer::new(&input));
        let program = match p.parse_program() {
            Ok(prog) => prog,
            _ => {
                println!("Error encountered while parsing the input!");
                p.print_errors();
                continue;
            }
        };

        match evaluator::eval(&program, Rc::clone(&env)) {
            Ok(evaluated) => println!("{}", evaluated),
            Err(error) => {
                println!("Error encountered while evaluating the input!");
                println!("{}", error)
            }
        }
    }
}

fn start_with_compiler() -> io::Result<()> {
    let constants: Rc<RefCell<Vec<Constant>>> = Rc::new(RefCell::new(vec![]));
    let symbol_table = Rc::new(RefCell::new(compiler::SymbolTable::new_with_builtins()));
    let globals: Rc<RefCell<Vec<Rc<Object>>>> = Rc::new(RefCell::new(vec![]));

    loop {
        print!("{}", PROMPT);
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let mut p = parser::Parser::new(lexer::Lexer::new(&input));
        let program = match p.parse_program() {
            Ok(prog) => prog,
            _ => {
                println!("Error encountered while parsing the input!");
                p.print_errors();
                continue;
            }
        };

        let mut compiler =
            compiler::Compiler::new_with_state(symbol_table.clone(), constants.clone());
        let bytecode = match compiler.compile(&program) {
            Ok(bc) => bc,
            _ => {
                println!("Error encountered during compilation!");
                continue;
            }
        };

        let mut vm = vm::Vm::new_with_globals_store(&bytecode, globals.clone());
        match vm.run() {
            Ok(obj) => println!("{}", obj),
            _ => println!("Error executing bytecode!"),
        }
    }
}
