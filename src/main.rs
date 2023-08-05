#![allow(dead_code)]

mod token;
mod lexer;
mod parser;
mod ast;
mod object;
mod evaluator;
mod repl;

fn main() -> std::io::Result<()>{
    repl::run()?;
    return Ok(());
}
