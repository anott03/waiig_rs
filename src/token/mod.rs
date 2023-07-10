use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    IDENT(String),
    INT(String),
    ILLEGAL,
    EOF,
    ASSIGN,
    PLUS,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
}

pub fn lookup_ident(ident: String) -> Token {
    let mut keywords: HashMap<&str, Token> = HashMap::new();

    keywords.insert("let", Token::LET);

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
