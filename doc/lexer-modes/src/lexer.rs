use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

/// The lexer's mode.
///
/// This is shared between the lexer and the parser.
#[derive(Debug)]
pub struct LexerMode {
    /// If `Some`, the next N characters should be returned as a
    /// `Literal` token.
    pub literal: Option<usize>,
}

impl Default for LexerMode {
    fn default() -> Self {
        Self::new()
    }
}

impl LexerMode {
    pub fn new() -> Self {
        Self { literal: None }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum LexicalError {
    LengthOverflow(String),
    TruncatedInput(String),
}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

pub type Spanned<Token, Loc, LexicalError> = Result<(Loc, Token, Loc), LexicalError>;

// The type of the parser's input.
//
// The parser iterators over tuples consisting of the token's starting
// position, the token itself, and the token's ending position.
pub(crate) type LexerItem<Token, Loc, LexicalError> = Spanned<Token, Loc, LexicalError>;

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Token<'a> {
    COLON,

    // Whitespace.
    SPACE,
    HTAB,
    VTAB,
    CR,
    LF,
    FORMFEED,

    // Digits.
    N_0,
    N_1,
    N_2,
    N_3,
    N_4,
    N_5,
    N_6,
    N_7,
    N_8,
    N_9,

    // Other.
    OTHER(&'a [u8]),

    // Returned when the lexer is in literal mode.
    LITERAL(&'a [u8]),
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl<'a> Token<'a> {
    pub fn as_bytes(&self) -> &'a [u8] {
        use self::Token::*;
        match self {
            COLON => b":",

            // Whitespace.
            SPACE => b" ",
            HTAB => b"\t",
            VTAB => &[0x0b],
            CR => &[0x0d],
            LF => &[0x0a],
            FORMFEED => &[0x0c],

            // Digits.
            N_0 => b"0",
            N_1 => b"1",
            N_2 => b"2",
            N_3 => b"3",
            N_4 => b"4",
            N_5 => b"5",
            N_6 => b"6",
            N_7 => b"7",
            N_8 => b"8",
            N_9 => b"9",

            OTHER(bytes) => bytes,

            LITERAL(bytes) => bytes,
        }
    }
}

#[derive(Debug)]
pub struct Lexer<'input> {
    input: &'input [u8],

    // Offset into the original input.
    offset: usize,

    // The lexer's mode.  This is behind a `Rc<RefCell>` so that it
    // is also accessible to the parser.
    pub mode: Rc<RefCell<LexerMode>>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input [u8]) -> Self {
        Lexer {
            input,
            offset: 0,
            mode: Rc::new(RefCell::new(LexerMode::new())),
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = LexerItem<Token<'input>, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        use self::Token::*;

        let mut mode = self.mode.borrow_mut();
        let (len, token) = if let Some(count) = mode.literal.take() {
            // We are in literal model: we return the next characters
            // as-is.

            if self.input.len() < count {
                return Some(Err(LexicalError::TruncatedInput(format!(
                    "Expected {} octets, got {}",
                    count,
                    self.input.len()
                ))));
            } else {
                (count, LITERAL(&self.input[..count]))
            }
        } else {
            // We are in normal mode.  Return the next token.

            match *self.input.first()? as char {
                ':' => (1, COLON),

                // Whitespace.
                ' ' => (1, SPACE),
                '\t' => (1, HTAB),
                '\u{0b}' => (1, VTAB),
                '\u{0d}' => (1, CR),
                '\u{0a}' => (1, LF),
                '\u{0c}' => (1, FORMFEED),

                // Digits.
                '0' => (1, N_0),
                '1' => (1, N_1),
                '2' => (1, N_2),
                '3' => (1, N_3),
                '4' => (1, N_4),
                '5' => (1, N_5),
                '6' => (1, N_6),
                '7' => (1, N_7),
                '8' => (1, N_8),
                '9' => (1, N_9),

                // Other.
                _ => (1, OTHER(&self.input[..1])),
            }
        };

        self.input = &self.input[len..];

        let start = self.offset;
        let end = start + len;
        self.offset += len;

        Some(Ok((start, token, end)))
    }
}

impl<'input> From<&'input [u8]> for Lexer<'input> {
    fn from(i: &'input [u8]) -> Lexer<'input> {
        Lexer::new(i)
    }
}
