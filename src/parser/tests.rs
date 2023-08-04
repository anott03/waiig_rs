#[cfg(test)]

use crate::ast::Inspect;

#[test]
fn parse_let_statement() {
    use crate::parser::Parser;
    use crate::lexer::Lexer;
    use crate::ast;

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
        if let ast::Statement::LetStatement(ls) = prog.statements[0].clone() {
            if let Some(ast::Expression::IntegerLiteral(il)) = ls.value {
                assert_eq!(5, il.value);
            } else {
                panic!("value is not an IntegerLiteral");
            }
        } else {
            panic!("statement is not a LetStatement");
        }
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
fn parse_return_statement() {
    use crate::parser::Parser;
    use crate::lexer::Lexer;
    use crate::ast;

    let input = "return 5;
    return 17 * 3;
    return 993322;
    ";
    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    if let Some(prog) = p.parse_program() {
        if p.errors.len() != 0 {
            p.errors.iter().for_each(|e| {
                println!("{}", e);
            });
            panic!("there were errors");
        }
        assert!(prog.statements.len() == 3);

        if let ast::Statement::ReturnStatement(rs) = prog.statements[0].clone() {
            if let ast::Expression::IntegerLiteral(is) = rs.return_val {
                assert_eq!(5, is.value);
            }
        }

        if let ast::Statement::ReturnStatement(rs) = prog.statements[1].clone() {
            if let ast::Expression::InfixExpression(ie) = rs.return_val {
                assert_eq!("*", ie.operator);
                if let ast::Expression::IntegerLiteral(l) = *ie.left {
                    assert_eq!(17, l.value);
                }
                if let ast::Expression::IntegerLiteral(r) = *ie.right {
                    assert_eq!(3, r.value);
                }
            }
        }

        if let ast::Statement::ReturnStatement(rs) = prog.statements[2].clone() {
            if let ast::Expression::IntegerLiteral(is) = rs.return_val {
                assert_eq!(993322, is.value);
            }
        }
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
            if let ast::Expression::InfixExpression(ie) = es.expression {
                if let ast::Expression::InfixExpression(ie2) = *ie.left {
                    if let ast::Expression::IntegerLiteral(l) = *ie2.left {
                        assert_eq!(5, l.value);
                    }
                    if let ast::Expression::IntegerLiteral(r) = *ie2.right {
                        assert_eq!(5, r.value);
                    }
                    assert_eq!("+", ie2.operator);
                }
                if let ast::Expression::IntegerLiteral(r) = *ie.right {
                    assert_eq!(5, r.value);
                }
                assert_eq!("-", ie.operator);
            }
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
    use crate::token::Token;

    let input = "if (x < y) { x } else { y }";
    let l = Lexer::new(input);
    let mut p = Parser::new(l);

    if let Some(program) = p.parse_program() {
        if let ast::Statement::ExpressionStatement(es) = program.statements[0].clone() {
            if let ast::Expression::IfExpression(ie) = es.expression {
                if let ast::Expression::InfixExpression(infe) = *ie.condition {
                    if let ast::Expression::Identifier(l) = *infe.left {
                        assert_eq!(Token::IDENT(String::from("x")), l.token);
                    }
                    if let ast::Expression::Identifier(r) = *infe.right {
                        assert_eq!(Token::IDENT(String::from("y")), r.token);
                    }
                } else {
                    panic!("not an InfixExpression");
                }

                assert_eq!(1, ie.consequence.statements.len());
                if let ast::Statement::ExpressionStatement(ses) = ie.consequence.statements[0].clone() {
                    if let ast::Expression::Identifier(i) = ses.expression  {
                        assert_eq!(Token::IDENT(String::from("x")), i.token);
                    }
                } else {
                    panic!("not an ExpressionStatement");
                }

                if let Some(bs) = ie.alternative {
                    assert_eq!(1, bs.statements.len());
                    if let ast::Statement::ExpressionStatement(ses) = bs.statements[0].clone() {
                        if let ast::Expression::Identifier(i) = ses.expression {
                            assert_eq!(Token::IDENT(String::from("y")), i.token);
                        }
                    }
                } else {
                    panic!("no alternative");
                }
            } else {
                panic!("not an IfExpression");
            }
        }
    }
}

#[test]
fn parse_function_literal() {
    use crate::parser::Parser;
    use crate::lexer::Lexer;
    use crate::ast;

    let input = "fn() {};";
    let l = Lexer::new(input);
    let mut p = Parser::new(l);

    if let Some(program) = p.parse_program() {
        assert!(p.errors.len() == 0);
        assert!(program.statements.len() == 1);
        if let ast::Statement::ExpressionStatement(es) = program.statements[0].clone() {
            if let ast::Expression::FunctionLiteral(fl) = es.expression {
                assert!(fl.parameters.len() == 0);
                assert!(fl.body.statements.len() == 0);
            } else {
                panic!("expression not a FunctionLiteral");
            }
        } else {
            panic!("statement not an ExpressionStatement");
        }
    }
}

#[test]
fn parse_function_literal2() {
    use crate::parser::Parser;
    use crate::lexer::Lexer;
    use crate::ast;

    let input = "fn(x, y) { x == y };";
    let l = Lexer::new(input);
    let mut p = Parser::new(l);

    if let Some(program) = p.parse_program() {
        assert!(p.errors.len() == 0);
        assert!(program.statements.len() == 1);
        if let ast::Statement::ExpressionStatement(es) = program.statements[0].clone() {
            if let ast::Expression::FunctionLiteral(fl) = es.expression {
                assert!(fl.parameters.len() == 2);
                assert!(fl.body.statements.len() == 1);
            } else {
                panic!("expression not a FunctionLiteral");
            }
        } else {
            panic!("statement not an ExpressionStatement");
        }
    }
}

#[test]
fn parse_call_expression() {
    use crate::parser::Parser;
    use crate::lexer::Lexer;
    use crate::ast;

    let input = "my_function(2 + 3, 17)";
    let l = Lexer::new(input);
    let mut p = Parser::new(l);

    if let Some(program) = p.parse_program() {
        assert!(p.errors.len() == 0);
        assert!(program.statements.len() == 1);

        if let ast::Statement::ExpressionStatement(es) = program.statements[0].clone() {
            if let ast::Expression::CallExpression(ce) = es.expression {
                assert_eq!("my_function", ce.function.to_string());
                assert_eq!(2, ce.arguments.len());
            }
        }
    }
}
