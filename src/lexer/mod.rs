use crate::token::{self, Token};

mod tests;

struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char
}

impl Lexer {
    pub fn new(inpt: String) -> Self {
        let mut l = Lexer {
            input: inpt,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();
        return l;
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len().try_into().unwrap() {
            self.ch = '\0';
        } else {
            self.ch = self
                .input
                .chars()
                .collect::<Vec<char>>()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.ch.is_alphabetic() || self.ch == '_' {
            self.read_char();
        }
        let chars = self.input.chars().collect::<Vec<char>>();
        let mut s = String::new();
        for i in position..self.position {
            s.push(chars[i]);
        }
        return s;
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.ch.is_digit(10) {
            self.read_char();
        }
        let chars = self.input.chars().collect::<Vec<char>>();
        let mut s = String::new();
        for i in position..self.position {
            s.push(chars[i]);
        }
        return s;
    }

    pub fn next_token(&mut self) -> Token {
        while self.ch.is_whitespace() {
            self.read_char();
        }

        let tok: Token = match self.ch {
            '=' => Token::ASSIGN,
            ';' => Token::SEMICOLON,
            '(' => Token::LPAREN,
            ')' => Token::RPAREN,
            '{' => Token::LBRACE,
            '}' => Token::RBRACE,
            '+' => Token::PLUS,
            ',' => Token::COMMA,
            _ => {
                if self.ch.is_alphabetic() || self.ch == '_' {
                    let literal= self.read_identifier();
                    token::lookup_ident(literal)
                } 
                else if self.ch.is_digit(10) {
                    Token::INT(self.read_number())
                } else {
                    Token::ILLEGAL
                }
            }
        };

        self.read_char();
        return tok;
    }
}
