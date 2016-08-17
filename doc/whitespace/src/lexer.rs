pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

#[derive(Debug)]
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
        Lexer { chars: input.char_indices() }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Tok, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            return Some(Ok(match self.chars.next() {
                Some((i, ' ')) => (i, Tok::Space, i+1),
                Some((i, '\t')) => (i, Tok::Tab, i+1),
                Some((i, '\n')) => (i, Tok::Linefeed, i+1),

                None => return None, // End of file
                _ => continue, // Comment
            }))
        }
    }
}

#[test]
fn skip_comments() {
    let source = "The quick brown fox jumped over the lazy dog";

    assert_eq!(Lexer::new(source).count(), 8);

    assert!(Lexer::new(source).all(|tok| match tok {
        Ok((_, Tok::Space, _)) => true,
        _ => false,
    }));
}
