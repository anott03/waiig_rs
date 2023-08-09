#[cfg(test)]

#[test]
fn eval_integer_expression() {
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::object::*;
    use crate::evaluator::eval;

    let input = "5";
    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program().unwrap();
    let mut env = Environment::new();
    let obj = eval(crate::ast::Node::Program(program), &mut env);
    if let Object::Integer(i) = obj {
        assert_eq!(5, i);
    } else {
        panic!("obj is not an Integer");
    }
}

#[test]
fn eval_boolean_expression() {
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::object::*;
    use crate::evaluator::eval;

    let input = "true";
    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program().unwrap();
    let mut env = Environment::new();
    let obj = eval(crate::ast::Node::Program(program), &mut env);
    if let Object::Boolean(b) = obj {
        assert_eq!(true, b);
    } else {
        panic!("obj is not an Boolean");
    }
}

#[test]
fn eval_bang_expression() {
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::object::*;
    use crate::evaluator::eval;

    let input = "!true";
    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program().unwrap();
    let mut env = Environment::new();
    let obj = eval(crate::ast::Node::Program(program), &mut env);
    if let Object::Boolean(b) = obj {
        assert_eq!(false, b);
    } else {
        panic!("obj is not an Boolean");
    }
}

#[test]
fn eval_minus_expression() {
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::object::*;
    use crate::evaluator::eval;

    let input = "-10";
    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program().unwrap();
    let mut env = Environment::new();
    let obj = eval(crate::ast::Node::Program(program), &mut env);
    if let Object::Integer(i) = obj {
        assert_eq!(-10, i);
    } else {
        panic!("obj is not an Integer");
    }
}

#[test]
fn eval_infix_int_expression() {
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::object::*;
    use crate::evaluator::eval;

    let tests = vec![
        (String::from("3 * 6"), 18),
        (String::from("6 / 3"), 2),
        (String::from("9 + 9"), 18),
        (String::from("9 - 9"), 0),
    ];

    tests.iter().for_each(|(i, o)| {
        let mut p = Parser::new(Lexer::new(i.as_str()));
        let program = p.parse_program().unwrap();
        let mut env = Environment::new();
        let obj = eval(crate::ast::Node::Program(program), &mut env);
        println!("{:?}", obj);
        if let Object::Integer(i) = obj {
            assert_eq!(*o, i);
        } else {
            panic!("obj is not an Integer");
        }
    });
}

#[test]
fn eval_infix_bool_expression() {
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::object::*;
    use crate::evaluator::eval;

    let tests = vec![
        (String::from("1 == 1"), true),
        (String::from("6 < 3"), false),
        (String::from("4 != 9"), true),
        (String::from("9 > 9"), false),
        (String::from("true == true"), true),
        (String::from("true != true"), false),
        (String::from("false == true"), false),
        (String::from("false != true"), true),
        (String::from("false < 17"), false),
    ];

    tests.iter().for_each(|(i, o)| {
        let mut p = Parser::new(Lexer::new(i.as_str()));
        let program = p.parse_program().unwrap();
        let mut env = Environment::new();
        let obj = eval(crate::ast::Node::Program(program), &mut env);
        println!("{} {:?}", i, obj);
        if let Object::Boolean(i) = obj {
            assert_eq!(*o, i);
        } else {
            panic!("obj is not an Boolean");
        }
    });
}

#[test]
fn eval_if_expression() {
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::object::*;
    use crate::evaluator::eval;

    let tests = vec![
        (String::from("if (0) { true } else { false }"), false),
        (String::from("if (1) { true } else { false }"), true),
    ];

    tests.iter().for_each(|(i, o)| {
        let mut p = Parser::new(Lexer::new(i.as_str()));
        let program = p.parse_program().unwrap();
        let mut env = Environment::new();
        let obj = eval(crate::ast::Node::Program(program), &mut env);
        println!("{} {:?}", i, obj);
        if let Object::Boolean(i) = obj {
            assert_eq!(*o, i);
        } else {
            panic!("obj is not an Boolean");
        }
    });
}

#[test]
fn eval_return_statement() {
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::object::*;
    use crate::evaluator::eval;

    let tests = vec![
        (String::from("return 5"), 5),
        (String::from("return 10"), 10),
        (String::from("if (10 > 1) {
            if (10 > 1) {
                return 10;
            }
            return 1;
        }"), 10)
    ];

    tests.iter().for_each(|(i, o)| {
        let mut p = Parser::new(Lexer::new(i.as_str()));
        let program = p.parse_program().unwrap();
        let mut env = Environment::new();
        let obj = eval(crate::ast::Node::Program(program), &mut env);
        println!("{} {:?}", i, obj);
        if let Object::Integer(i) = obj {
            assert_eq!(*o, i);
        } else {
            println!("{:?}", obj);
            panic!("obj is no an Integer");
        }
    });
}

#[test]
fn error_handling() {
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::object::*;
    use crate::evaluator::eval;

    let tests = vec![
        ("5 + true;", "type mismatch: INTEGER + BOOLEAN"),
        ("5 + true; 5;", "type mismatch: INTEGER + BOOLEAN"),
        ("-true", "unknown operator: -BOOLEAN"),
        ("true + false", "unknown operator: BOOLEAN + BOOLEAN"),
        ("5; true + false; 5", "unknown operator: BOOLEAN + BOOLEAN"),
        ("if (10 > 1) { return true + false; }", "unknown operator: BOOLEAN + BOOLEAN"),
    ];

    tests.iter().for_each(|(i, o)| {
        let mut p = Parser::new(Lexer::new(i));
        let program = p.parse_program().unwrap();
        let mut env = Environment::new();
        let obj = eval(crate::ast::Node::Program(program), &mut env);
        if let Object::Error(e) = obj {
            assert_eq!(*o, e);
        } else {
            panic!("obj is not an Error");
        }
    });
}
