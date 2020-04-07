use crate::lexer;
use crate::parser;
use std::io;
use std::io::Write;

const PROMPT: &str = ">>";

pub fn start() -> io::Result<()> {
    loop {
        print!("{}", PROMPT);
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        println!("Parsing...\n\n");
        let mut p = parser::Parser::new(lexer::Lexer::new(&input));
        if let Ok(program) = p.parse_program() {
            for statement in program.statements {
                println!("{}", statement);
            }
        } else {
            println!("Error parsing program.")
        }
        p.print_errors();
    }
}