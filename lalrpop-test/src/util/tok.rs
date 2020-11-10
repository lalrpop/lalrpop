use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tok {
    Num(i32),
    LParen,
    RParen,
    Minus,
    Plus,
    Times,
    Div,
    Comma,
    Open(Delim),
    Close(Delim),
    String(&'static str),
    #[allow(dead_code)]
    Fraction(i32, i32), // Not produced by tokenizer, used only in regression tests for #179
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Delim {
    /// '{' or '}'
    Brace,
    /// '[' or ']'
    Bracket,
}

// simplest and stupidest possible tokenizer
pub fn tokenize(s: &str) -> Vec<(usize, Tok, usize)> {
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
                ',' => tokens.push(Tok::Comma),
                '/' => tokens.push(Tok::Div),
                '{' => tokens.push(Tok::Open(Delim::Brace)),
                '[' => tokens.push(Tok::Open(Delim::Bracket)),
                '}' => tokens.push(Tok::Close(Delim::Brace)),
                ']' => tokens.push(Tok::Close(Delim::Bracket)),
                '"' => {
                    let c = chars.next().unwrap(); // consume opening '"'
                    let (tmp, _) = take_while(c, &mut chars, |c| c != '"');
                    lookahead = chars.next(); // consume closing '"'
                    tokens.push(Tok::String(Box::leak(tmp.into_boxed_str())));
                    continue;
                }
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
        .into_iter()
        .enumerate()
        .map(|(i, tok)| (i * 2, tok, i * 2 + 1))
        .collect()
}

fn take_while<C, F>(c0: char, chars: &mut C, f: F) -> (String, Option<char>)
where
    C: Iterator<Item = char>,
    F: Fn(char) -> bool,
{
    let mut buf = String::new();

    buf.push(c0);

    for c in chars {
        if !f(c) {
            return (buf, Some(c));
        }

        buf.push(c);
    }

    (buf, None)
}
