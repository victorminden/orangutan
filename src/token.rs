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