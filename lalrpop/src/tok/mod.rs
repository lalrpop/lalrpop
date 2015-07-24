//! A tokenizer for use in LALRPOP itself.

use std::str::CharIndices;
use unicode_xid::UnicodeXID;

use self::Error::*;
use self::Tok::*;

#[cfg(test)]
mod test;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    UnrecognizedToken(usize),
    UnterminatedEscape(usize),
    UnterminatedStringLiteral(usize),
    UnterminatedCode(usize),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tok<'input> {
    // Keywords;
    Enum,
    Extern,
    Grammar,
    If,
    Mut,
    Pub,
    Token,

    // Special keywords: these are accompanied by a series of
    // uninterpreted strings representing imports and stuff.
    Use(&'input str),
    Where(Vec<&'input str>),

    // Identifiers of various kinds:
    Escape(&'input str),
    Id(&'input str),
    MacroId(&'input str), // identifier followed immediately by `<`
    Lifetime(&'input str), // includes the `'`
    StringLiteral(&'input str), // excludes the `"`

    // Symbols:
    Ampersand,
    BangEquals,
    BangTilde,
    Colon,
    ColonColon,
    Comma,
    DotDot,
    Equals,
    EqualsEquals,
    EqualsGreaterThanCode(&'input str),
    EqualsGreaterThanQuestionCode(&'input str),
    EqualsGreaterThanLookahead,
    EqualsGreaterThanLookbehind,
    GreaterThan,
    LeftBrace,
    LeftBracket,
    LeftParen,
    LessThan,
    Lookahead, // @L
    Lookbehind, // @R
    Plus,
    Question,
    RightBrace,
    RightBracket,
    RightParen,
    Semi,
    Star,
    TildeTilde,
    Underscore,
}

struct Tokenizer<'input> {
    text: &'input str,
    chars: CharIndices<'input>,
    lookahead: Option<(usize, char)>,
}

macro_rules! eof {
    ($x:expr) => {
        match $x { Some(v) => v, None => { return None; } }
    }
}

type Spanned<T> = (usize, T, usize);

const KEYWORDS: &'static [(&'static str, Tok<'static>)] = &[
    ("enum", Enum),
    ("extern", Extern),
    ("grammar", Grammar),
    ("if", If),
    ("mut", Mut),
    ("pub", Pub),
    ("token", Token),
    ];

impl<'input> Tokenizer<'input> {
    fn new(text: &'input str) -> Tokenizer<'input> {
        let mut t = Tokenizer {
            text: text,
            chars: text.char_indices(),
            lookahead: None
        };
        t.bump();
        t
    }

    fn bump(&mut self) -> Option<(usize, char)> {
        self.lookahead = self.chars.next();
        self.lookahead
    }

    fn right_arrow(&mut self, idx0: usize) -> Result<Spanned<Tok<'input>>, Error> {
        // we've seen =>, now we have to choose between:
        //
        // => code
        // =>? code
        // =>@L
        // =>@R

        match self.lookahead {
            Some((_, '@')) => {
                match self.bump() {
                    Some((idx2, 'L')) => {
                        self.bump();
                        Ok((idx0, EqualsGreaterThanLookahead, idx2+1))
                    }
                    Some((idx2, 'R')) => {
                        self.bump();
                        Ok((idx0, EqualsGreaterThanLookbehind, idx2+1))
                    }
                    _ => {
                        Err(UnrecognizedToken(idx0))
                    }
                }
            }

            Some((idx1, '?')) => {
                self.bump();
                let idx2 = try!(self.code(idx0, "([{", "}])"));
                let code = &self.text[idx1+1..idx2];
                Ok((idx0, EqualsGreaterThanQuestionCode(code), idx2))
            }

            Some((idx1, _)) => {
                let idx2 = try!(self.code(idx0, "([{", "}])"));
                let code = &self.text[idx1..idx2];
                Ok((idx0, EqualsGreaterThanCode(code), idx2))
            }

            None => {
                Err(UnterminatedCode(idx0))
            }
        }
    }

    fn code(&mut self, idx0: usize, open_delims: &str, close_delims: &str) -> Result<usize, Error> {
        // This is the interesting case. To find the end of the code,
        // we have to scan ahead, matching (), [], and {}, and looking
        // for a suitable terminator: `,`, `;`, `]`, `}`, or `)`.
        let mut balance = 0; // number of unclosed `(` etc
        loop {
            if let Some((idx, c)) = self.lookahead {
                if open_delims.find(c).is_some() {
                    balance += 1;
                } else if balance > 0 {
                    if close_delims.find(c).is_some() {
                        balance -= 1;
                    }
                } else {
                    debug_assert!(balance == 0);

                    if c == ',' || c == ';' || close_delims.find(c).is_some() {
                        // Note: we do not consume the
                        // terminator. The code is everything *up
                        // to but not including* the terminating
                        // `,`, `;`, etc.
                        return Ok(idx);
                    }
                }
            } else if balance > 0 {
                // the input should not end with an
                // unbalanced number of `{` etc!
                return Err(UnterminatedCode(idx0));
            } else {
                debug_assert!(balance == 0);
                return Ok(self.text.len());
            }

            self.bump();
        }
    }

    fn escape(&mut self, idx0: usize) -> Result<Spanned<Tok<'input>>, Error> {
        match self.take_until(|c| c == '`') {
            Some(idx1) => {
                self.bump(); // consume the '`'
                let text: &'input str = &self.text[idx0+1..idx1]; // do not include the `` in the str
                Ok((idx0, Escape(text), idx1+1))
            }
            None => {
                Err(UnterminatedEscape(idx0))
            }
        }
    }

    fn string_literal(&mut self, idx0: usize) -> Result<Spanned<Tok<'input>>, Error> {
        let mut escape = false;
        let terminate = |c: char| {
            if escape {
                escape = false;
                false
            } else if c == '\\' {
                escape = true;
                false
            } else if c == '"' {
                true
            } else {
                false
            }
        };
        match self.take_until(terminate) {
            Some(idx1) => {
                self.bump(); // consume the '"'
                let text = &self.text[idx0+1..idx1]; // do not include the "" in the str
                Ok((idx0, StringLiteral(text), idx1+1))
            }
            None => {
                Err(UnterminatedStringLiteral(idx0))
            }
        }
    }

    fn lifetime(&mut self, idx0: usize) -> Spanned<Tok<'input>> {
        let (start, word, end) = self.word(idx0);
        (start, Lifetime(word), end)
    }

    fn identifierish(&mut self, idx0: usize) -> Result<Spanned<Tok<'input>>, Error> {
        let (start, word, end) = self.word(idx0);

        if word == "use" {
            let code_end = try!(self.code(idx0, "([{", "}])"));
            let code = &self.text[end..code_end];
            return Ok((start, Tok::Use(code), code_end));
        }

        if word == "where" {
            let mut wcs = vec![];
            let mut wc_start = end;
            let mut wc_end;
            loop {
                // Note: do not include `{` as a delimeter here, as
                // that is not legal in the trait/where-clause syntax,
                // and in fact signals start of the fn body. But do
                // include `<`.
                wc_end = try!(self.code(wc_start, "([<", ">])"));
                let wc = &self.text[wc_start..wc_end];
                wcs.push(wc);

                // if this ended in a comma, maybe expect another where-clause
                if let Some((_, ',')) = self.lookahead {
                    self.bump();
                    wc_start = wc_end + 1;
                } else {
                    break;
                }
            }

            return Ok((start, Tok::Where(wcs), wc_end));
        }

        let tok =
            // search for a keyword first; if none are found, this is
            // either a MacroId or an Id, depending on whether there
            // is a `<` immediately afterwards
            KEYWORDS.iter()
                    .filter(|&&(w, _)| w == word)
                    .map(|&(_, ref t)| t.clone())
                    .next()
                    .unwrap_or_else(|| {
                        match self.lookahead {
                            Some((_, '<')) => MacroId(word),
                            _ => Id(word),
                        }
                    });

        Ok((start, tok, end))
    }

    fn word(&mut self, idx0: usize) -> Spanned<&'input str> {
        match self.take_while(is_identifier_continue) {
            Some(end) => (idx0, &self.text[idx0..end], end),
            None => (idx0, &self.text[idx0..], self.text.len()),
        }
    }

    fn take_while<F>(&mut self, mut keep_going: F) -> Option<usize>
        where F: FnMut(char) -> bool
    {
        self.take_until(|c| !keep_going(c))
    }

    fn take_until<F>(&mut self, mut terminate: F) -> Option<usize>
        where F: FnMut(char) -> bool
    {
        loop {
            match self.lookahead {
                None => {
                    return None;
                }
                Some((idx1, c)) => {
                    if terminate(c) {
                        return Some(idx1);
                    } else {
                        self.bump();
                    }
                }
            }
        }
    }
}

