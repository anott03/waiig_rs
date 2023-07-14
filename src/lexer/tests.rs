#[cfg(test)]
use crate::lexer::Lexer;
use crate::token::Token;

#[test]
fn next_token() {
    let input = String::from("=+(){},;");
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
    let input = String::from("let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;
");
    let mut l = Lexer::new(input);
    let t = l.next_token();
    assert_eq!(t, Token::LET);
    // t = l.next_token();
    // assert_eq!(t, Token::IDENT(String::from("five")));
}

#[test]
fn read_identifier() {
    let input = String::from("let five = 5");
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
