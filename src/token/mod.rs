use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    IDENT(String),
    INT(String),

    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,

    ILLEGAL,
    EOF,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LSQUIRLY,
    RSQUIRLY,

    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    POWER,

    LT,
    GT,
    EQ,
    NEQ,
}

pub fn lookup_ident(ident: String) -> Token {
    let mut keywords: HashMap<&str, Token> = HashMap::new();

    keywords.insert("let", Token::LET);
    keywords.insert("fn", Token::FUNCTION);
    keywords.insert("true", Token::TRUE);
    keywords.insert("false", Token::FALSE);
    keywords.insert("if", Token::IF);
    keywords.insert("else", Token::ELSE);
    keywords.insert("return", Token::RETURN);

    return match keywords.get(ident.as_str()) {
        Some(x) => x.clone(),
        None => Token::IDENT(ident)
    }
}

pub fn get_literal(token: &Token) -> String {
    return match token {
        Token::IDENT(s) => s.to_string(),
        Token::INT(s) => s.to_string(),
        Token::FUNCTION => String::from("fn"),
        Token::LET => String::from("let"),
        Token::TRUE => String::from("true"),
        Token::FALSE => String::from("false"),
        Token::IF => String::from("if"),
        Token::ELSE => String::from("else"),
        Token::RETURN => String::from("return"),
        Token::ILLEGAL => String::from("ILLEGAL"),
        Token::EOF => String::from("\0"),
        Token::COMMA => String::from(","),
        Token::SEMICOLON => String::from(";"),
        Token::LPAREN => String::from("("),
        Token::RPAREN => String::from(")"),
        Token::LSQUIRLY => String::from("{"),
        Token::RSQUIRLY => String::from("}"),
        Token::ASSIGN => String::from("="),
        Token::PLUS => String::from("+"),
        Token::MINUS => String::from("-"),
        Token::BANG => String::from("!"),
        Token::ASTERISK => String::from("*"),
        Token::SLASH => String::from("/"),
        Token::LT => String::from("<"),
        Token::GT => String::from(">"),
        Token::EQ => String::from("=="),
        Token::NEQ => String::from("!="),
        Token::POWER => String::from("**"),
    }
}

#[cfg(test)]
mod tests {
    use crate::{token, token::Token};

    #[test]
    fn lookup_ident() {
        assert_eq!(token::lookup_ident(String::from("let")), Token::LET);
    }
}
