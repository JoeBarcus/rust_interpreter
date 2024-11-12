mod lexer;
mod token;

fn main() {
    //let x: String = "let five = 5;".to_owned();
    //let mut lex = lexer::Lexer::new(x);
    //let tok = &lex.next_token();
    //println!("{:?}", tok.typed);
    //lex.read_char();
    //let tok = &lex.next_token();
    //println!("{:?}", tok.typed);
    //lex.read_char();
    //let tok = &lex.next_token();
    //println!("{:?}", tok.typed);
    //lex.read_char();
    //let tok = &lex.next_token();
    //println!("{:?}", tok.typed);
    //lex.read_char();
    //let tok = &lex.next_token();
    //println!("{:?}", tok.typed);

    let x = "let five = 5;";
    let mut l = lexer::Lexer::new(x.to_owned());
    for i in 0..6 {
        //l.next_token();
        println!("{:?}", l.next_token());
    }
}
