extern crate orangutan;
use std::env;

fn main() -> Result<(), std::io::Error> {
    let compile = env::args().any(|arg| arg == "--compile");
    let repl_or_benchmark = env::args().nth(1);
    match repl_or_benchmark {
        Some(repl_or_benchmark) => match repl_or_benchmark.as_ref() {
            "repl" => orangutan::repl::start(compile),
            "bench" => {
                orangutan::benchmark::start(compile);
                Ok(())
            }
            _ => {
                println!("Unrecognized input!");
                Ok(())
            }
        },
        None => orangutan::repl::start(compile),
    }
}
