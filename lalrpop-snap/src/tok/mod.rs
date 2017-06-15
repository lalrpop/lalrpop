//! A tokenizer for use in LALRPOP itself.

use std::str::CharIndices;
use unicode_xid::UnicodeXID;

use self::ErrorCode::*;
use self::Tok::*;

#[cfg(test)]
mod test;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    pub location: usize,
    pub code: ErrorCode
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ErrorCode {
    UnrecognizedToken,
    UnterminatedEscape,
    UnterminatedStringLiteral,
    UnterminatedCharacterLiteral,
    UnterminatedCode,
    ExpectedStringLiteral,
}

fn error<T>(c: ErrorCode, l: usize) -> Result<T,Error> {
    Err(Error { location: l, code: c })
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tok<'input> {
    // Keywords;
    Enum,
    Extern,
    Grammar,
    Match,
    Else,
    If,
    Mut,
    Pub,
    Type,

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
    CharLiteral(&'input str), // excludes the `'`
    RegexLiteral(&'input str), // excludes the `r"` and `"`

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
    Hash,
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
    Bang
}

pub struct Tokenizer<'input> {
    text: &'input str,
    chars: CharIndices<'input>,
    lookahead: Option<(usize, char)>,
    shift: usize,
}

pub type Spanned<T> = (usize, T, usize);

const KEYWORDS: &'static [(&'static str, Tok<'static>)] = &[
    ("enum", Enum),
    ("extern", Extern),
    ("grammar", Grammar),
    ("match", Match),
    ("else", Else),
    ("if", If),
    ("mut", Mut),
    ("pub", Pub),
    ("type", Type),
    ];


impl<'input> Tokenizer<'input> {
    pub fn new(text: &'input str, shift: usize) -> Tokenizer<'input> {
        let mut t = Tokenizer {
            text: text,
            chars: text.char_indices(),
            lookahead: None,
            shift: shift,
        };
        t.bump();
        t
    }

    fn next_unshifted(&mut self) -> Option<Result<Spanned<Tok<'input>>, Error>> {
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
                            Some(Ok((idx0, Bang, idx0+1)))
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
                            Some(error(UnrecognizedToken, idx0))
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
                Some((idx0, '#')) => {
                    self.bump();
                    Some(Ok((idx0, Hash, idx0+1)))
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
                        Some((idx1, 'L')) => {
                            self.bump();
                            Some(Ok((idx0, Lookahead, idx1+1)))
                        }
                        Some((idx1, 'R')) => {
                            self.bump();
                            Some(Ok((idx0, Lookbehind, idx1+1)))
                        }
                        _ => {
                            Some(error(UnrecognizedToken, idx0))
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
                            Some(error(UnrecognizedToken, idx0))
                        }
                    }
                }
                Some((idx0, '`')) => {
                    self.bump();
                    Some(self.escape(idx0))
                }
                Some((idx0, '\'')) => {
                    self.bump();
                    Some(self.lifetimeish(idx0))
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
                            Some(error(UnrecognizedToken, idx0))
                        }
                    }
                }
                Some((idx0, c)) if is_identifier_start(c) => {
                    if c == 'r' {
                        // watch out for r"..." or r#"..."# strings
                        self.bump();
                        match self.lookahead {
                            Some((_, '#')) |
                            Some((_, '"')) => {
                                Some(self.regex_literal(idx0))
                            }
                            _ => {
                                // due to the particulars of how identifierish works,
                                // it's ok that we already consumed the 'r', because the
                                // identifier will run from idx0 (the 'r') to the end
                                Some(self.identifierish(idx0))
                            }
                        }
                    } else {
                        Some(self.identifierish(idx0))
                    }
                }
                Some((_, c)) if c.is_whitespace() => {
                    self.bump();
                    continue;
                }
                Some((idx, _)) => {
                    Some(error(UnrecognizedToken, idx))
                }
                None => {
                    None
                }
            };
        }
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
                        error(UnrecognizedToken, idx0)
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
                error(UnterminatedCode, idx0)
            }
        }
    }

    fn code(&mut self, idx0: usize, open_delims: &str, close_delims: &str) -> Result<usize, Error> {
        // This is the interesting case. To find the end of the code,
        // we have to scan ahead, matching (), [], and {}, and looking
        // for a suitable terminator: `,`, `;`, `]`, `}`, or `)`.
        // Additionaly we had to take into account that we can encounter an character literal
        // equal to one of delimeters.
        let mut balance = 0; // number of unclosed `(` etc
        loop {
            if let Some((idx, c)) = self.lookahead {
                if c == '"' {
                    self.bump();
                    try!(self.string_literal(idx)); // discard the produced token
                    continue;
                } else if c == '\'' {
                    self.bump();
                    if self.take_lifetime_or_character_literal().is_none() {
                        return error(UnterminatedCharacterLiteral, idx);
                    }
                    continue;
                } else if c == 'r' {
                    self.bump();
                    if let Some((idx, '#')) = self.lookahead {
                        try!(self.regex_literal(idx));
                    }
                    continue;
                } else if c == '/' {
                    self.bump();
                    if let Some((_, '/')) = self.lookahead {
                        self.take_until(|c| c == '\n');
                    }
                    continue;
                } else if open_delims.find(c).is_some() {
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
                return error(UnterminatedCode, idx0);
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
                error(UnterminatedEscape, idx0)
            }
        }
    }

    fn take_lifetime_or_character_literal(&mut self) -> Option<usize> {
        // try to decide if `'` is for lifetime or it oppens a character literal

        let forget_character = |p : (usize, char)| { p.0 };

        self.lookahead.and_then( |(_, c)| {
            if c == '\\' {
                // escape after `'` => it had to be character literal token, consume
                self.take_until_and_consume_terminating_character(|c:char| { c == '\'' })
            } else {
                // no escape, then we require to see next `'` or we assume it was lifetime
                self.bump().and_then( |(idx, c)| {
                    if c == '\'' {
                        self.bump().map(forget_character)
                    } else {
                        Some(idx)
                    }
                })
            }
        })
    }

    fn string_or_char_literal(&mut self,
                              idx0: usize,
                              quote: char,
                              variant: fn(&'input str) -> Tok<'input>)
                              -> Option<Spanned<Tok<'input>>> {
        let mut escape = false;
        let terminate = |c: char| {
            if escape {
                escape = false;
                false
            } else if c == '\\' {
                escape = true;
                false
            } else if c == quote {
                true
            } else {
                false
            }
        };

        match self.take_until(terminate) {
            Some(idx1) => {
                self.bump(); // consume the closing quote
                let text = &self.text[idx0+1..idx1]; // do not include quotes in the str
                Some((idx0, variant(text), idx1+1))
            }
            None => {
                None
            }
        }
    }

    fn string_literal(&mut self, idx0: usize) -> Result<Spanned<Tok<'input>>, Error> {
        match self.string_or_char_literal(idx0, '"', StringLiteral) {
            Some(x) => Ok(x),
            None => {
                error(UnterminatedStringLiteral, idx0)
            }
        }
    }

    // parses `r#"..."#` (for some number of #), starts after the `r`
    // has been consumed; idx0 points at the `r`
    fn regex_literal(&mut self, idx0: usize) -> Result<Spanned<Tok<'input>>, Error> {
        match self.take_while(|c| c == '#') {
            Some(idx1) if self.lookahead == Some((idx1, '"')) => {
                self.bump();
                let hashes = idx1 - idx0 - 1;
                let mut state = 0;
                let end_of_regex = |c: char| {
                    if state > 0 {
                        // state N>0 means: observed n-1 hashes
                        if c == '#' {
                            state += 1;
                        } else {
                            state = 0;
                        }
                    }

                    // state 0 means: not yet seen the `"`
                    if state == 0 && c == '"' {
                        state = 1;
                    }

                    state == (hashes + 1)
                };
                match self.take_until(end_of_regex) {
                    Some(idx1) => { // idx1 is the closing quote
                        self.bump();
                        let start = idx0 + 2 + hashes; // skip the `r###"`
                        let end = idx1 - hashes; // skip the `###`.
                        Ok((idx0, RegexLiteral(&self.text[start..end]), idx1 + 1))
                    }
                    None => {
                        error(UnterminatedStringLiteral, idx0)
                    }
                }
            }
            Some(idx1) => {
                error(ExpectedStringLiteral, idx1)
            }
            None => {
                error(UnterminatedStringLiteral, idx0)
            }
        }
    }

    // Saw a `'`, could either be: `'a` or `'a'`.
    fn lifetimeish(&mut self, idx0: usize) -> Result<Spanned<Tok<'input>>, Error> {
        match self.lookahead {
            None => {
                error(UnterminatedCharacterLiteral, idx0)
            },

            Some((_, c)) => {
                if is_identifier_start(c) {
                    let (start, word, end) = self.word(idx0);
                    match self.lookahead {
                        Some((idx2, '\'')) => {
                            self.bump();
                            let text = &self.text[idx0+1..idx2];
                            Ok((idx0, CharLiteral(text), idx2 + 1))
                        }
                        _ => {
                            Ok((start, Lifetime(word), end))
                        }
                    }
                } else {
                    match self.string_or_char_literal(idx0, '\'', CharLiteral) {
                        Some(x) => Ok(x),
                        None => error(UnterminatedCharacterLiteral, idx0),
                    }
                }
            }
        }
    }

    fn identifierish(&mut self, idx0: usize) -> Result<Spanned<Tok<'input>>, Error> {
        let (start, word, end) = self.word(idx0);

        if word == "_" {
            return Ok((idx0, Underscore, idx0+1));
        }

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

    fn take_until_and_consume_terminating_character<F>(&mut self, terminate : F) -> Option<usize>
        where F: FnMut(char) -> bool
    {
        self.take_until(terminate).and_then(|_| {
            self.bump().map(|p| {p.0})
        })
    }
}

impl<'input> Iterator for Tokenizer<'input> {
    type Item = Result<Spanned<Tok<'input>>, Error>;

    fn next(&mut self) -> Option<Result<Spanned<Tok<'input>>, Error>> {
        match self.next_unshifted() {
            None =>
                None,
            Some(Ok((l, t, r))) =>
                Some(Ok((l+self.shift, t, r+self.shift))),
            Some(Err(Error { location, code })) =>
                Some(Err(Error { location: location+self.shift, code: code })),
        }
    }
}

fn is_identifier_start(c: char) -> bool {
    UnicodeXID::is_xid_start(c) || c == '_'
}

fn is_identifier_continue(c: char) -> bool {
    UnicodeXID::is_xid_continue(c) || c == '_'
}

