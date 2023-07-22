use crate::lexer::Lexer;
use crate::token::{Token, get_literal};
use crate::ast;

mod tests;

type PrefixParseFn = fn(&Parser) -> Option<ast::Expression>;
type InfixParseFn<'a> = fn(&Parser, ast::Expression<'a>) -> Option<ast::Expression<'a>>;

fn parse_identifier(p: &Parser) -> Option<ast::Expression> {
    return Some(ast::Expression::Identifier(ast::Identifier{
        token: p.curr_token.clone(),
        value: get_literal(&p.curr_token)
    }));
}

fn parse_integer_literal(p: &Parser) -> Option<ast::Expression> {
    if let Ok(val) = get_literal(&p.curr_token).parse() {
        let lit = ast::IntegerLiteral {
            token: p.curr_token.clone(),
            value: val,
        };

        return Some(ast::Expression::IntegerLiteral(lit));
    }
    return None;
}

fn get_prefix_fn(token: &Token) -> Option<PrefixParseFn> {
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
    pub curr_token: Token,
    pub peek_token: Token,
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
            curr_token: Token::EOF,
            peek_token: Token::EOF,
            errors: Vec::new(),
        };
        p.next_token();
        p.next_token();
        return p;
    }

    fn next_token(&mut self) {
        self.curr_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    fn expect_peek(&self, t: Token) -> bool {
        if self.peek_token.clone() == t {
            return match t {
                Token::IDENT(_) => {
                    if let Token::IDENT(_) = self.peek_token {
                        return true;
                    }
                    // self.peek_error(t);
                    return false;
                },
                Token::INT(_) => {
                    if let Token::INT(_) = self.peek_token {
                        return true;
                    }
                    // self.peek_error(t);
                    return false;
                },
                _ => self.peek_token == t,
            };
        }
        // self.peek_error(t);
        return false;
    }

    fn expect_curr(&self, t: Token) -> bool {
        if self.curr_token == t {
            return match t {
                Token::IDENT(_) => {
                    if let Token::IDENT(_) = self.curr_token {
                        return true;
                    }
                    return false;
                },
                Token::INT(_) => {
                    if let Token::INT(_) = self.curr_token {
                        return true;
                    }
                    return false;
                },
                _ => self.curr_token == t,
            };
        }
        return false;
    }

    fn peek_error(&mut self, t: Token) {
        let msg = format!("expected next token to be {:?}, got {:?} instead", t, self.peek_token);
        self.errors.push(msg);
    }

    fn parse_let_statement(&mut self) -> Option<ast::Statement> {
        let mut stmt = ast::LetStatement{
            token: self.curr_token.clone(),
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
        stmt.name.token = self.curr_token.clone();
        stmt.name.value = get_literal(&self.curr_token);
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
            token: self.curr_token.clone(),
            return_val: ast::Expression::Empty
        };
        // TODO: parse expression
        while !self.expect_curr(Token::SEMICOLON) {
            self.next_token();
        }
        return Some(ast::Statement::ReturnStatement(stmt));
    }

    fn parse_expression(&self, p: Priority) -> Option<ast::Expression> {
        if let Some(prefix) = get_prefix_fn(&self.curr_token) {
            return prefix(self);
        }
        return None;
    }

    fn parse_expression_statement(&self) -> Option<ast::Statement> {
        let stmt = ast::ExpressionStatement {
            token: self.curr_token.clone(),
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

    fn parse_statement(&mut self) -> Option<ast::Statement> {
        return match self.curr_token.clone() {
            Token::LET => self.parse_let_statement(),
            Token::RETURN => self.parse_return_statement(),
            _ => {
                if self.peek_token == Token::SEMICOLON {
                    self.next_token();
                }
                self.parse_expression_statement()
            },
        }
    }

    fn parse_program(&mut self) -> Option<ast::Program> {
        let mut prog = ast::Program{
            statements: Vec::new(),
        };
        while self.curr_token.clone() != Token::EOF {
            if let Some(statement) = self.parse_statement() {
                prog.statements.push(statement);
            }
            self.next_token();
        }
        return Some(prog);
    }
}
