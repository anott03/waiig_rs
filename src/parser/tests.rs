#[cfg(test)]

#[test]
fn parse_let_statement() {
    use crate::parser::Parser;
    use crate::lexer::Lexer;

    let input = "let x = 5;";
    let l = Lexer::new(input.to_string());
    let mut p = Parser::new(l);
    if let Some(prog) = p.parse_program() {
        println!("{:?}", prog.statements);
        assert!(prog.statements.len() == 1);
    }
    assert!(false);
}
