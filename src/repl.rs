use crate::lexer;
use crate::parser;
use std::io;
use std::io::Write;

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


pub fn start() -> io::Result<()> {
    println!("Welcome to the Monkey programming language!");
    println!("{}", MONKEY_FACE);
    println!("Feel free to type in commands to be parsed (but not yet evaluated)");
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