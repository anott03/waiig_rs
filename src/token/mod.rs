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

#[cfg(test)]
mod tests {
    use crate::{token, token::Token};

    #[test]
    fn lookup_ident() {
        assert_eq!(token::lookup_ident(String::from("let")), Token::LET);
    }
}
