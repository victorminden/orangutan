extern crate monkey_interpreter;

// TODO: Determine an appropriate error type to return.
fn main() -> Result<(), std::io::Error> {
    monkey_interpreter::repl::start()
}
