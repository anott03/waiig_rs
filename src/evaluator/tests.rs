#[cfg(test)]

#[test]
fn eval_integer_expression() {
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::object::Object;
    use crate::evaluator::eval;

    let input = "5";
    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program().unwrap();

    eval(program);
}
