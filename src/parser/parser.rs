use crate::lexer::Lexer;
use crate::ast::{Program, Statement, Expression};
use crate::token::Token;
use crate::parser::{ParseError, Precedence};

type PrefixParseFn = fn(&mut Parser) -> Result<Expression, ParseError>;
type InfixParseFn = fn(&mut Parser, Expression) -> Result<Expression, ParseError>;

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
        // Hack to check the variant of the enum without the value.
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

    // TODO: Determine appropriate parameters to this and infix flavor.
    fn prefix_parse_fn(&mut self) -> Option<PrefixParseFn> {
        None
    }

    fn infix_parse_fn(&mut self) -> Option<InfixParseFn> {
        None
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
        Ok(Expression::Null)
    }
}
