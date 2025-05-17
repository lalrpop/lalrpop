pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;
use std::fmt::{Debug, Error, Formatter};

#[derive(PartialEq)]
pub enum ExprSymbol<'input> {
    NumSymbol(&'input str),
    Op(Box<ExprSymbol<'input>>, Opcode, Box<ExprSymbol<'input>>),
}

#[derive(Copy, Clone, PartialEq)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Tok<'input> {
    NumSymbol(&'input str),
    FactorOp(Opcode),
    ExprOp(Opcode),
    ParenOpen,
    ParenClose,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MyLoc {
    pub loc: usize,
}

use std::str::CharIndices;

#[derive(Clone, Debug)]
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
    type Item = Spanned<Tok<'input>, MyLoc, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.chars.next() {
                Some((_, ' ')) | Some((_, '\n')) | Some((_, '\t')) => continue,
                Some((i, ')')) => {
                    return Some(Ok((
                        MyLoc { loc: i },
                        Tok::ParenClose,
                        MyLoc { loc: i + 1 },
                    )))
                }
                Some((i, '(')) => {
                    return Some(Ok((MyLoc { loc: i }, Tok::ParenOpen, MyLoc { loc: i + 1 })))
                }
                Some((i, '+')) => {
                    return Some(Ok((
                        MyLoc { loc: i },
                        Tok::ExprOp(Opcode::Add),
                        MyLoc { loc: i + 1 },
                    )))
                }
                Some((i, '-')) => {
                    return Some(Ok((
                        MyLoc { loc: i },
                        Tok::ExprOp(Opcode::Sub),
                        MyLoc { loc: i + 1 },
                    )))
                }
                Some((i, '*')) => {
                    return Some(Ok((
                        MyLoc { loc: i },
                        Tok::FactorOp(Opcode::Mul),
                        MyLoc { loc: i + 1 },
                    )))
                }
                Some((i, '/')) => {
                    return Some(Ok((
                        MyLoc { loc: i },
                        Tok::FactorOp(Opcode::Div),
                        MyLoc { loc: i + 1 },
                    )))
                }

                None => return None, // End of file
                Some((i, _)) => loop {
                    match self.chars.peek() {
                        Some((j, ')')) | Some((j, '(')) | Some((j, '+')) | Some((j, '-'))
                        | Some((j, '*')) | Some((j, '/')) | Some((j, ' ')) => {
                            return Some(Ok((
                                MyLoc { loc: i },
                                Tok::NumSymbol(&self.input[i..*j]),
                                MyLoc { loc: *j },
                            )))
                        }
                        None => {
                            return Some(Ok((
                                MyLoc { loc: i },
                                Tok::NumSymbol(&self.input[i..]),
                                MyLoc {
                                    loc: self.input.len(),
                                },
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

impl Debug for ExprSymbol<'_> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        use self::ExprSymbol::*;
        match *self {
            NumSymbol(n) => write!(fmt, "{n:?}"),
            Op(ref l, op, ref r) => write!(fmt, "({l:?} {op:?} {r:?})"),
        }
    }
}

impl Debug for Opcode {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        use self::Opcode::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
        }
    }
}

use crate::loc_clone::ExprParser;
use crate::util::expect_debug;

#[test]
fn loc_clone_issue1064() {
    let lexer = Lexer::new("22 * pi + 66");
    let result = ExprParser::new().parse(lexer);
    println!("{result:#?}");
    expect_debug(
        result,
        r#"
Ok(
    (("22" * "pi") + "66")
)
"#
        .trim(),
    );
}
