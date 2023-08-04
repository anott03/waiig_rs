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

    let obj = eval(crate::ast::Node::Program(program));
    if let Object::Integer(i) = obj {
        assert_eq!(5, i.value);
    } else {
        panic!("obj is not an Integer");
    }
}

#[test]
fn eval_boolean_expression() {
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::object::Object;
    use crate::evaluator::eval;

    let input = "true";
    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program().unwrap();

    let obj = eval(crate::ast::Node::Program(program));
    if let Object::Boolean(b) = obj {
        assert_eq!(true, b.value);
    } else {
        panic!("obj is not an Boolean");
    }
}

#[test]
fn eval_bang_expression() {
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::object::Object;
    use crate::evaluator::eval;

    let input = "!true";
    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program().unwrap();

    let obj = eval(crate::ast::Node::Program(program));
    if let Object::Boolean(b) = obj {
        assert_eq!(false, b.value);
    } else {
        panic!("obj is not an Boolean");
    }
}

#[test]
fn eval_minus_expression() {
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::object::Object;
    use crate::evaluator::eval;

    let input = "-10";
    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program().unwrap();

    let obj = eval(crate::ast::Node::Program(program));
    if let Object::Integer(i) = obj {
        assert_eq!(-10, i.value);
    } else {
        panic!("obj is not an Integer");
    }
}
