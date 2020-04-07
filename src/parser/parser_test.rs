use super::*;
use crate::lexer::Lexer;
use crate::ast::{Statement, Expression};
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
            _ => panic!(),
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
    
    let mut parser = Parser::new(Lexer::new(input));
    let program = parser.parse_program()?;
    parser.print_errors();
    let mut count = 0;
    for statement in program.statements {
        match statement {
            Statement::Return(_) => { count += 1; },
            _ => panic!(),
        }
    }
    assert_eq!(count, 3);

    Ok(())
}

#[test]
fn let_and_return_statements_with_expressions_test() -> Result<(), ParseError> {
    let input = "
    let a = b + (c - d) / e;
    return a / b;
    ";
    
    let expected = vec![
        "let a = (b + ((c - d) / e));",
        "return (a / b);",
    ];

    let mut parser = Parser::new(Lexer::new(input));
    let program = parser.parse_program()?;
    parser.print_errors();
    assert_eq!(program.statements.len(), expected.len());

    for (expected, statement) in 
    expected.iter().zip(program.statements.iter()) {
        assert_eq!(&statement.to_string(), expected);
    }

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
           panic!();
        }
    }
    else {
        panic!();
    }

    Ok(())
}

#[test]
fn integer_literal_statement_test() -> Result<(), ParseError> {
    let input = "5";
    
    let mut parser = Parser::new(Lexer::new(input));
    let program = parser.parse_program()?;
    parser.print_errors();
    assert_eq!(program.statements.len(), 1);
   
    if let Statement::Expression(exp) = &program.statements[0] {
        if let Expression::IntegerLiteral(val) = exp {
            assert_eq!(*val, 5);
        } else {
            panic!();
        }
    }
    else {
        panic!();
    }

    Ok(())
}

#[test]
fn prefix_statement_test() -> Result<(), ParseError> {
    let input = "!5; -15;";
    let expected = vec![
        (Token::Bang, 5),
        (Token::Minus, 15),
    ];

    let mut parser = Parser::new(Lexer::new(input));
    let program = parser.parse_program()?;
    parser.print_errors();
    assert_eq!(program.statements.len(), 2);

    for ((expected_prefix, expected_literal), statement) in 
    expected.iter().zip(program.statements.iter()) {
        let expression = match statement {
            Statement::Expression(exp) => exp,
            _ => panic!(),
        };
        let (prefix, tail_expression) = match expression {
            Expression::Prefix(pref, tail_expr) => (pref, tail_expr),
            _ => panic!(),
        };
        assert_eq!(prefix, expected_prefix);
        let literal = match **tail_expression {
            Expression::IntegerLiteral(integer) => integer,
            _ => panic!(),
        };
        assert_eq!(literal, *expected_literal);
    }

    Ok(())
}

#[test]
fn infix_statement_test() -> Result<(), ParseError> {
    let input = "
    5 + 7; 
    5 - 7;
    5 * 7
    5 / 7
    5 > 7
    5 < 7
    5 == 7
    5 != 7";

    let expected = vec![
        (5, Token::Plus, 7),
        (5, Token::Minus, 7),
        (5, Token::Asterisk, 7),
        (5, Token::Slash, 7),
        (5, Token::GreaterThan, 7),
        (5, Token::LessThan, 7),
        (5, Token::Equal, 7),
        (5, Token::NotEqual, 7),
    ];

    let mut parser = Parser::new(Lexer::new(input));
    let program = parser.parse_program()?;
    parser.print_errors();
    assert_eq!(program.statements.len(), 8);

    for ((expected_left, expected_infix, expected_right), statement) in 
    expected.iter().zip(program.statements.iter()) {
        let expression = match statement {
            Statement::Expression(exp) => exp,
            _ => panic!(),
        };
        let (left, infix, right) = match expression {
            Expression::Infix(left, infix, right) => (left, infix, right),
            _ => panic!(),
        };
        assert_eq!(infix, expected_infix);
        let left_literal = match **left {
            Expression::IntegerLiteral(integer) => integer,
            _ => panic!(),
        };
        assert_eq!(left_literal, *expected_left);
        let right_literal = match **right {
            Expression::IntegerLiteral(integer) => integer,
            _ => panic!(),
        };
        assert_eq!(right_literal, *expected_right);
    }

    Ok(())
}

