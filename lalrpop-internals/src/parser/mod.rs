use std::iter;

use crate::grammar::parse_tree::*;
use crate::grammar::pattern::*;
use crate::tok;

#[rustfmt::skip]
#[allow(dead_code)]
#[allow(clippy::all)]
mod lrgrammar;


#[cfg(test)]
mod test;

// The TypeRef and GrammarWhereClauses variants have data that is only read under cfg(test) (the
// parse_type_ref() and parse_where_clauses() functions lower in this file).  Those functions use
// the parser!() macro, which expects all variants to have a single data field.  They are set in
// the parser.  So to have those fields only in the test configuration requires changes at multiple
// code points across several files to define both a cfg(test) variant and a cfg(not(test))
// variant, reducing readability.
#[allow(dead_code)]
pub enum Top {
    Grammar(Grammar),
    Pattern(Pattern<TypeRef>),
    MatchMapping(MatchMapping),
    TypeRef(TypeRef),
    GrammarWhereClauses(Vec<WhereClause<TypeRef>>),
}

pub type ParseError<'input> = lalrpop_util::ParseError<usize, tok::Tok<'input>, tok::Error>;

macro_rules! parser {
    ($input:expr, $offset:expr, $pat:ident, $tok:ident) => {{
        let input = $input;
        let tokenizer =
            iter::once(Ok((0, tok::Tok::$tok, 0))).chain(tok::Tokenizer::new(input, $offset));
        lrgrammar::TopParser::new()
            .parse(input, tokenizer)
            .map(|top| match top {
                Top::$pat(x) => x,
                _ => unreachable!(),
            })
    }};
}

pub fn parse_grammar(input: &str) -> Result<Grammar, ParseError<'_>> {
    let mut grammar = parser!(input, 0, Grammar, StartGrammar)?;

    // find a unique prefix that does not appear anywhere in the input
    while input.contains(&grammar.prefix) {
        grammar.prefix.push('_');
    }

    Ok(grammar)
}

fn parse_pattern(input: &str, offset: usize) -> Result<Pattern<TypeRef>, ParseError<'_>> {
    parser!(input, offset, Pattern, StartPattern)
}

fn parse_match_mapping(input: &str, offset: usize) -> Result<MatchMapping, ParseError<'_>> {
    parser!(input, offset, MatchMapping, StartMatchMapping)
}

#[cfg(test)]
pub fn parse_type_ref(input: &str) -> Result<TypeRef, ParseError<'_>> {
    parser!(input, 0, TypeRef, StartTypeRef)
}

#[cfg(test)]
pub fn parse_where_clauses(input: &str) -> Result<Vec<WhereClause<TypeRef>>, ParseError<'_>> {
    parser!(input, 0, GrammarWhereClauses, StartGrammarWhereClauses)
}
