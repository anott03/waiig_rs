use crate::lexer::Lexer;
use crate::token::{Token, get_literal};
use crate::ast;

mod tests;

type PrefixParseFn = fn(&Parser) -> Option<ast::Expression>;
type InfixParseFn<'a> = fn(&Parser, ast::Expression<'a>) -> Option<ast::Expression<'a>>;

fn parse_identifier(p: &Parser) -> Option<ast::Expression> {
    return Some(ast::Expression::Identifier(ast::Identifier{
        token: p.curr_token.clone().unwrap(),
        value: get_literal(&p.curr_token.clone().unwrap())
    }));
}

fn parse_integer_literal(p: &Parser) -> Option<ast::Expression> {
    if let Ok(val) = get_literal(&p.curr_token.clone().unwrap()).parse() {
        let lit = ast::IntegerLiteral {
            token: p.curr_token.clone().unwrap(),
            value: val,
        };

        return Some(ast::Expression::IntegerLiteral(lit));
    }
    return None;
}

fn get_prefix_fn(token: Token) -> Option<PrefixParseFn> {
    return match token {
        Token::IDENT(_) => Some(parse_identifier),
        Token::INT(_) => Some(parse_integer_literal),
        _ => None
    }
}

fn get_infix_fn(token: Token) -> Option<InfixParseFn<'static>> { None }

enum Priority {
    LOWEST,
    EQUALS,
    LESSGREATER,
    SUM,
    PRODUCT,
    PREFIX,
    CALL,
}

pub struct Parser {
    l: Lexer,
    pub curr_token: Option<Token>,
    pub peek_token: Option<Token>,
    pub errors: Vec<String>,
}

impl Clone for Parser {
    fn clone(&self) -> Self {
        Self { l: self.l.clone(), curr_token: self.curr_token.clone(), peek_token: self.peek_token.clone(), errors: self.errors.clone() }
    }
}

impl Parser {

    pub fn new(l: Lexer) -> Self {
        let mut p = Self {
            l,
            curr_token: None,
            peek_token: None,
            errors: Vec::new(),
        };
        p.next_token();
        p.next_token();
        return p;
    }

    fn next_token(&mut self) {
        self.curr_token = self.peek_token.clone();
        self.peek_token = Some(self.l.next_token());
    }

    fn expect_peek(&mut self, t: Token) -> bool {
        if let Some(tok) = self.peek_token.clone() {
            return match t {
                Token::IDENT(_) => {
                    if let Token::IDENT(_) = tok {
                        return true;
                    }
                    self.peek_error(t);
                    return false;
                },
                Token::INT(_) => {
                    if let Token::INT(_) = tok {
                        return true;
                    }
                    self.peek_error(t);
                    return false;
                },
                _ => tok == t,
            };
        }
        self.peek_error(t);
        return false;
    }

    fn expect_curr(&self, t: Token) -> bool {
        if let Some(tok) = self.curr_token.clone() {
            return match t {
                Token::IDENT(_) => {
                    if let Token::IDENT(_) = tok {
                        return true;
                    }
                    return false;
                },
                Token::INT(_) => {
                    if let Token::INT(_) = tok {
                        return true;
                    }
                    return false;
                },
                _ => tok == t,
            };
        }
        return false;
    }

    fn peek_error(&mut self, t: Token) {
        let msg = format!("expected next token to be {:?}, got {:?} instead", t, self.peek_token.as_ref().unwrap());
        self.errors.push(msg);
    }

    fn parse_let_statement(&mut self) -> Option<ast::Statement> {
        let mut stmt = ast::LetStatement{
            token: self.curr_token.clone().unwrap(),
            name: ast::Identifier {
                token: Token::ILLEGAL,
                value: String::new(),
            },
            value: None,
        };

        if !self.expect_peek(Token::IDENT(String::new())) {
            return None;
        }
        self.next_token();
        stmt.name.token = self.curr_token.clone().unwrap();
        stmt.name.value = get_literal(&self.curr_token.clone().unwrap());
        // TODO
        // if !self.expect_peek(Token::ASSIGN) {
        //     return None;
        // }

        while !self.expect_curr(Token::SEMICOLON) {
            self.next_token();
        }

        return Some(ast::Statement::LetStatement(stmt));
    }

    fn parse_return_statement(&mut self) -> Option<ast::Statement> {
        let stmt = ast::ReturnStatement {
            token: self.curr_token.clone().unwrap(),
            return_val: ast::Expression::Empty
        };
        // TODO: parse expression
        while !self.expect_curr(Token::SEMICOLON) {
            self.next_token();
        }
        return Some(ast::Statement::ReturnStatement(stmt));
    }

    fn parse_expression(&self, p: Priority) -> Option<ast::Expression> {
        if let Some(prefix) = get_prefix_fn(self.curr_token.clone().unwrap()) {
            return prefix(self);
        }
        return None;
    }

    fn parse_expression_statement(&self) -> Option<ast::Statement> {
        let stmt = ast::ExpressionStatement {
            token: self.curr_token.clone().unwrap(),
            expression: match self.parse_expression(Priority::LOWEST) {
                Some(exp) => exp,
                None => ast::Expression::Empty,
            },
        };

        // if self.peek_token.as_ref().unwrap() == &Token::SEMICOLON {
        //     self.next_token();
        // }

        return Some(ast::Statement::ExpressionStatement(stmt));
    }

    fn parse_statement(&'static mut self) -> Option<ast::Statement> {
        return match self.curr_token {
            Some(Token::LET) => self.parse_let_statement(),
            Some(Token::RETURN) => self.parse_return_statement(),
            _ => {
                if self.peek_token.as_ref().unwrap() == &Token::SEMICOLON {
                    self.next_token();
                }
                self.parse_expression_statement()
            },
        }
    }

    fn parse_program(&'static mut self) -> Option<ast::Program> {
        let mut prog = ast::Program{
            statements: Vec::new(),
        };
        while self.curr_token.as_ref().unwrap() != &Token::EOF {
            let stmt = self.parse_statement();
            if let Some(s) = stmt {
                prog.statements.push(s);
            }
            self.next_token();
        }
        return Some(prog);
    }
}
