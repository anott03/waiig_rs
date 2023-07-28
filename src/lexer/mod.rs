use crate::token::{self, Token};

mod tests;

#[derive(Clone)]
pub struct Lexer<'a> {
    input: &'a [u8],
    position: usize,
    read_position: usize,
    ch: char
}

#[allow(dead_code)]
impl<'a> Lexer<'a> {
    pub fn new(inpt: &'a str) -> Self {
        let mut l = Lexer {
            input: inpt.as_bytes(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();
        return l;
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len().into() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position].into();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.ch.is_alphanumeric() || self.ch == '_' {
            self.read_char();
        }
        let mut s = String::new();
        for i in position..self.position {
            s.push(self.input[i].into());
        }
        return s;
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.ch.is_digit(10) {
            self.read_char();
        }
        let mut s = String::new();
        for i in position..self.position {
            s.push(self.input[i].into());
        }
        return s;
    }

    fn peek_char(&mut self) -> char {
        if self.read_position >= self.input.len() {
            return '\0';
        }
        return self.input[self.read_position].into();
    }

    pub fn next_token(&mut self) -> Token {
        while self.ch.is_whitespace() {
            self.read_char();
        }

        let tok: Token = match self.ch {
            '\0' => Token::EOF,
            ',' => Token::COMMA,
            ';' => Token::SEMICOLON,
            '(' => Token::LPAREN,
            ')' => Token::RPAREN,
            '{' => Token::LSQUIRLY,
            '}' => Token::RSQUIRLY,

            '+' => Token::PLUS,
            '-' => Token::MINUS,
            '*' => Token::ASTERISK,
            '/' => Token::SLASH,
            '<' => Token::LT,
            '>' => Token::GT,
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::EQ
                } else {
                    Token::ASSIGN
                }
            },
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::NEQ
                } else {
                    Token::BANG
                }
            },

            _ => {
                if self.ch.is_alphabetic() || self.ch == '_' {
                    let literal= self.read_identifier();
                    token::lookup_ident(literal)
                } else if self.ch.is_digit(10) {
                    // intentionally omit self.read_char() as it is taken care
                    // of in read_number
                    return Token::INT(self.read_number());
                } else {
                    Token::ILLEGAL
                }
            }
        };

        self.read_char();
        return tok;
    }
}
