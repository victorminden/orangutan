extern crate orangutan;

// TODO: Determine an appropriate error type to return.
fn main() -> Result<(), std::io::Error> {
    println!("Welcome to the Monkey programming language!");
    println!("Feel free to type in commands to be parsed (but not yet evaluated)");
    orangutan::repl::start()
}
