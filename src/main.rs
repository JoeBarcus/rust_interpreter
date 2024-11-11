mod lexer;
mod token;

fn main() {
    //println!("{:?}", token::EOF);

    let x: String = "let five = 5;".to_owned();
    let mut lex = lexer::Lexer::new(x);
    let tok = &lex.next_token();
    println!("{:?}", tok.typed);
    lex.read_char();
    let tok = &lex.next_token();
    println!("{:?}", tok.typed);
    lex.read_char();
    let tok = &lex.next_token();
    println!("{:?}", tok.typed);
    lex.read_char();
    let tok = &lex.next_token();
    println!("{:?}", tok.typed);
    lex.read_char();
    let tok = &lex.next_token();
    println!("{:?}", tok.typed);
}
