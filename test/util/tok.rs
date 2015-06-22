#[derive(Debug)]
pub enum Tok {
    Num(u32),
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
         '0' ... '9' => Some(Tok::Num((c as u32) - ('0' as u32))),
         _ => if c.is_whitespace() {None} else {panic!("invalid character `{}`", c)},
     })
     .collect()
}
