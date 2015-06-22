use std::io::{self, Read};

mod tok;
mod sub;

fn main() {
    let mut input = String::new();
    let mut stdin = io::stdin();
    let mut stdin = stdin.lock();
    stdin.read_to_string(&mut input).unwrap();
    let tokens = tok::tokenize(&input);
    let (lookahead, ()) = sub::parse_S(&mut tokens.into_iter()).unwrap();
    if lookahead.is_some() {
        println!("input not completely consumed");
    } else {
        println!("input ok");
    }
}
