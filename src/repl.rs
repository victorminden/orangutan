use crate::lexer;
use crate::parser;
use crate::evaluator;
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
    println!("Feel free to type in commands");
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

        if let Ok(evaluated) = evaluator::eval(program) {
            println!("{}", evaluated);
        } else {
            println!("Error encountered while evaluating the input!");
            continue;
        }
    }
}