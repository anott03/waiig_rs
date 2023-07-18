#[cfg(test)]

#[test]
fn parse_let_statement() {
    use crate::parser::Parser;
    use crate::lexer::Lexer;

    let input = "let x = 5;";
    let l = Lexer::new(input.to_string());
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
    let l = Lexer::new(input.to_string());
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
    let l = Lexer::new(input.to_string());
    let mut p = Parser::new(l);
    if let Some(prog) = p.parse_program() {
        assert!(p.errors.len() == 0);
        assert!(prog.statements.len() == 3);
    }
}
