pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

#[derive(Copy, Clone, Debug)]
pub enum Tok {
    Space,
    Tab,
    Linefeed,
}

#[derive(Debug)]
pub enum LexicalError {
    // Not possible
}

use std::str::CharIndices;

pub struct Lexer<'input> {
    chars: CharIndices<'input>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            chars: input.char_indices(),
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Tok, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.chars.next() {
                Some((i, ' ')) => return Some(Ok((i, Tok::Space, i + 1))),
                Some((i, '\t')) => return Some(Ok((i, Tok::Tab, i + 1))),
                Some((i, '\n')) => return Some(Ok((i, Tok::Linefeed, i + 1))),

                None => return None, // End of file
                _ => continue,       // Comment; skip this character
            }
        }
    }
}

#[test]
fn skip_comments() {
    let source = "The quick brown fox jumped over the lazy dog";

    assert_eq!(Lexer::new(source).count(), 8);

    assert!(Lexer::new(source).all(|tok| matches!(tok, Ok((_, Tok::Space, _)))));
}
