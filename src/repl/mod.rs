use std::sync::{Arc, Mutex};
use std::io::{stdout, stdin, Write};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::evaluator::eval;

pub fn run() -> std::io::Result<()>{
    let env = Arc::new(crate::object::Environment::new());
    loop {
        print!("> ");
        let _ = stdout().flush();
        let mut input = String::new();
        stdin().read_line(&mut input)?;

        let l = Lexer::new(input.as_str());
        let mut p = Parser::new(l);
        let program = p.parse_program().expect("error parsing program");
        let obj = eval(crate::ast::Node::Program(program), env.clone());

        println!("{}", obj.inspect());
    }
}
