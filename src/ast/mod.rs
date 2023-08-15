use crate::token::{get_literal, Token};

pub trait Inspect {
    fn token_literal(&self) -> String;
    fn to_string(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct ArrayLiteral {
    token: Token,
    elements: Vec<Expression>,
}

impl Inspect for ArrayLiteral {
    fn token_literal(&self) -> String {
        return get_literal(&self.token);
    }

    fn to_string(&self) -> String {
        return format!("[{}]", self
            .elements
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join(", ")
        );
    }
}

#[derive(Debug, Clone)]
pub struct ImportStatement {
    pub token: Token,
    pub namespace: StringLiteral,
}

impl Inspect for ImportStatement {
    fn token_literal(&self) -> String {
        return get_literal(&self.token);
    }

    fn to_string(&self) -> String {
        return format!("import {}", self.namespace.value);
    }
}

#[derive(Debug, Clone)]
pub struct CallExpression {
    pub token: Token,
    pub function: Box<Expression>,
    pub arguments: Vec<Box<Expression>>,
}

impl Inspect for CallExpression {
    fn token_literal(&self) -> String {
        return get_literal(&self.token);
    }

    fn to_string(&self) -> String {
        let mut params = String::new();
        self.arguments.iter().for_each(|p| {
            params += format!("{}, ", p.to_string()).as_str();
        });
        return format!("{}({})", self.function.to_string(), params);
    }
}

#[derive(Debug, Clone)]
pub struct FunctionLiteral {
    pub token: Token,
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
}

impl Inspect for FunctionLiteral {
    fn token_literal(&self) -> String {
        return get_literal(&self.token);
    }

    fn to_string(&self) -> String {
        let mut params = String::new();
        self.parameters.iter().for_each(|p| {
            params += format!("{}, ", p.to_string()).as_str();
        });

        return format!("{} ({}) {}", self.token_literal(), params, self.body.to_string());
    }
}

#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<Statement>,
}

impl Inspect for BlockStatement {
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

impl Inspect for IfExpression {
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
pub struct BooleanLiteral {
    pub token: Token,
    pub value: bool,
}

impl Inspect for BooleanLiteral {
    fn token_literal(&self) -> String {
        return get_literal(&self.token);
    }

    fn to_string(&self) -> String {
        return get_literal(&self.token);
    }
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Inspect for Identifier {
    fn token_literal(&self) -> String {
        return get_literal(&self.token);
    }

    fn to_string(&self) -> String {
        return get_literal(&self.token);
    }
}

#[derive(Debug, Clone)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i32,
}

impl Inspect for IntegerLiteral {
    fn token_literal(&self) -> String {
        return get_literal(&self.token);
    }

    fn to_string(&self) -> String {
        return get_literal(&self.token);
    }
}

#[derive(Debug, Clone)]
pub struct StringLiteral {
    pub token: Token,
    pub value: String,
}

impl Inspect for StringLiteral {
    fn token_literal(&self) -> String {
        return get_literal(&self.token);
    }

    fn to_string(&self) -> String {
        return format!("{:?}", self.value);
    }
}

#[derive(Debug, Clone)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Box<Expression>,
}

impl Inspect for PrefixExpression {
    fn token_literal(&self) -> String {
        return get_literal(&self.token);
    }

    fn to_string(&self) -> String {
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

impl Inspect for InfixExpression {
    fn token_literal(&self) -> String {
        return get_literal(&self.token);
    }

    fn to_string(&self) -> String {
        return format!("({} {} {})", self.left.to_string(), self.operator, self.right.to_string());
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Empty,
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    StringLiteral(StringLiteral),
    PrefixExpression(PrefixExpression),
    InfixExpression(InfixExpression),
    Boolean(BooleanLiteral),
    IfExpression(IfExpression), 
    FunctionLiteral(FunctionLiteral),
    CallExpression(CallExpression),
}

impl Inspect for Expression {
    fn token_literal(&self) -> String {
        return match self {
            Expression::Empty => String::new(),
            Expression::Identifier(i) => get_literal(&i.token),
            Expression::IntegerLiteral(il) => get_literal(&il.token),
            Expression::StringLiteral(sl) => get_literal(&sl.token),
            Expression::PrefixExpression(pe) => get_literal(&pe.token),
            Expression::InfixExpression(ie) => get_literal(&ie.token),
            Expression::Boolean(b) => get_literal(&b.token),
            Expression::IfExpression(ie) => get_literal(&ie.token),
            Expression::FunctionLiteral(fl) => get_literal(&fl.token),
            Expression::CallExpression(ce) => get_literal(&ce.token),
        }
    }

    fn to_string(&self) -> String {
        return match self {
            Expression::Empty => String::new(),
            Expression::Identifier(i) => i.to_string(),
            Expression::IntegerLiteral(il) => il.to_string(),
            Expression::StringLiteral(sl) => sl.to_string(),
            Expression::PrefixExpression(pe) => pe.to_string(),
            Expression::InfixExpression(ie) => ie.to_string(),
            Expression::Boolean(b) => b.to_string(),
            Expression::IfExpression(ie) => ie.to_string(),
            Expression::FunctionLiteral(fl) => fl.to_string(),
            Expression::CallExpression(ce) => ce.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Option<Expression>,
}

impl Inspect for LetStatement {
    fn token_literal(&self) -> String {
        return get_literal(&self.token);
    }

    fn to_string(&self) -> String {
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

impl Inspect for ReturnStatement {
    fn token_literal(&self) -> String {
        return get_literal(&self.token);
    }

    fn to_string(&self) -> String {
        return String::from("return statement");
    }
}

#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Expression,
}

impl Inspect for ExpressionStatement {
    fn token_literal(&self) -> String {
        return get_literal(&self.token);
    }

    fn to_string(&self) -> String {
        // TODO
        return get_literal(&self.token);
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
    ExpressionStatement(ExpressionStatement),
    ImportStatement(ImportStatement),
}

impl Inspect for Statement {
    fn token_literal(&self) -> String {
        return String::from("");
    }

    fn to_string(&self) -> String {
        return match self {
            Statement::LetStatement(ls) => ls.to_string(),
            Statement::ReturnStatement(rs) => rs.to_string(),
            Statement::ExpressionStatement(es) => es.to_string(),
            Statement::ImportStatement(is) => is.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Inspect for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            return self.statements.first().unwrap().token_literal();
        }
        return String::from("");
    }

    fn to_string(&self) -> String {
        let mut out = String::new();
        self.statements.iter().for_each(|s| {
            out += &s.to_string();
        });
        return out;
    }
}

pub enum Node {
    Program(Program),
    Expression(Expression),
    Statement(Statement),
    BlockStatement(BlockStatement),
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
