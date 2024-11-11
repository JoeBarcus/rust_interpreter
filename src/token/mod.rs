use once_cell::sync::Lazy;
use std::collections::HashMap;

//type TokenType string
pub struct Token {
    pub typed: String,
    pub literal: String,
}

pub const ILLEGAL: &str = "ILLEGAL";
pub const EOF: &str = "EOF";
pub const IDENT: &str = "IDENT";
pub const INT: &str = "INT";
pub const ASSIGN: &str = "=";
pub const PLUS: &str = "+";
pub const COMMA: &str = ",";
pub const SEMICOLON: &str = ";";
pub const LPAREN: &str = "(";
pub const RPAREN: &str = ")";
pub const LBRACE: &str = "{";
pub const RBRACE: &str = "}";
pub const FUNCTION: &str = "FUNCTION";
pub const LET: &str = "LET";

static KEYWORDS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("fn", FUNCTION);
    map.insert("let", LET);
    map
});

pub fn lookup_ident(ident: String) -> String {
    let res = match KEYWORDS.get(ident.as_str()) {
        Some(child) => child.to_string(),
        None => ident,
    };
    res
}
