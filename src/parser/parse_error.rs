use crate::token::Token;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken,
    ExpectedIdent(Token),
    ExpectedLet(Token),
    ExpectedAssign(Token),
    UnknownError,
}