use crate::token;
use crate::lexer;
use std::io;
use std::io::Write;

const PROMPT: &str = ">>";

pub fn start() -> io::Result<()> {
    loop {
        print!("{}", PROMPT);
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        println!("Lexing...\n\n");
        let mut l = lexer::Lexer::new(&input);
        // TODO: This is currently an infinite loop.
        loop {
            let t = l.next_token();
            println!("{:?}", t);
            if t == token::Token::EndOfFile {
                break;
            }
        }
    }
}