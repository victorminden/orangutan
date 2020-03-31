use super::*;
use crate::lexer::Lexer;
use crate::ast::{Program, Statement, Expression};
use crate::token::Token;

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

#[test]
fn identifier_statement_test() -> Result<(), ParseError> {
    let input = "foobar;";
    
    let mut parser = Parser::new(Lexer::new(input));
    let program = parser.parse_program()?;
    parser.print_errors();
    assert_eq!(program.statements.len(), 1);
   
    if let Statement::Expression(exp) = &program.statements[0] {
        if let Expression::Ident(name) = exp {
            assert_eq!(name, "foobar");
        } else {
            assert!(false);
        }
    }
    else {
        assert!(false);
    }

    Ok(())
}