impl<'input> Iterator for Tokenizer<'input> {
    type Item = Result<Spanned<Tok<'input>>, Error>;

    fn next(&mut self) -> Option<Result<Spanned<Tok<'input>>, Error>> {
        loop {
            return match self.lookahead {
                Some((idx0, '&')) => {
                    self.bump();
                    Some(Ok((idx0, Ampersand, idx0+1)))
                }
                Some((idx0, '!')) => {
                    match self.bump() {
                        Some((idx1, '=')) => {
                            self.bump();
                            Some(Ok((idx0, BangEquals, idx1+1)))
                        }
                        Some((idx1, '~')) => {
                            self.bump();
                            Some(Ok((idx0, BangTilde, idx1+1)))
                        }
                        _ => {
                            Some(Err(UnrecognizedToken(idx0)))
                        }
                    }
                }
                Some((idx0, ':')) => {
                    match self.bump() {
                        Some((idx1, ':')) => {
                            self.bump();
                            Some(Ok((idx0, ColonColon, idx1+1)))
                        }
                        _ => {
                            Some(Ok((idx0, Colon, idx0+1)))
                        }
                    }
                }
                Some((idx0, ',')) => {
                    self.bump();
                    Some(Ok((idx0, Comma, idx0+1)))
                }
                Some((idx0, '.')) => {
                    match self.bump() {
                        Some((idx1, '.')) => {
                            self.bump();
                            Some(Ok((idx0, DotDot, idx1+1)))
                        }
                        _ => {
                            Some(Err(UnrecognizedToken(idx0)))
                        }
                    }
                }
                Some((idx0, '=')) => {
                    match self.bump() {
                        Some((idx1, '=')) => {
                            self.bump();
                            Some(Ok((idx0, EqualsEquals, idx1+1)))
                        }
                        Some((_, '>')) => {
                            self.bump();
                            Some(self.right_arrow(idx0))
                        }
                        _ => {
                            Some(Ok((idx0, Equals, idx0+1)))
                        }
                    }
                }
                Some((idx0, '>')) => {
                    self.bump();
                    Some(Ok((idx0, GreaterThan, idx0+1)))
                }
                Some((idx0, '{')) => {
                    self.bump();
                    Some(Ok((idx0, LeftBrace, idx0+1)))
                }
                Some((idx0, '[')) => {
                    self.bump();
                    Some(Ok((idx0, LeftBracket, idx0+1)))
                }
                Some((idx0, '(')) => {
                    self.bump();
                    Some(Ok((idx0, LeftParen, idx0+1)))
                }
                Some((idx0, '<')) => {
                    self.bump();
                    Some(Ok((idx0, LessThan, idx0+1)))
                }
                Some((idx0, '@')) => {
                    match self.bump() {
                        Some((idx1, '<')) => {
                            self.bump();
                            Some(Ok((idx0, Lookahead, idx1+1)))
                        }
                        Some((idx1, '>')) => {
                            self.bump();
                            Some(Ok((idx0, Lookbehind, idx1+1)))
                        }
                        _ => {
                            Some(Err(UnrecognizedToken(idx0)))
                        }
                    }
                }
                Some((idx0, '+')) => {
                    self.bump();
                    Some(Ok((idx0, Plus, idx0+1)))
                }
                Some((idx0, '?')) => {
                    self.bump();
                    Some(Ok((idx0, Question, idx0+1)))
                }
                Some((idx0, '}')) => {
                    self.bump();
                    Some(Ok((idx0, RightBrace, idx0+1)))
                }
                Some((idx0, ']')) => {
                    self.bump();
                    Some(Ok((idx0, RightBracket, idx0+1)))
                }
                Some((idx0, ')')) => {
                    self.bump();
                    Some(Ok((idx0, RightParen, idx0+1)))
                }
                Some((idx0, ';')) => {
                    self.bump();
                    Some(Ok((idx0, Semi, idx0+1)))
                }
                Some((idx0, '*')) => {
                    self.bump();
                    Some(Ok((idx0, Star, idx0+1)))
                }
                Some((idx0, '~')) => {
                    match self.bump() {
                        Some((idx1, '~')) => {
                            self.bump();
                            Some(Ok((idx0, TildeTilde, idx1+1)))
                        }
                        _ => {
                            Some(Err(UnrecognizedToken(idx0)))
                        }
                    }
                }
                Some((idx0, '_')) => {
                    self.bump();
                    Some(Ok((idx0, Underscore, idx0+1)))
                }
                Some((idx0, '`')) => {
                    self.bump();
                    Some(self.escape(idx0))
                }
                Some((idx0, '\'')) => {
                    self.bump();
                    Some(Ok(self.lifetime(idx0)))
                }
                Some((idx0, '"')) => {
                    self.bump();
                    Some(self.string_literal(idx0))
                }
                Some((idx0, '/')) => {
                    match self.bump() {
                        Some((_, '/')) => {
                            self.take_until(|c| c == '\n');
                            continue;
                        }
                        _ => {
                            Some(Err(UnrecognizedToken(idx0)))
                        }
                    }
                }
                Some((idx0, c)) if is_identifier_start(c) => {
                    Some(self.identifierish(idx0))
                }
                Some((_, c)) if c.is_whitespace() => {
                    self.bump();
                    continue;
                }
                Some((idx, _)) => {
                    Some(Err(UnrecognizedToken(idx)))
                }
                None => {
                    None
                }
            };
        }
    }
}

fn is_identifier_start(c: char) -> bool {
    UnicodeXID::is_xid_start(c)
}

fn is_identifier_continue(c: char) -> bool {
    UnicodeXID::is_xid_continue(c)
}
