use crate::token::{get_literal, Token};

#[derive(Debug, Clone)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Identifier {
    pub fn token_literal(&self) -> String {
        return get_literal(&self.token);
    }

    pub fn to_string(&self) -> String {
        return get_literal(&self.token);
    }
}

#[derive(Debug, Clone)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i32,
}

impl IntegerLiteral {
    pub fn token_literal(&self) -> String {
        return get_literal(&self.token);
    }

    pub fn to_string(&self) -> String {
        return get_literal(&self.token);
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Empty,
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
}

impl Expression {
    pub fn to_string(&self) -> String {
        return String::from("expression");
    }
}

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Option<Expression>,
}

impl LetStatement {
    pub fn token_literal(&self) -> String {
        return get_literal(&self.token);
    }

    pub fn to_string(&self) -> String {
        if let Some(val) = &self.value {
            return get_literal(&self.token) + " " + self.name.to_string().as_str() + " = " + val.to_string().as_str();
        }
        return get_literal(&self.token) + " " + self.name.to_string().as_str() + " = null";
    }
}

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_val: Expression,
}

impl ReturnStatement {
    pub fn token_literal(&self) -> String {
        return get_literal(&self.token);
    }

    pub fn to_string(&self) -> String {
        return String::from("return statement");
    }
}

#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Expression,
}

impl ExpressionStatement {
    pub fn token_literal(&self) -> String {
        return get_literal(&self.token);
    }

    pub fn to_string(&self) -> String {
        // TODO
        return get_literal(&self.token);
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
    ExpressionStatement(ExpressionStatement),
}

impl Statement {
    pub fn token_literal(&self) -> String {
        return String::from("");
    }

    pub fn to_string(&self) -> String {
        return match self {
            Statement::LetStatement(ls) => ls.to_string(),
            Statement::ReturnStatement(rs) => rs.to_string(),
            Statement::ExpressionStatement(es) => es.to_string(),
        }
    }
}

#[derive(Clone)]
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

    pub fn to_string(&self) -> String {
        let mut out = String::new();
        self.statements.iter().for_each(|s| {
            out += &s.to_string();
        });
        return out;
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::*;

    #[test]
    fn string() {
        let program = Program {
            statements: vec![
                Statement::LetStatement(LetStatement {
                    token: Token::LET,
                    name: Identifier { token: Token::IDENT(String::from("x")), value: String::from("x") },
                    value: None,
                }),
            ],
        };

        println!("{}", program.clone().to_string());
        assert_eq!(program.to_string(), String::from("let x = null"));
    }
}
