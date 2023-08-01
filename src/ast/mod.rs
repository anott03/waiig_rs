use crate::token::{get_literal, Token};

#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<Statement>,
}

impl BlockStatement {
    fn token_literal(&self) -> String {
        return get_literal(&self.token);
    }

    fn to_string(&self) -> String {
        let mut out = String::new();
        self.statements.iter().for_each(|s| {
            out += &s.to_string();
        });
        return out;
    }
}

#[derive(Debug, Clone)]
pub struct IfExpression {
    pub token: Token,
    pub condition: Box<Expression>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

impl IfExpression {
    fn token_literal(&self) -> String {
        return get_literal(&self.token);
    }

    fn to_string(&self) -> String {
        return match &self.alternative {
            Some(alt) => format!("if {:?} {:?} else {:?}", self.condition.to_string(), self.consequence.to_string(), &alt.to_string()),
            None => format!("if {:?} {:?}", self.condition.to_string(), self.consequence.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Boolean {
    pub token: Token,
    pub value: bool,
}

impl Boolean {
    pub fn token_literal(&self) -> String {
        return get_literal(&self.token);
    }

    pub fn to_string(&self) -> String {
        return get_literal(&self.token);
    }
}

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
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Box<Expression>,
}

impl PrefixExpression {
    pub fn token_literal(&self) -> String {
        return get_literal(&self.token);
    }

    pub fn to_string(&self) -> String {
        return format!("({}{})", self.operator, self.right.to_string());
    }
}

#[derive(Debug, Clone)]
pub struct InfixExpression {
    pub token: Token,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub operator: String,
}

impl InfixExpression {
    pub fn token_literal(&self) -> String {
        return get_literal(&self.token);
    }

    pub fn to_string(&self) -> String {
        return format!("({} {} {})", self.left.to_string(), self.operator, self.right.to_string());
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Empty,
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    PrefixExpression(PrefixExpression),
    InfixExpression(InfixExpression),
    Boolean(Boolean),
    IfExpression(IfExpression), 
    BlockStatement(BlockStatement),
}

impl Expression {
    pub fn to_string(&self) -> String {
        return match self {
            Expression::Empty => String::new(),
            Expression::Identifier(i) => i.to_string(),
            Expression::IntegerLiteral(il) => il.to_string(),
            Expression::PrefixExpression(pe) => pe.to_string(),
            Expression::InfixExpression(ie) => ie.to_string(),
            Expression::Boolean(b) => b.to_string(),
            Expression::IfExpression(ie) => ie.to_string(),
            Expression::BlockStatement(bs) => bs.to_string(),
        }
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
            return format!("{} {} = {}", get_literal(&self.token), self.name.to_string(), val.to_string());
        }
        return format!("{} {} = null", get_literal(&self.token), self.name.to_string());
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

#[derive(Debug, Clone)]
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
    fn string1() {
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

    #[test]
    fn string2() {
        let program = Program {
            statements: vec![
                Statement::LetStatement(LetStatement {
                    token: Token::LET,
                    name: Identifier { token: Token::IDENT(String::from("x")), value: String::from("x") },
                    value: Some(Expression::IntegerLiteral(IntegerLiteral {
                        token: Token::INT(String::from("5")),
                        value: 5,
                    })),
                }),
            ],
        };

        println!("{}", program.clone().to_string());
        assert_eq!(program.to_string(), String::from("let x = 5"));
    }
}
