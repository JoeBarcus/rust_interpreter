use crate::token::*;

#[derive(Debug)]
struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    fn new(input: String) -> Self {
        let mut lexer: Lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };

        lexer.read_char();

        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        match self.ch {
            '=' => new_token(ASSIGN, self.ch),
            ';' => new_token(SEMICOLON, self.ch),
            '(' => new_token(LPAREN, self.ch),
            ')' => new_token(RPAREN, self.ch),
            ',' => new_token(COMMA, self.ch),
            '+' => new_token(PLUS, self.ch),
            '{' => new_token(LBRACE, self.ch),
            '}' => new_token(RBRACE, self.ch),
            _ => new_token(EOF, self.ch),
        }
    }
}

fn new_token(token_type: &str, ch: char) -> Token {
    Token {
        typed: token_type.to_owned(),
        literal: ch,
    }
}

#[test]
fn test_assign() {
    let x: String = "=+".to_owned();
    let mut lex = Lexer::new(x);
    let tok = &lex.next_token();
    assert_eq!(tok.typed, "=");
    assert_eq!(tok.literal, '=');
}

#[test]
fn test_assign_twice() {
    let x: String = "=+".to_owned();
    let mut lex = Lexer::new(x);
    lex.read_char();
    let tok = &lex.next_token();
    assert_eq!(tok.typed, "+");
    assert_eq!(tok.literal, '+');
}

#[test]
fn test_assign_eof() {
    let x: String = "=+".to_owned();
    let mut lex = Lexer::new(x);
    lex.read_char();
    lex.read_char();
    let tok = &lex.next_token();
    assert_eq!(tok.typed, "EOF");
    assert_eq!(tok.literal, '0');
}

#[test]
fn test_real_code() {
    let x: String = "let five = 5".to_owned();
}
