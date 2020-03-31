use crate::lexer::Lexer;
use crate::ast::{Program, Statement, Expression};
use crate::token::Token;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken,
    ExpectedIdent(Token),
    ExpectedLet(Token),
    ExpectedAssign(Token),
    UnknownError,
}

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
            _ => {
                self.lexer.next_token();
                Err(ParseError::UnexpectedToken)
            },
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

    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        Ok(Expression::Null)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn let_statement_test() -> Result<(), ParseError> {
        let input = "
        let x = 5;
        let y = 10;
        let foobar = x + y;
        ";

        let tests = vec![
            "x",
            "y",
            "foobar",
        ];
        
        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse_program()?;
        parser.print_errors();

        for (expected_name, statement) in tests.iter().zip(program.statements.iter()) {
            match statement {
                Statement::Let(name, _) => {
                    assert_eq!(name, expected_name);
                },
                _ => assert!(false),
            }
        }

        Ok(())
    }

    #[test]
    fn return_statement_test() -> Result<(), ParseError> {
        let input = "
        return 5;
        return 10;
        return 9932;
        ";

        let tests = vec![
            5,
            10,
            9932,
        ];
        
        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse_program()?;
        parser.print_errors();
        let mut count = 0;
        for (expected_name, statement) in tests.iter().zip(program.statements.iter()) {
            match statement {
                Statement::Return(_) => { count += 1; },
                _ => assert!(false),
            }
        }
        assert_eq!(count, 3);

        Ok(())
    }
}