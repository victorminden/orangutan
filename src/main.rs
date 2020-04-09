extern crate orangutan;

// TODO: Determine an appropriate error type to return.
fn main() -> Result<(), std::io::Error> {
    orangutan::repl::start()
}
