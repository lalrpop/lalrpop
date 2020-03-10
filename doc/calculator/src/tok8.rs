use crate::ast::Opcode;

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

#[derive(Copy, Clone, Debug)]
pub enum Tok<'input> {
    NumSymbol(&'input str),
    FactorOp(Opcode),
    ExprOp(Opcode),
    ParenOpen,
    ParenClose,
}

use std::str::CharIndices;

pub struct Lexer<'input> {
    chars: std::iter::Peekable<CharIndices<'input>>,
    input: &'input str,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            chars: input.char_indices().peekable(),
            input,
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Tok<'input>, usize, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.chars.next() {
                Some((_, ' ')) | Some((_, '\n')) | Some((_, '\t')) => continue,
                Some((i, ')')) => return Some(Ok((i, Tok::ParenClose, i + 1))),
                Some((i, '(')) => return Some(Ok((i, Tok::ParenOpen, i + 1))),
                Some((i, '+')) => return Some(Ok((i, Tok::ExprOp(Opcode::Add), i + 1))),
                Some((i, '-')) => return Some(Ok((i, Tok::ExprOp(Opcode::Sub), i + 1))),
                Some((i, '*')) => return Some(Ok((i, Tok::FactorOp(Opcode::Mul), i + 1))),
                Some((i, '/')) => return Some(Ok((i, Tok::FactorOp(Opcode::Div), i + 1))),

                None => return None, // End of file
                Some((i, _)) => loop {
                    match self.chars.peek() {
                        Some((j, ')')) | Some((j, '(')) | Some((j, '+')) | Some((j, '-'))
                        | Some((j, '*')) | Some((j, '/')) | Some((j, ' ')) => {
                            return Some(Ok((i, Tok::NumSymbol(&self.input[i..*j]), *j)))
                        }
                        None => {
                            return Some(Ok((
                                i,
                                Tok::NumSymbol(&self.input[i..]),
                                self.input.len(),
                            )))
                        }
                        _ => {}
                    }
                    self.chars.next();
                },
            }
        }
    }
}
