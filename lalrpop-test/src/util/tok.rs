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
    Comma,
}

// simplest and stupidest possible tokenizer
pub fn tokenize(s: &str) -> Vec<(usize, Tok, usize)> {
    let mut tokens = vec![];
    let mut chars = s.char_indices();
    let mut lookahead = chars.next();
    while let Some((pos, c)) = lookahead {
        // skip whitespace characters
        if !c.is_whitespace() {
            match c {
                '(' => tokens.push((pos, Tok::LParen, pos+1)),
                ')' => tokens.push((pos, Tok::RParen, pos+1)),
                '-' => tokens.push((pos, Tok::Minus, pos+1)),
                '+' => tokens.push((pos, Tok::Plus, pos+1)),
                '*' => tokens.push((pos, Tok::Times, pos+1)),
                ',' => tokens.push((pos, Tok::Comma, pos+1)),
                '/' => tokens.push((pos, Tok::Div, pos+1)),
                _ if c.is_digit(10) => {
                    let (tmp, next) = take_while(c, &mut chars, |c| c.is_digit(10));
                    lookahead = next;
                    let end = lookahead.map(|(pos, _)| pos).unwrap_or(s.len());
                    tokens.push((pos, Tok::Num(i32::from_str(&tmp).unwrap()), end));
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

fn take_while<C,F>(c0: char, chars: &mut C, f: F) -> (String, Option<(usize, char)>)
    where C: Iterator<Item=(usize, char)>, F: Fn(char) -> bool
{
    let mut buf = String::new();

    buf.push(c0);

    while let Some((pos, c)) = chars.next() {
        if !f(c) {
            return (buf, Some((pos, c)));
        }

        buf.push(c);
    }

    return (buf, None);
}
