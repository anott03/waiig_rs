use crate::token::{self, Token};

struct Lexer {
    input: String,
    position: i32,
    read_position: i32,
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
                .collect::<Vec<char>>()[self.read_position as usize];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position as usize;
        while self.ch.is_alphabetic() {
            self.read_char();
        }
        let chars: &str = self.input.as_str();
        // return String::from(chars[position..(self.position as usize)]);
        return String::new(); // placeholder return
    }

    pub fn next_token(&mut self) -> Token {
        let tok: Token = match self.ch {
            '=' => Token::ASSIGN(String::from(self.ch)),
            ';' => Token::SEMICOLON(String::from(self.ch)),
            '(' => Token::LPAREN(String::from(self.ch)),
            ')' => Token::RPAREN(String::from(self.ch)),
            '{' => Token::LBRACE(String::from(self.ch)),
            '}' => Token::RBRACE(String::from(self.ch)),
            '+' => Token::PLUS(String::from(self.ch)),
            ',' => Token::COMMA(String::from(self.ch)),
            _ => {
                if self.ch.is_alphabetic() {
                    Token::IDENT(self.read_identifier())
                } else {
                    Token::ILLEGAL(self.ch.to_string())
                }
            }
        };

        self.read_char();
        return tok;
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::token::Token;

    #[test]
    fn next_token() {
        let input = String::from("=+(){},;");
        let mut l = Lexer::new(input);
        let mut t = l.next_token();
        assert_eq!(t, Token::ASSIGN(String::from("=")));
        t = l.next_token();
        assert_eq!(t, Token::PLUS(String::from("+")));
        t = l.next_token();
        assert_eq!(t, Token::LPAREN(String::from("(")));
    }

    #[test]
    fn next_token_code() {
        let input = String::from("let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);");
        let mut l = Lexer::new(input);
        let mut t = l.next_token();
        assert_eq!(t, Token::LET(String::from("let")));
        t = l.next_token();
        assert_eq!(t, Token::IDENT(String::from("five")));
    }
}
