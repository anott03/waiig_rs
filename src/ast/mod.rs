use crate::token::{get_literal, Token};

#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Identifier {
    pub fn token_literal(&self) -> String {
        return get_literal(&self.token);
    }
}

#[derive(Debug)]
pub struct Expression {}

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Option<Expression>,
}

impl LetStatement {
    pub fn token_literal(&self) -> String {
        return get_literal(&self.token);
    }
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_val: Expression,
}

impl ReturnStatement {
    pub fn token_literal(&self) -> String {
        return get_literal(&self.token);
    }
}

#[derive(Debug)]
pub enum Statement {
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
}

impl Statement {
    pub fn token_literal(&self) -> String {
        return String::from("");
    }
}

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            return self.statements.first().unwrap().token_literal();
        }
        return String::from("");
    }
}
