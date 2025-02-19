//!  ParseError
//!
//! `parse_error` contains an enum type for representing errors encountered during parsing.
use crate::token::Token;
use std::fmt;

///  Represents any errors encountered during parsing of Monkey tokens.
///
/// Most errors are specific and explain exactly which token was expected instead of the found token.
/// However, in some cases we fall back to generic errors to make implementation less cumbersome.
#[derive(Debug, Clone)]
pub enum ParseError {
    UnexpectedToken(Token),
    ExpectedIdent(Token),
    ExpectedLet(Token),
    ExpectedAssign(Token),
    ExpectedInteger(Token),
    ExpectedBoolean(Token),
    ExpectedPrefix(Token),
    ExpectedRParen(Token),
    ExpectedSemicolon(Token),
    ExpectedStr(Token),
    UnknownError,
}

fn expected_x_got_y(f: &mut fmt::Formatter, expected: &str, got: &Token) -> fmt::Result {
    write!(f, "ParseError: expected `{}`, got {}!", expected, got)
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::ExpectedIdent(token) => expected_x_got_y(f, "identifier", token),
            ParseError::ExpectedStr(token) => expected_x_got_y(f, "string", token),
            ParseError::ExpectedLet(token) => expected_x_got_y(f, "let", token),
            ParseError::ExpectedAssign(token) => expected_x_got_y(f, "assign", token),
            ParseError::ExpectedInteger(token) => expected_x_got_y(f, "integer", token),
            ParseError::ExpectedBoolean(token) => expected_x_got_y(f, "boolean", token),
            ParseError::ExpectedPrefix(token) => expected_x_got_y(f, "prefix", token),
            ParseError::ExpectedRParen(token) => expected_x_got_y(f, "(", token),
            ParseError::ExpectedSemicolon(token) => expected_x_got_y(f, ";", token),
            ParseError::UnexpectedToken(token) => {
                write!(f, "ParseError: UnexpectedToken `{}`!", token)
            }
            ParseError::UnknownError => write!(f, "ParseError: UnknownError!"),
        }
    }
}
