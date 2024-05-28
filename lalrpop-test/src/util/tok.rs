use std::str::{CharIndices, FromStr};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tok<'input> {
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
    String(&'input str),
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
pub fn tokenize(s: &str) -> Vec<(usize, Tok<'_>, usize)> {
    let mut tokens = vec![];
    let mut char_indices = s.char_indices();
    let mut lookahead = char_indices.next();
    while let Some((ci, c)) = lookahead {
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
                    let (ci, _) = char_indices.next().expect("Unclosed '\"'"); // consume opening '"'
                    let (slice_end, _) = take_while(ci, &mut char_indices, |c| c != '"');
                    lookahead = char_indices.next(); // consume closing '"'
                    tokens.push(Tok::String(&s[ci..slice_end]));
                    continue;
                }
                _ if c.is_ascii_digit() => {
                    let (slice_end, next) =
                        take_while(ci, &mut char_indices, |c| c.is_ascii_digit());
                    lookahead = next;
                    tokens.push(Tok::Num(i32::from_str(&s[ci..slice_end]).unwrap()));
                    continue;
                }
                _ => {
                    panic!("invalid character: {:?}", c);
                }
            }
        }

        // advance to next character by default
        lookahead = char_indices.next();
    }

    tokens
        .into_iter()
        .enumerate()
        .map(|(i, tok)| (i * 2, tok, i * 2 + 1))
        .collect()
}

fn take_while<F>(
    slice_start: usize,
    char_indices: &mut CharIndices<'_>,
    f: F,
) -> (usize, Option<(usize, char)>)
where
    F: Fn(char) -> bool,
{
    let mut slice_end = slice_start + 1;

    for (ci, c) in char_indices {
        if f(c) {
            slice_end = ci + 1;
        } else {
            return (ci, Some((ci, c)));
        }
    }

    (slice_end, None)
}
