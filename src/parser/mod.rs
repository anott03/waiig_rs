use crate::lexer::Lexer;
use crate::token::{Token, get_literal};
use crate::ast;

mod tests;

pub struct Parser {
    l: Lexer,
    pub curr_token: Option<Token>,
    pub peek_token: Option<Token>,
    pub errors: Vec<String>,
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

    fn parse_statement(&mut self) -> Option<ast::Statement> {
        return match self.curr_token {
            Some(Token::LET) => self.parse_let_statement(),
            _ => None,
        }
    }

    fn parse_program(&mut self) -> Option<ast::Program> {
        let mut prog = ast::Program{
            statements: Vec::new(),
        };
        while self.curr_token != Some(Token::EOF) {
            let stmt = self.parse_statement();
            if let Some(s) = stmt {
                prog.statements.push(s);
            }
            self.next_token();
        }
        return Some(prog);
    }
}
