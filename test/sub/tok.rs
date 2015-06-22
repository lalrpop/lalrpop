#[derive(Debug)]
pub enum Tok {
    N(char),
    LParen,
    RParen,
    Minus,
}

// simplest and stupidest possible tokenizer
pub fn tokenize(s: &str) -> Vec<Tok> {
    s.chars()
     .filter_map(|c| match c {
         '(' => Some(Tok::LParen),
         ')' => Some(Tok::RParen),
         '-' => Some(Tok::Minus),
         '0' ... '9' => Some(Tok::N(c)),
         _ => if c.is_whitespace() {None} else {panic!("invalid character `{}`", c)},
     })
     .collect()
}
