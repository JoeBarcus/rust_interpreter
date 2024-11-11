use crate::token::*;

#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer: Lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };

        lexer.read_char();

        lexer
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.ch {
            '=' => new_token(ASSIGN, self.ch),
            ';' => new_token(SEMICOLON, self.ch),
            '(' => new_token(LPAREN, self.ch),
            ')' => new_token(RPAREN, self.ch),
            ',' => new_token(COMMA, self.ch),
            '+' => new_token(PLUS, self.ch),
            '{' => new_token(LBRACE, self.ch),
            '}' => new_token(RBRACE, self.ch),
            '0' => Token {
                typed: EOF.to_owned(),
                literal: "".to_owned(),
            },
            _ => {
                if is_letter(self.ch) {
                    let lit = self.read_identifier();
                    let typ = lookup_ident(lit.clone());
                    Token {
                        typed: typ,
                        literal: lit,
                    }
                } else if is_digit(self.ch) {
                    let lit = self.read_number();
                    let typ = INT.to_string();
                    Token {
                        typed: typ,
                        literal: lit,
                    }
                } else {
                    Token {
                        typed: ILLEGAL.to_owned(),
                        literal: self.ch.to_string(),
                    }
                }
            }
        }
    }
    fn skip_whitespace(&mut self) {
        if self.ch.is_whitespace() || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        loop {
            //println!("Read_number");
            if is_digit(self.ch) {
                self.read_char();
            } else {
                break;
            }
        }

        self.input[position - 1..self.position].to_owned()
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        loop {
            //println!("read_identifier");
            if is_letter(self.ch) {
                self.read_char();
            } else {
                break;
            }
        }

        self.input[position..self.position].to_owned()
    }
}

fn new_token(token_type: &str, ch: char) -> Token {
    Token {
        typed: token_type.to_owned(),
        literal: ch.to_string(),
    }
}

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

#[test]
fn test_assign() {
    let x: String = "=+".to_owned();
    let mut lex = Lexer::new(x);
    let tok = &lex.next_token();
    assert_eq!(tok.typed, "=");
    assert_eq!(tok.literal, "=");
}

#[test]
fn test_assign_twice() {
    let x: String = "=+".to_owned();
    let mut lex = Lexer::new(x);
    lex.read_char();
    let tok = &lex.next_token();
    assert_eq!(tok.typed, "+");
    assert_eq!(tok.literal, "+");
}

#[test]
fn test_assign_eof() {
    let x: String = "=+".to_owned();
    let mut lex = Lexer::new(x);
    lex.read_char();
    lex.read_char();
    let tok = &lex.next_token();
    assert_eq!(tok.typed, "EOF");
    assert_eq!(tok.literal, "");
}

#[test]
fn test_real_code() {
    let x: String = "let five = 5;".to_owned();
    let mut lex = Lexer::new(x);
    let tok = &lex.next_token();
    assert_eq!(tok.typed, "LET");
    lex.read_char();
    let tok = &lex.next_token();
    assert_eq!(tok.typed, "five");
    lex.read_char();
    let tok = &lex.next_token();
    assert_eq!(tok.typed, "=");
    lex.read_char();
    let tok = &lex.next_token();
    assert_eq!(tok.typed, "INT");
}
