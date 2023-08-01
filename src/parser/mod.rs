use crate::ast::{self, BlockStatement};
use crate::lexer::Lexer;
use crate::token::{get_literal, Token};

mod tests;

type PrefixParseFn = fn(&mut Parser) -> Option<ast::Expression>;
type InfixParseFn = fn(&mut Parser, ast::Expression) -> Option<ast::Expression>;

#[derive(Debug, PartialEq, PartialOrd)]
enum Priority {
    LOWEST,
    EQUALS,
    LESSGREATER,
    SUM,
    PRODUCT,
    PREFIX,
    CALL,
}

fn get_priority(t: &Token) -> Priority {
    return match t {
        Token::EQ | Token::NEQ => Priority::EQUALS,
        Token::LT | Token::GT => Priority::LESSGREATER,
        Token::PLUS | Token::MINUS => Priority::SUM,
        Token::SLASH | Token::ASTERISK => Priority::PRODUCT,
        _ => Priority::LOWEST,
    };
}

fn parse_block_statement(p: &mut Parser) -> Option<ast::BlockStatement> {
    let mut block = ast::BlockStatement {
        token: p.curr_token.clone(),
        statements: Vec::new(),
    };

    p.next_token();
    while !p.expect_curr(Token::RSQUIRLY) && !p.expect_curr(Token::EOF) {
        if let Some(stmt) = p.parse_statement() {
            block.statements.push(stmt);
        }
        p.next_token();
    }

    return Some(block);
}

fn parse_if_statement(p: &mut Parser) -> Option<ast::Expression> {
    let mut exp = ast::IfExpression {
        token: p.curr_token.clone(),
        condition: Box::new(ast::Expression::Empty),
        consequence: BlockStatement {
            token: Token::EOF,
            statements: Vec::new(),
        },
        alternative: None,
    };

    if !p.expect_peek(Token::LPAREN) {
        return None;
    }

    p.next_token();
    p.next_token();
    let cond = p.parse_expression(Priority::LOWEST);
    exp.condition = Box::new(cond.unwrap());

    // TODO for some reason the token is getting advanced somewhere it shouldn't
    // be ... or something. the book's peek_token advances the token and mine
    // does not and I think there are now several inconsistencies re. where the
    // token gets advanced
    // if !p.expect_peek(Token::RPAREN) {
    //     return None;
    // }
    // p.next_token();

    if !p.expect_peek(Token::LSQUIRLY) {
        return None;
    }
    p.next_token();

    if let Some(bs) = parse_block_statement(p) {
        exp.consequence = bs;
    }

    if p.peek_token == Token::ELSE {
        p.next_token();
        if !p.expect_peek(Token::LSQUIRLY) {
            return None;
        }
        p.next_token();

        if let Some(bs) = parse_block_statement(p) {
            exp.alternative = Some(bs);
        }
    }

    return Some(ast::Expression::IfExpression(exp));
}

fn parse_grouped_expression(p: &mut Parser) -> Option<ast::Expression> {
    p.next_token();
    let exp = p.parse_expression(Priority::LOWEST);
    if !p.expect_peek(Token::RPAREN) {
        return None;
    }
    p.next_token();

    return exp;
}

fn parse_identifier(p: &mut Parser) -> Option<ast::Expression> {
    return Some(ast::Expression::Identifier(ast::Identifier {
        token: p.curr_token.clone(),
        value: get_literal(&p.curr_token),
    }));
}

fn parse_integer_literal(p: &mut Parser) -> Option<ast::Expression> {
    if let Ok(val) = get_literal(&p.curr_token).parse() {
        let lit = ast::IntegerLiteral {
            token: p.curr_token.clone(),
            value: val,
        };

        return Some(ast::Expression::IntegerLiteral(lit));
    }
    return None;
}

fn parse_prefix_expression(p: &mut Parser) -> Option<ast::Expression> {
    let tok = p.curr_token.clone();
    p.next_token();
    let expression = ast::PrefixExpression {
        token: tok.clone(),
        operator: get_literal(&tok),
        right: Box::new(p.parse_expression(Priority::PREFIX).unwrap()),
    };
    return Some(ast::Expression::PrefixExpression(expression));
}

fn parse_infix_expression(p: &mut Parser, exp: ast::Expression) -> Option<ast::Expression> {
    let mut expression = ast::InfixExpression {
        token: p.curr_token.clone(),
        operator: get_literal(&p.curr_token),
        left: Box::new(exp),
        right: Box::new(ast::Expression::Empty),
    };

    let priority = p.curr_priority();
    p.next_token();
    expression.right = Box::new(p.parse_expression(priority).unwrap());
    return Some(ast::Expression::InfixExpression(expression));
}

fn parse_boolean(p: &mut Parser) -> Option<ast::Expression> {
    return Some(ast::Expression::Boolean(ast::Boolean {
        token: p.curr_token.clone(),
        value: if p.curr_token == Token::TRUE { true } else { false },
    }));
}

