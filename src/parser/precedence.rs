use crate::token::Token;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
    Index,
}

pub fn precedence(token: Token) -> Precedence {
    match token {
        _ => Precedence::Lowest
    }
}