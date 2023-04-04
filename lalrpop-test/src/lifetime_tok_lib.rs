/// A token that includes a lifetime parameter.
#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum LtTok<'input> {
    Other(&'input str),
    Dummy,
}

pub fn lt_tokenize(s: &str) -> Vec<LtTok<'_>> {
    let mut tokens = vec![];
    for (index, c) in s.char_indices() {
        if !c.is_whitespace() {
            tokens.push(LtTok::Other(&s[index..index + c.len_utf8()]));
        }
    }
    tokens
}
