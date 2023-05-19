use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    ILLEGAL(String),
    EOF(String),
    IDENT(String),
    INT(String),
    ASSIGN(String),
    PLUS(String),
    COMMA(String),
    SEMICOLON(String),
    LPAREN(String),
    RPAREN(String),
    LBRACE(String),
    RBRACE(String),
    FUNCTION(String),
    LET(String),
}

pub fn lookup_ident(ident: String) -> Token {
    let mut keywords: HashMap<&str, Token> = HashMap::new();

    keywords.insert("let", Token::LET(String::from("let")));

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
        assert_eq!(token::lookup_ident(String::from("let")), Token::LET(String::from("let")));
    }
}