fn get_prefix_fn(token: &Token) -> Option<PrefixParseFn> {
    return match token {
        Token::IDENT(_) => Some(parse_identifier),
        Token::INT(_) => Some(parse_integer_literal),
        Token::BANG | Token::MINUS => Some(parse_prefix_expression),
        Token::TRUE | Token::FALSE => Some(parse_boolean),
        Token::LPAREN => Some(parse_grouped_expression),
        Token::IF => Some(parse_if_statement),
        _ => None,
    };
}

fn get_infix_fn(token: Token) -> Option<InfixParseFn> {
    return match token {
        Token::PLUS
        | Token::MINUS
        | Token::SLASH
        | Token::ASTERISK
        | Token::EQ
        | Token::NEQ
        | Token::LT
        | Token::GT => Some(parse_infix_expression),
        _ => None,
    };
}

pub struct Parser<'a> {
    l: Lexer<'a>,
    pub curr_token: Token,
    pub peek_token: Token,
    pub errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(l: Lexer<'a>) -> Self {
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

    fn expect_peek(&mut self, t: Token) -> bool {
        return match t {
            Token::IDENT(_) => {
                if let Token::IDENT(_) = self.peek_token {
                    true
                } else {
                    self.peek_error(t);
                    false
                }
            }
            Token::INT(_) => {
                if let Token::INT(_) = self.peek_token {
                    true
                } else {
                    self.peek_error(t);
                    false
                }
            }
            _ => {
                if self.peek_token == t {
                    true
                } else {
                    self.peek_error(t);
                    false
                }
            }
        };
    }

    fn expect_curr(&self, t: Token) -> bool {
        if self.curr_token == t {
            return match t {
                Token::IDENT(_) => {
                    if let Token::IDENT(_) = self.curr_token {
                        return true;
                    }
                    return false;
                }
                Token::INT(_) => {
                    if let Token::INT(_) = self.curr_token {
                        return true;
                    }
                    return false;
                }
                _ => self.curr_token == t,
            };
        }
        return false;
    }

    fn peek_error(&mut self, t: Token) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            t, self.peek_token
        );
        self.errors.push(msg);
    }

    fn peek_priority(&self) -> Priority {
        return get_priority(&self.peek_token);
    }

    fn curr_priority(&self) -> Priority {
        return get_priority(&self.curr_token);
    }

    fn no_prefix_parse_fn_error(&mut self, t: Token) {
        let msg = format!("no prefix parse function for {} found", get_literal(&t));
        self.errors.push(msg);
    }

    fn parse_let_statement(&mut self) -> Option<ast::Statement> {
        let mut stmt = ast::LetStatement {
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
        if !self.expect_peek(Token::ASSIGN) {
            return None;
        }

        while !self.expect_curr(Token::SEMICOLON) {
            self.next_token();
        }

        return Some(ast::Statement::LetStatement(stmt));
    }

    fn parse_return_statement(&mut self) -> Option<ast::Statement> {
        let stmt = ast::ReturnStatement {
            token: self.curr_token.clone(),
            return_val: ast::Expression::Empty,
        };
        // TODO: parse expression
        while !self.expect_curr(Token::SEMICOLON) {
            self.next_token();
        }
        return Some(ast::Statement::ReturnStatement(stmt));
    }

    fn parse_expression(&mut self, p: Priority) -> Option<ast::Expression> {
        if let Some(prefix) = get_prefix_fn(&self.curr_token) {
            let mut exp = prefix(self);
            while self.peek_token != Token::SEMICOLON && p < self.peek_priority() {
                if let Some(infix) = get_infix_fn(self.peek_token.clone()) {
                    self.next_token();
                    exp = infix(self, exp.unwrap());
                } else {
                    return exp;
                }
            }
            return exp;
        }
        self.no_prefix_parse_fn_error(self.curr_token.clone());
        return None;
    }

    fn parse_expression_statement(&mut self) -> Option<ast::Statement> {
        let stmt = ast::ExpressionStatement {
            token: self.curr_token.clone(),
            expression: match self.parse_expression(Priority::LOWEST) {
                Some(exp) => exp,
                None => ast::Expression::Empty,
            },
        };

        if self.peek_token == Token::SEMICOLON {
            self.next_token();
        }

        return Some(ast::Statement::ExpressionStatement(stmt));
    }

    fn parse_statement(&mut self) -> Option<ast::Statement> {
        return match self.curr_token.clone() {
            Token::LET => self.parse_let_statement(),
            Token::RETURN => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        };
    }

    pub fn parse_program(&mut self) -> Option<ast::Program> {
        let mut prog = ast::Program {
            statements: Vec::new(),
        };
        while self.curr_token != Token::EOF {
            if let Some(statement) = self.parse_statement() {
                prog.statements.push(statement);
            }
            self.next_token();
        }
        return Some(prog);
    }
}
