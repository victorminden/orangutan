extern crate orangutan;

// TODO: Determine an appropriate error type to return.
fn main() -> Result<(), std::io::Error> {
    let compile = false;
    //orangutan::repl::start(compile);
    orangutan::benchmark::start(compile);
    Ok(())
}
