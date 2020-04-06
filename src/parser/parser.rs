use crate::lexer::Lexer;
use crate::ast::{Program, Statement, Expression, BlockStatement};
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
            Token::RParen => Err(ParseError::ExpectedRParen(got)),
            _ => Err(ParseError::UnknownError),
        }

    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParseError> {
        // Advance past the "Return".
        self.expect_peek(Token::Return)?;
        let expr = self.parse_expression(Precedence::Lowest)?;
        // Advance past the required semicolon.
        self.expect_peek(Token::Semicolon)?;
        return Ok(Statement::Return(expr));
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
        let expr = self.parse_expression(Precedence::Lowest)?;
        // Advance past the required semicolon.
        self.expect_peek(Token::Semicolon)?;
        return Ok(Statement::Let(name, expr));
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParseError> {
        let expression = self.parse_expression(Precedence::Lowest)?;
        // Optional semicolon.
        if *self.lexer.peek_token() == Token::Semicolon {
            self.lexer.next_token();
        }
        Ok(Statement::Expression(expression))
    }

    fn parse_boolean_literal(&mut self) -> Result<Expression, ParseError> {
        match self.lexer.next_token() {
            Token::True => Ok(Expression::BooleanLiteral(true)),
            Token::False => Ok(Expression::BooleanLiteral(false)),
            other => Err(ParseError::ExpectedBoolean(other)),
        }
    }

    fn parse_grouped_expression(&mut self) -> Result<Expression, ParseError> {
        self.expect_peek(Token::LParen)?;
        let exp = self.parse_expression(Precedence::Lowest)?;
        self.expect_peek(Token::RParen)?;
        Ok(exp)
    }

    fn parse_block_statement(&mut self) -> Result<BlockStatement, ParseError> {
        self.expect_peek(Token::LBrace)?;
        let mut statements = vec![];
        while *self.lexer.peek_token() != Token::RBrace {
            if *self.lexer.peek_token() == Token::EndOfFile {
                return Err(ParseError::UnexpectedToken(Token::EndOfFile));
            }
            statements.push(self.parse_statement()?);
        }
        self.expect_peek(Token::RBrace)?;
        Ok(BlockStatement{ statements })
    }

    fn parse_if_expression(&mut self) -> Result<Expression, ParseError> {
        self.expect_peek(Token::If)?;
        let condition = self.parse_grouped_expression()?;
        let consequence = self.parse_block_statement()?;
        let alternative = match *self.lexer.peek_token() {
            Token::Else => {
                self.lexer.next_token();
                Some(self.parse_block_statement()?)
            },
            _ => None,
        };
        Ok(Expression::If(Box::new(condition), consequence, alternative))

    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, ParseError> {
        // Match left/primary expression.
        let mut expr = match *self.lexer.peek_token() {
            Token::Ident(_) => self.parse_identifier()?, 
            Token::Integer(_) => self.parse_integer_literal()?,
            Token::Bang | Token::Minus => self.parse_prefix_expression()?,
            Token::True | Token::False => self.parse_boolean_literal()?,
            Token::LParen => self.parse_grouped_expression()?,
            Token::If => self.parse_if_expression()?,
            // TODO: Treat the following tokens explicitly.
            Token::RParen |
            Token::LBrace |
            Token::RBrace |
            Token::Function |
            _ => { 
                let other = self.lexer.next_token();
                return Err(ParseError::UnexpectedToken(other)); 
            },
        };
        // Repeatedly look for infix tokens.
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
