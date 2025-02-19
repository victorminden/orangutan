//! Token
//!
//! `token` holds a simple type and functionality for dealing with tokens of the Monkey language during lexing and parsing.
use std::fmt;

/// Represents a single input token recognized as valid in some Monkey language context.
///
/// The different possible values of `Token` represent all tokens defined in the Monkey language.
/// In addition, a few special tokens are defined to represent special cases during lexing and parsing.
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Null,
    Illegal,
    EndOfFile,
    // Identifiers + literals
    Ident(String),
    Integer(i64),
    Str(String),
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
    Colon,
    // Groups
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    // Keywords,
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

/// Converts an input string to its corresponding token type.
///
/// If `ident` is a known keyword, the corresponding keyword token is returned.
/// Otherwise, a generic identifier token is returned.
pub fn lookup_ident(ident: String) -> Token {
    match &*ident {
        "fn" => Token::Function,
        "let" => Token::Let,
        "true" => Token::True,
        "false" => Token::False,
        "if" => Token::If,
        "else" => Token::Else,
        "return" => Token::Return,
        _ => Token::Ident(ident),
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
            Token::Bang => write!(f, "!"),
            Token::LessThan => write!(f, "<"),
            Token::GreaterThan => write!(f, ">"),
            Token::LBrace => write!(f, "{{"),
            Token::LParen => write!(f, "("),
            Token::LBracket => write!(f, "["),
            Token::RBrace => write!(f, "}}"),
            Token::RParen => write!(f, ")"),
            Token::RBracket => write!(f, "]"),
            Token::Null => write!(f, "null"),
            Token::Illegal => write!(f, "illegal"),
            Token::EndOfFile => write!(f, "EOF"),
            Token::Str(s) => write!(f, "{}", s),
            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),
            Token::Function => write!(f, "fn"),
            Token::Let => write!(f, "let"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::Return => write!(f, "return"),
            Token::Colon => write!(f, ":"),
        }
    }
}