#[test]
fn operator_precedence_test() -> Result<(), ParseError> {
    let input = "
    -a * b; 
    !-a;
    a+b+c
    a+b-c
    a*b*c
    a*b/c
    a+b/c
    a+b*c+d/e-f
    3 < 5 == false
    3 < 5 == true
    !true != false
    1 + (2 + 3) + 4;
    (5 + 5) * 2
    2 / (5 + 5)
    !(true == true)
    a + add(b * c) + d
    add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))
    add(a + b + c * d / f + g)
    ";
    
    let expected = vec![
        "((-a) * b);",
        "(!(-a));",
        "((a + b) + c);",
        "((a + b) - c);",
        "((a * b) * c);",
        "((a * b) / c);",
        "(a + (b / c));",
        "(((a + (b * c)) + (d / e)) - f);",
        "((3 < 5) == false);",
        "((3 < 5) == true);",
        "((!true) != false);",
        "((1 + (2 + 3)) + 4);",
        "((5 + 5) * 2);",
        "(2 / (5 + 5));",
        "(!(true == true));",
        "((a + add((b * c))) + d);",
        "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)));",
        "add((((a + b) + ((c * d) / f)) + g));",
    ];

    let mut parser = Parser::new(Lexer::new(input));
    let program = parser.parse_program()?;
    parser.print_errors();
    assert_eq!(program.statements.len(), expected.len());

    for (expected, statement) in 
    expected.iter().zip(program.statements.iter()) {
        assert_eq!(&statement.to_string(), expected);
    }

    Ok(())
}

#[test]
fn boolean_literal_statement_test() -> Result<(), ParseError> {
    let input = "false 
    true";
    
    let expected = vec![
        "false;",
        "true;",
    ];

    let mut parser = Parser::new(Lexer::new(input));
    let program = parser.parse_program()?;
    parser.print_errors();
    assert_eq!(program.statements.len(), expected.len());

    for (expected, statement) in 
    expected.iter().zip(program.statements.iter()) {
        assert_eq!(&statement.to_string(), expected);
    }

    Ok(())
}

#[test]
fn if_statement_test() -> Result<(), ParseError> {
    let input = "if(x<y){x}";

    let mut parser = Parser::new(Lexer::new(input));
    let program = parser.parse_program()?;
    parser.print_errors();
    assert_eq!(program.statements.len(), 1);

    
    if let Statement::Expression(expr) = &program.statements[0] {
        if let Expression::If(condition, consequence, None) = expr {
            assert_eq!(condition.to_string(), "(x < y)");
            assert_eq!(consequence.to_string(), "{ x; }");
            Ok(())
        } else {
            panic!();
        }
    } else {
        panic!();
    }
}

#[test]
fn if_statement_with_else_test() -> Result<(), ParseError> {
    let input = "if(x<y){x}else{z+7}";

    let mut parser = Parser::new(Lexer::new(input));
    let program = parser.parse_program()?;
    parser.print_errors();
    assert_eq!(program.statements.len(), 1);

    
    if let Statement::Expression(expr) = &program.statements[0] {
        if let Expression::If(condition, consequence, Some(alt_bs)) = expr {
            assert_eq!(condition.to_string(), "(x < y)");
            assert_eq!(consequence.to_string(), "{ x; }");
            assert_eq!(alt_bs.to_string(), "{ (z + 7); }");
            Ok(())
        } else {
            panic!();
        }
    } else {
        panic!();
    }
}

#[test]
fn function_literal_statement_test() -> Result<(), ParseError> {
    let input = "fn(x,y){return x+y;}";

    let mut parser = Parser::new(Lexer::new(input));
    let program = parser.parse_program()?;
 
    parser.print_errors();
    assert_eq!(program.statements.len(), 1);
    
    if let Statement::Expression(expr) = &program.statements[0] {
        if let Expression::FunctionLiteral(parameters, body) = expr {
            assert_eq!(parameters.join(", ").to_string(), "x, y");
            assert_eq!(body.to_string(), "{ return (x + y); }");
            Ok(())
        } else {
            panic!();
        }
    } else {
        panic!();
    }
}

#[test]
fn function_parameter_edge_case_test() -> Result<(), ParseError> {
    let input = "fn(){}
    fn(x){};
    fn(x,y,z){}";
    let expected = vec!["fn() {  };", "fn(x) {  };", "fn(x, y, z) {  };"];

    let mut parser = Parser::new(Lexer::new(input));
    let program = parser.parse_program()?;
 
    parser.print_errors();
    assert_eq!(program.statements.len(), 3);
    
    for (expected, statement) in expected.iter().zip(program.statements.iter()) {
        assert_eq!(&statement.to_string(), expected);
    }
    Ok(())
}

#[test]
fn call_expression_test() -> Result<(), ParseError> {
    let input = "add(1, 2*3, 4+5+6)";
    let expected = "add(1, (2 * 3), ((4 + 5) + 6));";

    let mut parser = Parser::new(Lexer::new(input));
    let program = parser.parse_program()?;
 
    parser.print_errors();
    assert_eq!(program.statements.len(), 1);
    assert_eq!(&program.statements[0].to_string(), expected);

    Ok(())
}

