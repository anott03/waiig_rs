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
