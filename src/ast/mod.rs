use crate::token::{get_literal, Token};

pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Identifier {
    pub fn token_literal(&self) -> String {
        return get_literal(&self.token);
    }
}

pub struct Expression {}

pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Option<Expression>,
}

impl LetStatement {
    fn token_literal(&self) -> String {
        return get_literal(&self.token);
    }
}

pub enum Statement {
    LetStatement(LetStatement)
}

impl Statement {
    fn token_literal(&self) -> String {
        return String::from("");
    }
}

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            return self.statements.first().unwrap().token_literal();
        }
        return String::from("");
    }
}
