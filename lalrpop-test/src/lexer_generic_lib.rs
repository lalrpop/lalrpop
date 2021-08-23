/// Token returned by the lexers
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Num(u32),
    Plus,
}

impl From<Token> for u32 {
    fn from(value: Token) -> Self {
        match value {
            Token::Num(num) => num,
            _ => u32::MAX,
        }
    }
}

/// Generic trait for lexing
pub trait LexerTrait {
    /// Type of error returned by the lexer
    type Error: std::fmt::Debug + std::cmp::PartialEq;
}

/// A concrete lexer
#[derive(Clone)]
pub struct Lexer<'i> {
    source: &'i str,
    input: std::iter::Peekable<std::str::CharIndices<'i>>,
}

impl<'i> Lexer<'i> {
    pub fn new(source: &'i str) -> Self {
        Self {
            source,
            input: source.char_indices().peekable(),
        }
    }
}

/// Concrete lexer error
#[derive(Debug, PartialEq)]
pub struct Error(String);

impl<'i> Iterator for Lexer<'i> {
    type Item = Result<(usize, Token, usize), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        // Try to parse a token
        loop {
            if let Some((pos, ch)) = self.input.next() {
                // Skip over whitespace
                if ch.is_ascii_whitespace() {
                    continue;
                }

                // Scan until next boundary
                let start = pos;
                let mut end;
                loop {
                    match self.input.peek() {
                        Some((pos, ch)) => {
                            end = *pos;

                            if ch.is_ascii_whitespace() {
                                break;
                            } else {
                                self.input.next();
                            }
                        }
                        None => {
                            end = self.source.len();
                            break;
                        }
                    }
                }

                // Get slice
                // SAFETY: the indices where returned by char_indices
                let slice = unsafe { self.source.get_unchecked(start..end) };

                eprintln!("{:?}", slice);

                return if slice == "+" {
                    Some(Ok((start, Token::Plus, end)))
                } else if slice.is_empty() {
                    return None;
                } else {
                    match slice.parse::<u32>() {
                        Ok(num) => Some(Ok((start, Token::Num(num), end))),
                        Err(_) => Some(Err(Error(slice.to_owned()))),
                    }
                };
            } else {
                // End of input
                return None;
            }
        }
    }
}

/// Implement LexerTrait
impl<'i> LexerTrait for Lexer<'i> {
    type Error = Error;
}
