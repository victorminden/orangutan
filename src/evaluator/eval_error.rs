//! EvalError
//!
//! `eval_error` contains an enum type representing errors encountered while evaluating Monkey statements.
use crate::object::Object;
use crate::token::Token;
use std::fmt;

/// Represents errors encountered during evaluation of the Monkey language.
///
/// In most cases the error is specific and self-explanatory.
/// However, in some cases we fallback to a generic error to make implementation less cumbersome.
#[derive(Debug)]
pub enum EvalError {
    UnknownError,
    UnknownPrefixOperator(Token),
    UnknownInfixOperator(Token),
    UnknownIdentifier(String),
    InfixTypeMismatch(Object, Token, Object),
    PrefixTypeMismatch(Token, Object),
    WrongNumberOfArguments(u32, u32),
    UnsupportedInputToBuiltIn,
    HashError(Object),
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EvalError::UnknownPrefixOperator(token) => {
                write!(f, "EvalError: Unknown prefix operator `{}`", token)
            }
            EvalError::UnknownInfixOperator(token) => {
                write!(f, "EvalError: Unknown infix operator `{}`", token)
            }
            EvalError::InfixTypeMismatch(_, token, _) => {
                write!(f, "EvalError: Type mismatch for infix operator `{}`", token)
            }
            EvalError::PrefixTypeMismatch(token, _) => write!(
                f,
                "EvalError: Type mismatch for prefix operator `{}`",
                token
            ),
            EvalError::UnknownIdentifier(name) => {
                write!(f, "EvalError: Unknown identifier `{}`", name)
            }
            EvalError::WrongNumberOfArguments(got, want) => write!(
                f,
                "EvalError: Wrong number of parameters (got: {}, want: {}",
                got, want
            ),
            EvalError::UnknownError => write!(f, "EvalError: UnknownError"),
            EvalError::UnsupportedInputToBuiltIn => {
                write!(f, "EvalError: Unsupported input to built-in function")
            }
            EvalError::HashError(obj) => write!(f, "{} is not hashable!", obj),
        }
    }
}
