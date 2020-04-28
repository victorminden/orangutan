//! Orangutan
//!
//! `orangutan` is a rust implementation of the Monkey language.
//! The public interface consists only of the simple read-eval-print-loop in the `repl` module.
//!
//! Documentation also exists for the private modules within the package (run `cargo doc --document-private-items`).
extern crate num_enum;

mod ast;
mod code;
mod compiler;
mod evaluator;
mod lexer;
mod object;
mod parser;
pub mod repl;
mod token;
mod vm;
