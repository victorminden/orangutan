use crate::token::Token;
use crate::token::lookup_ident;

use std::str::Chars;
use std::iter::Peekable;

fn is_valid_name_symbol(ch: &char) -> bool {
    is_valid_name_start_symbol(ch) || ch.is_numeric()
}

fn is_valid_name_start_symbol(ch: &char) -> bool {
    ch.is_alphabetic() || *ch == '_'
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    peek_buffer: Token,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: input.chars().peekable(),
            peek_buffer: Token::Null,
        }
    }

    pub fn peek_token(&mut self) -> &Token {
        if self.peek_buffer == Token::Null {
            self.peek_buffer = self.next_token_from_input();
        }
        &self.peek_buffer
    }

    pub fn next_token(&mut self) -> Token {
        match self.peek_buffer {
            Token::Null => self.next_token_from_input(),
            _ => std::mem::replace(&mut self.peek_buffer, Token::Null),
        }
    }

    fn next_token_from_input(&mut self) -> Token {
        self.skip_whitespace();
        match self.input.next() {
            Some('=') => {
                if let Some('=') = self.input.peek() {
                    self.input.next();
                    return Token::Equal;
                }
                return Token::Assign;
            },
            Some(';') => Token::Semicolon,
            Some('(') => Token::LParen,
            Some(')') => Token::RParen,
            Some(',') => Token::Comma,
            Some('+') => Token::Plus,
            Some('{') => Token::LBrace,
            Some('}') => Token::RBrace,
            Some('[') => Token::LBracket,
            Some(']') => Token::RBracket,
            Some('-') => Token::Minus,
            Some('/') => Token::Slash,
            Some('*') => Token::Asterisk,
            Some('<') => Token::LessThan,
            Some('>') => Token::GreaterThan,
            Some(':') => Token::Colon,
            Some('!') => {
                if let Some('=') = self.input.peek() {
                    let _ = self.input.next();
                    return Token::NotEqual;
                }
                return Token::Bang;
            },
            None => Token::EndOfFile,
            Some('"') => {
                self.read_string()
            },
            Some(a) => {
                if is_valid_name_start_symbol(&a) {
                    return lookup_ident(self.read_identifier(a));
                } else if a.is_numeric() {
                    return Token::Integer(self.read_number(a));
                }
                return Token::Illegal;
            }
        }
    } 

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.input.peek() {
            if !ch.is_whitespace() {
                return;
            }
            self.input.next();
        }
    }

    fn read_number(&mut self, first: char) -> i64 {
        let mut ident = String::new();
        ident.push(first);
        while let Some(ch) = self.input.peek() {
            if !ch.is_numeric() {
                break;
            }
            if let Some(ch) = self.input.next() {
                ident.push(ch);
            }
        }
        // Bad practice to use unwrap, but we know that what we put together can be a valid int.
        return ident.parse::<i64>().unwrap();
    }

    fn read_identifier(&mut self, first: char) -> String {
        let mut ident = String::new();
        ident.push(first);
        while let Some(ch) = self.input.peek() {
            if !is_valid_name_symbol(ch) {
                break;
            }
            if let Some(ch) = self.input.next() {
                ident.push(ch);
            }
        }
        ident
    }

    fn read_string(&mut self) -> Token {
        // If the string is the final token of the input, the closing quote may be ignored.
        // TODO: Consider changing this to throw an error.
        let mut string = String::new();
        while let Some(ch) = self.input.next() {
            if ch == '"' {
                break;
            }
            string.push(ch);
        }
        return Token::Str(string);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_token_test() {
        let sample_input = "=+(){},;-!*/<>";
        let tests = vec![
            Token::Assign,
            Token::Plus,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Comma,
            Token::Semicolon,
            Token::Minus,
            Token::Bang,
            Token::Asterisk,
            Token::Slash,
            Token::LessThan,
            Token::GreaterThan,
        ];
        let mut line = Lexer::new(sample_input);
        for t in tests {
            let peek_tok = line.peek_token();
            assert_eq!(*peek_tok, t);
            let tok = line.next_token();
            assert_eq!(tok, t);
        }
    }

    #[test]
    fn next_token_harder_test() {
        let sample_input = "let five = 5;
          let ten = 10;

        let add = fn(x, y) {
        x + y;
        };

        let result = add(five, ten);
        if (5 < 10) {
            return true;
            } else {
            return false;
            }
        1 != 2
        1 == 1
        \"foobar\"
        \"foo bar\"
        [1, 2];
        {\"foo\": \"bar\"}";
        let tests = vec![
            Token::Let,
            Token::Ident(String::from("five")),
            Token::Assign,
            Token::Integer(5),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("ten")),
            Token::Assign,
            Token::Integer(10),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("add")),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident(String::from("x")),
            Token::Comma,
            Token::Ident(String::from("y")),
            Token::RParen,
            Token::LBrace,
            Token::Ident(String::from("x")),
            Token::Plus,
            Token::Ident(String::from("y")),
            Token::Semicolon,
            Token::RBrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("result")),
            Token::Assign,
            Token::Ident(String::from("add")),
            Token::LParen,
            Token::Ident(String::from("five")),
            Token::Comma,
            Token::Ident(String::from("ten")),
            Token::RParen,
            Token::Semicolon,
            Token::If,
            Token::LParen,
            Token::Integer(5),
            Token::LessThan,
            Token::Integer(10),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RBrace,
            Token::Else,
            Token::LBrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RBrace,
            Token::Integer(1),
            Token::NotEqual,
            Token::Integer(2),
            Token::Integer(1),
            Token::Equal,
            Token::Integer(1),
            Token::Str(String::from("foobar")),
            Token::Str(String::from("foo bar")),
            Token::LBracket,
            Token::Integer(1),
            Token::Comma,
            Token::Integer(2),
            Token::RBracket,
            Token::Semicolon,
            Token::LBrace,
            Token::Str(String::from("foo")),
            Token::Colon,
            Token::Str(String::from("bar")),
            Token::RBrace,
            Token::EndOfFile,
        ];
        let mut line = Lexer::new(sample_input);
        for t in tests {
            let tok = line.next_token();
            assert_eq!(tok, t);
        }
    }
}