// const ILLEGAL: &str = "ILLEGAL";
// const EOF: &str = "EOF";
// const IDENT: &str = "IDENT";
// const INT: &str = "INT";
// const ASSIGN: &str = "=";
// const PLUS: &str = "+";
// const COMMA: &str = ",";
// const SEMICOLON: &str = ";";
// const LPAREN: &str = "(";
// const RPAREN: &str = ")";
// const LBRACE: &str = "{";
// const RBRACE: &str = "}";
// const FUNCTION: &str = "FUNCTION";
// const LET: &str = "LET";
//
// type TokenType = String;
//
// pub struct Token {
//     token_type: TokenType,
//     literal: String,
// }

#[derive(Debug, PartialEq, Eq)]
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
