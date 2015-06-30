use std::str::FromStr;

#[derive(Debug)]
pub enum Tok {
    Num(i32),
    LParen,
    RParen,
    Minus,
    Plus,
    Times,
    Div,
}

impl Tok {
    pub fn as_num(&self) -> i32 {
        match *self {
            Tok::Num(x) => x,
            _ => panic!("as_num invoked with non-number"),
        }
    }
}

// simplest and stupidest possible tokenizer
pub fn tokenize(s: &str) -> Vec<Tok> {
    let mut tokens = vec![];
    let mut chars = s.chars();
    let mut lookahead = chars.next();
    while let Some(c) = lookahead {
        // skip whitespace characters
        if !c.is_whitespace() {
            match c {
                '(' => tokens.push(Tok::LParen),
                ')' => tokens.push(Tok::RParen),
                '-' => tokens.push(Tok::Minus),
                '+' => tokens.push(Tok::Plus),
                '*' => tokens.push(Tok::Times),
                '/' => tokens.push(Tok::Div),
                _ if c.is_digit(10) => {
                    let (tmp, next) = take_while(c, &mut chars, |c| c.is_digit(10));
                    lookahead = next;
                    tokens.push(Tok::Num(i32::from_str(&tmp).unwrap()));
                    continue;
                }
                _ => {
                    panic!("invalid character: {:?}", c);
                }
            }
        }

        // advance to next character by default
        lookahead = chars.next();
    }
    tokens
}

fn take_while<C,F>(c0: char, chars: &mut C, f: F) -> (String, Option<char>)
    where C: Iterator<Item=char>, F: Fn(char) -> bool
{
    let mut buf = String::new();

    buf.push(c0);

    while let Some(c) = chars.next() {
        if !f(c) {
            return (buf, Some(c));
        }

        buf.push(c);
    }

    return (buf, None);
}
