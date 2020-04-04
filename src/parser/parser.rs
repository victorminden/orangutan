use crate::lexer::Lexer;
use crate::ast::{Program, Statement, Expression};
use crate::token::Token;
use crate::parser::{ParseError, Precedence, token_precedence};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    errors: Vec<ParseError>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Parser {
        Parser { lexer, errors: Vec::new() }
    }

    pub fn print_errors(self) {
        for err in self.errors {
            println!("Error: {:?}", err );
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut statements = vec![];
        while *self.lexer.peek_token() != Token::EndOfFile {
            match self.parse_statement() {
                Ok(statement) => statements.push(statement),
                Err(error) => {
                    self.errors.push(error);
                }
            }
        }
        Ok(Program{ statements })
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        match &*self.lexer.peek_token() {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn expect_peek(&mut self, expected: Token) -> Result<(), ParseError> {
        // Check the variant of the enum without the value.
        let got = self.lexer.next_token();
        if std::mem::discriminant(&got) == std::mem::discriminant(&expected) {
               return Ok(());
        } 
        
        match expected {
            Token::Let => Err(ParseError::ExpectedLet(got)),
            Token::Assign => Err(ParseError::ExpectedAssign(got)),
            _ => Err(ParseError::UnknownError),
        }

    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParseError> {
        // Advance past the "Return".
        self.expect_peek(Token::Return)?;
        // TODO: Implement expression parsing.
        while self.lexer.next_token() != Token::Semicolon {
            continue;
        }
        return Ok(Statement::Return(Expression::Null));
    }

    fn parse_let_statement(&mut self) -> Result<Statement, ParseError> {
        // Advance past the "Let".
        self.expect_peek(Token::Let)?;
        // Get the name of the identifier.
        let name = match self.lexer.next_token() {
            Token::Ident(ident) => ident,
            got => {
                return Err(ParseError::ExpectedIdent(got));
            },
        };
        // Advance past the "Assign".
        self.expect_peek(Token::Assign)?;
        // TODO: Implement expression parsing.
        while self.lexer.next_token() != Token::Semicolon {
            continue;
        }
        return Ok(Statement::Let(name, Expression::Null));
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParseError> {
        let expression = self.parse_expression(Precedence::Lowest)?;
        // Optional semicolon.
        if *self.lexer.peek_token() == Token::Semicolon {
            self.lexer.next_token();
        }
        Ok(Statement::Expression(expression))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, ParseError> {
        // Match left/primary expression.
        let mut expr = match *self.lexer.peek_token() {
            Token::Ident(_) => self.parse_identifier()?, 
            Token::Integer(_) => self.parse_integer_literal()?,
            Token::Bang | Token::Minus => self.parse_prefix_expression()?,
            // TODO: Treat the following tokens explicitly.
            Token::True |
            Token::False |
            Token::LParen |
            Token::RParen |
            Token::LBrace |
            Token::RBrace |
            Token::If |
            Token::Function => self.parse_identifier()?,
            _ => { 
                self.lexer.next_token();
                return Err(ParseError::UnexpectedToken); 
            },
        };
        // Repeatedly look for infix tokens.
        // TODO: Finish additional tests.
        while *self.lexer.peek_token() != Token::Semicolon &&
            token_precedence(&*self.lexer.peek_token()) > precedence {
                expr = match *self.lexer.peek_token() {
                    Token::Plus |
                    Token::Minus |
                    Token::Asterisk |
                    Token::Slash |
                    Token::Equal |
                    Token::NotEqual |
                    Token::LessThan |
                    Token::GreaterThan => self.parse_infix_expression(expr)?,
                    _ => { return Ok(expr); },
                };
        }
        Ok(expr)
    }

    fn parse_identifier(&mut self) -> Result<Expression, ParseError> {
        match self.lexer.next_token() {
            Token::Ident(name) => Ok(Expression::Ident(name)),
            other => Err(ParseError::ExpectedIdent(other)),
        }
    }

    fn parse_integer_literal(&mut self) -> Result<Expression, ParseError> {
        match self.lexer.next_token() {
            Token::Integer(int) => Ok(Expression::IntegerLiteral(int)),
            other => Err(ParseError::ExpectedInteger(other)),
        }
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, ParseError> {
        match self.lexer.next_token() {
            prefix 
            if (prefix == Token::Minus) | (prefix ==Token::Bang) => {
                let expr = self.parse_expression(Precedence::Prefix)?;
                Ok(Expression::Prefix(prefix, Box::new(expr)))
            },
            other => Err(ParseError::ExpectedPrefix(other)),
        }
    }

    fn parse_infix_expression(
        &mut self, left_expr: Expression) -> Result<Expression, ParseError> {
        let token = self.lexer.next_token();
        let right_expr = self.parse_expression(token_precedence(&token))?;
        Ok(Expression::Infix(Box::new(left_expr), token, Box::new(right_expr)))
    }
}
