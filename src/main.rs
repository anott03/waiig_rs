#![allow(dead_code)]

mod token;
mod lexer;
mod parser;
mod ast;

use crate::parser::Parser;
use crate::lexer::Lexer;

fn main() {
    let input = "true !=";
    let l = Lexer::new(input);
    let _p = Parser::new(l);
}
