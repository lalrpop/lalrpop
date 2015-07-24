use grammar::parse_tree::*;
use grammar::pattern::*;
use lalrpop_util;
use tok;

#[allow(dead_code)]
mod lrgrammar;

pub type ParseError<'input> = lalrpop_util::ParseError<usize, tok::Tok<'input>, tok::Error>;

pub fn parse_grammar<'input>(input: &'input str)
                             -> Result<Grammar, ParseError<'input>>
{
    let tokenizer = tok::Tokenizer::new(input, 0);
    lrgrammar::parse_Grammar(input, tokenizer)
}

pub fn parse_pattern<'input>(input: &'input str, offset: usize)
                             -> Result<Pattern<TypeRef>, ParseError<'input>>
{
    let tokenizer = tok::Tokenizer::new(input, offset);
    lrgrammar::parse_Pattern(input, tokenizer)
}

#[cfg(test)]
pub fn parse_type_ref<'input>(input: &'input str)
                              -> Result<TypeRef, ParseError<'input>>
{
    let tokenizer = tok::Tokenizer::new(input);
    lrgrammar::parse_TypeRef(input, tokenizer)
}
