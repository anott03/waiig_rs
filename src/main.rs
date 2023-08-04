#![allow(dead_code)]

mod token;
mod lexer;
mod parser;
mod ast;
mod object;
mod evaluator;

use crate::parser::Parser;
use crate::lexer::Lexer;

fn main() {
    let input = "let x = 5;";
    let l = Lexer::new(input);
    let mut p = Parser::new(l);

    if let Some(program) = p.parse_program() {
        println!("{:?}", program);
    }
}
