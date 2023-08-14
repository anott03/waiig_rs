#[cfg(test)]
use crate::lexer::Lexer;
use crate::token::Token;

#[test]
fn read_char() {
    let input = "let five = 5;";
    let mut l = Lexer::new(input);
    assert!(l.ch == 'l');
    l.read_char();
    assert!(l.ch == 'e');
}

#[test]
fn read_number() {
    let input = "12345";
    let mut l = Lexer::new(input);
    let num = l.read_number();
    assert_eq!(num, String::from("12345"));
}

#[test]
fn next_token() {
    let input = "=+(){},;";
    let mut l = Lexer::new(input);
    let mut t = l.next_token();
    assert_eq!(t, Token::ASSIGN);
    t = l.next_token();
    assert_eq!(t, Token::PLUS);
    t = l.next_token();
    assert_eq!(t, Token::LPAREN);
}

#[test]
fn next_token_code() {
    let input = "let five = 5; \"foobar\";";
    let mut l = Lexer::new(input);
    let mut t = l.next_token();
    assert_eq!(t, Token::LET);
    t = l.next_token();
    assert_eq!(t, Token::IDENT(String::from("five")));
    t = l.next_token();
    assert_eq!(t, Token::ASSIGN);
    t = l.next_token();
    assert_eq!(t, Token::INT(String::from("5")));
    t = l.next_token();
    assert_eq!(t, Token::SEMICOLON);
    t = l.next_token();
    assert_eq!(t, Token::STRING(String::from("foobar")));
    t = l.next_token();
    assert_eq!(t, Token::SEMICOLON);
}

#[test]
fn read_identifier() {
    let input = "let five = 5";
    let mut l = Lexer::new(input);
    let mut t = l.next_token();
    assert_eq!(t, Token::LET);
    t = l.next_token();
    assert_eq!(t, Token::IDENT(String::from("five")));
    t = l.next_token();
    assert_eq!(t, Token::ASSIGN);
    t = l.next_token();
    assert_eq!(t, Token::INT(String::from("5")));
}
