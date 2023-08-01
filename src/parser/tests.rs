#[cfg(test)]

#[test]
fn parse_let_statement() {
    use crate::parser::Parser;
    use crate::lexer::Lexer;

    let input = "let x = 5;";
    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    if let Some(prog) = p.parse_program() {
        if p.errors.len() != 0 {
            p.errors.iter().for_each(|e| {
                println!("{}", e);
            });
            panic!("there were errors");
        }
        println!("{:?}", prog.statements);
        assert!(prog.statements.len() == 1);
    }
}

#[test]
fn peek_error() {
    use crate::parser::Parser;
    use crate::lexer::Lexer;

    let input = "let = 5;";
    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    if let Some(_) = p.parse_program() {
        assert!(p.errors.len() > 0);
        // if p.errors.len() != 0 {
        //     p.errors.iter().for_each(|e| {
        //         println!("{}", e);
        //     });
        //     panic!();
        // }
    }
}

#[test]
fn return_statement() {
    use crate::parser::Parser;
    use crate::lexer::Lexer;

    let input = "return 5;
    return 10;
    return 993322;
    ";
    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    if let Some(prog) = p.parse_program() {
        assert!(p.errors.len() == 0);
        assert!(prog.statements.len() == 3);
    }
}

#[test]
fn parse_identifier() {
    use crate::parser::Parser;
    use crate::lexer::Lexer;
    use crate::ast;

    let input = "foobar;";
    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    if let Some(program) = p.parse_program() {
        assert!(p.errors.len() == 0);
        assert!(program.statements.len() == 1);
        let stmt = program.statements[0].clone();
        if let ast::Statement::ExpressionStatement(es) = stmt {
            if let ast::Expression::Identifier(ident) = es.expression {
                assert_eq!(ident.value, "foobar");
            } else {
                panic!("ExpressionStatement expression is not an identifier");
            }
        } else {
            panic!("statement not an ExpressionStatement");
        }
    }
}

#[test]
fn parse_integer_literal() {
    use crate::parser::Parser;
    use crate::lexer::Lexer;
    use crate::ast;
    
    let input = "5;";
    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    
    if let Some(program) = p.parse_program() {
        assert!(p.errors.len() == 0);
        assert!(program.statements.len() == 1);
        let stmt = program.statements[0].clone();
        if let ast::Statement::ExpressionStatement(es) = stmt {
            if let ast::Expression::IntegerLiteral(int) = es.expression {
                assert_eq!(int.value, 5);
            } else {
                panic!("ExpressionStatement expression is not an integer literal");
            }
        } else {
            panic!("statement not an ExpressionStatement");
        }
    }
}

#[test]
fn parse_prefix_expression() {
    use crate::parser::Parser;
    use crate::lexer::Lexer;
    use crate::ast;

    let input = "-5;";
    let l = Lexer::new(input);
    let mut p = Parser::new(l);

    if let Some(program) = p.parse_program() {
        assert!(p.errors.len() == 0);
        assert!(program.statements.len() == 1);
        let stmt = program.statements[0].clone();
        if let ast::Statement::ExpressionStatement(es) = stmt {
            if let ast::Expression::PrefixExpression(pe) = es.expression {
                assert_eq!("-", pe.operator);
                let right = *pe.right;
                if let ast::Expression::IntegerLiteral(i) = right {
                    assert_eq!(5, i.value);
                } else {
                    panic!("right is not an IntegerLiteral");
                }
            } else {
                panic!("ExpressionStatement expression is not a PrefixExpression");
            }
        } else {
            panic!("statement is not an ExpressionStatement");
        }
    }
}

#[test]
fn parse_infix_expression() {
    use crate::parser::Parser;
    use crate::lexer::Lexer;
    use crate::ast;

    let input = "5 + 5;";
    let l = Lexer::new(input);
    let mut p = Parser::new(l);

    if let Some(program) = p.parse_program() {
        assert!(p.errors.len() == 0);
        assert!(program.statements.len() == 1);

        let statement = program.statements[0].clone();
        if let ast::Statement::ExpressionStatement(es) = statement {
            if let ast::Expression::InfixExpression(ie) = es.expression {
                assert_eq!("+", ie.operator);
                let right = *ie.right;
                let left = *ie.left;

                if let ast::Expression::IntegerLiteral(l) = left {
                    assert_eq!(5, l.value);
                } else {
                    panic!("left value is not 5");
                }

                if let ast::Expression::IntegerLiteral(r) = right {
                    assert_eq!(5, r.value);
                } else {
                    panic!("right value is not 5");
                }
            } else {
                panic!("ExpressionStatement expression is not an InfixExpression");
            }
        } else {
            panic!("statement is not an ExpressionStatement");
        }
    }
}

// TODO: test a bunch of other expressions to ensure priority works as intended
// a + b + c
// a + b * c
// (a + b) * c
// etc.

#[test]
fn parse_boolean() {
    use crate::parser::Parser;
    use crate::lexer::Lexer;
    use crate::ast;

    let input = "true != false;";
    let l = Lexer::new(input);
    let mut p = Parser::new(l);

    if let Some(program) = p.parse_program() {
        assert!(p.errors.len() == 0);
        assert!(program.statements.len() == 1);

        let statement = program.statements[0].clone();
        if let ast::Statement::ExpressionStatement(es) = statement {
            if let ast::Expression::InfixExpression(ie) = es.expression {
                assert_eq!("!=", ie.operator);
                let right = *ie.right;
                let left = *ie.left;

                if let ast::Expression::Boolean(l) = left {
                    assert_eq!(true, l.value);
                } else {
                    panic!("left value is not true");
                }

                if let ast::Expression::Boolean(r) = right {
                    assert_eq!(false, r.value);
                } else {
                    panic!("right value is not false");
                }
            } else {
                panic!("ExpressionStatement expression is not an InfixExpression");
            }
        } else {
            panic!("statement is not an ExpressionStatement");
        }
    }
}

#[test]
fn parse_grouped_expression() {
    use crate::parser::Parser;
    use crate::lexer::Lexer;
    use crate::ast;

    let input = "(5 + 5) - 5";
    let l = Lexer::new(input);
    let mut p = Parser::new(l);

    if let Some(program) = p.parse_program() {
        println!("{:?}", p.errors);

        let statement = program.statements[0].clone();
        if let ast::Statement::ExpressionStatement(es) = statement {
            println!("{:?}", es.expression);
        } else {
            panic!("statement is not an ExpressionStatement");
        }
    }
}

#[test]
fn parse_if_expression() {
    use crate::parser::Parser;
    use crate::lexer::Lexer;
    use crate::ast;

    let input = "if (x < y) { x }";
    let l = Lexer::new(input);
    let mut p = Parser::new(l);

    if let Some(program) = p.parse_program() {
        println!("{:?}", p.errors);
        assert!(p.errors.len() == 0);
    }
}

#[test]
fn parse_if_else_expression() {
    use crate::parser::Parser;
    use crate::lexer::Lexer;
    use crate::ast;

    let input = "if (x < y) { x } else { y }";
    let l = Lexer::new(input);
    let mut p = Parser::new(l);

    if let Some(program) = p.parse_program() {
        println!("{:?}", p.errors);
        assert!(p.errors.len() == 0);
    }
}
