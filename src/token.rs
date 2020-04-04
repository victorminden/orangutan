use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Token {
    Null,
    Illegal,
    EndOfFile,
    // Identifiers + literals
    Ident(String),
    Integer(i32),
    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    LessThan,
    GreaterThan,
    Equal,
    NotEqual,
    // Delimiters
    Comma,
    Semicolon,
    // Groups
    LParen,
    RParen, 
    LBrace,
    RBrace,
    // Keywords,
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

pub fn lookup_ident(ident: String) -> Token {
    match &*ident {
        "fn" => Token::Function,
        "let" => Token::Let,
        "true" => Token::True,
        "false" => Token::False,
        "if" => Token::If,
        "else" => Token::Else,
        "return" => Token::Return,
        _ => Token::Ident(ident)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Ident(ident) => write!(f, "{}", ident),
            Token::Integer(i) => write!(f, "{}", i),
            Token::Assign => write!(f, "="),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Equal => write!(f, "=="),
            Token::NotEqual => write!(f, "!="),
            Token::Asterisk => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Bang =>write!(f, "!"),
            Token::LessThan => write!(f, "<"),
            Token::GreaterThan => write!(f, ">"),
            other => write!(f, "{:?}", other),
        }
    }
}
