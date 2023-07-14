#[cfg(test)]

#[test]
fn parse_let_statement() {
    use crate::parser::Parser;
    use crate::lexer::Lexer;

    let input = "let x = 5;";
    let l = Lexer::new(input.to_string());
    let p = Parser::new(l);
    p.parse_program();
}
