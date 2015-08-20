//! If an extern token is provided, then this pass validates that
//! terminal IDs have conversions. Otherwise, it generates a
//! tokenizer. This can only be done after macro expansion because
//! some macro arguments never make it into an actual production and
//! are only used in `if` conditions; we use string literals for
//! those, but they do not have to have a defined conversion.

use super::{NormResult, NormError};

use intern;
use lexer::re;
use lexer::dfa::{self, Precedence};
use grammar::parse_tree::*;
use util::{Set};
use util::{map, Map};

#[cfg(test)]
mod test;

pub fn validate(grammar: &mut Grammar) -> NormResult<()> {
    let (has_extern_token, all_literals) = {
        let opt_extern_token = grammar.extern_token();
        let conversions = opt_extern_token.map(|tt| {
            tt.enum_token.conversions.iter()
                                     .map(|conversion| conversion.from)
                                     .collect()
        });

        let mut validator = Validator {
            grammar: grammar,
            all_literals: map(),
            conversions: conversions,
        };

        try!(validator.validate());

        (opt_extern_token.is_some(), validator.all_literals)
    };

    if has_extern_token {
        Ok(())
    } else {
        construct(grammar, all_literals)
    }
}

///////////////////////////////////////////////////////////////////////////
// Validation phase -- this phase walks the grammar and visits all
// terminals. If using an external set of tokens, it checks that all
// terminals have a defined conversion to some pattern. Otherwise,
// it collects all terminals into the `all_literals` set for later use.

struct Validator<'grammar> {
    grammar: &'grammar Grammar,
    all_literals: Map<TerminalLiteral, Span>,
    conversions: Option<Set<TerminalString>>,
}

impl<'grammar> Validator<'grammar> {
    fn validate(&mut self) -> NormResult<()> {
        for item in &self.grammar.items {
            match *item {
                GrammarItem::Use(..) => { }
                GrammarItem::ExternToken(_) => { }
                GrammarItem::InternToken(_) => { }
                GrammarItem::Nonterminal(ref data) => {
                    for alternative in &data.alternatives {
                        try!(self.validate_alternative(alternative));
                    }
                }
            }
        }
        Ok(())
    }

    fn validate_alternative(&mut self, alternative: &Alternative) -> NormResult<()> {
        assert!(alternative.condition.is_none()); // macro expansion should have removed these
        try!(self.validate_expr(&alternative.expr));
        Ok(())
    }

    fn validate_expr(&mut self, expr: &ExprSymbol) -> NormResult<()> {
        for symbol in &expr.symbols {
            try!(self.validate_symbol(symbol));
        }
        Ok(())
    }

    fn validate_symbol(&mut self, symbol: &Symbol) -> NormResult<()> {
        match symbol.kind {
            SymbolKind::Expr(ref expr) => {
                try!(self.validate_expr(expr));
            }
            SymbolKind::Terminal(term) => {
                try!(self.validate_terminal(symbol.span, term));
            }
            SymbolKind::Nonterminal(_) => {
            }
            SymbolKind::Repeat(ref repeat) => {
                try!(self.validate_symbol(&repeat.symbol));
            }
            SymbolKind::Choose(ref sym) | SymbolKind::Name(_, ref sym) => {
                try!(self.validate_symbol(sym));
            }
            SymbolKind::Lookahead | SymbolKind::Lookbehind => {
            }
            SymbolKind::AmbiguousId(id) => {
                panic!("ambiguous id `{}` encountered after name resolution", id)
            }
            SymbolKind::Macro(..) => {
                panic!("macro not removed: {:?}", symbol);
            }
        }

        Ok(())
    }

    fn validate_terminal(&mut self, span: Span, term: TerminalString) -> NormResult<()> {
        match self.conversions {
            // If there is an extern token definition, validate that
            // this terminal has a defined conversion.
            Some(ref c) => {
                if !c.contains(&term) {
                    return_err!(span, "terminal `{}` does not have a pattern defined for it",
                                term);
                }
            }

            // If there is no extern token definition, then collect
            // the terminal literals ("class", r"[a-z]+") into a set.
            None => match term {
                TerminalString::Bare(c) => {
                    // Bare identifiers like `x` can never be resolved
                    // as terminals unless there is a conversion
                    // defined for them that indicates they are a
                    // terminal; otherwise it's just an unresolved
                    // identifier.
                    panic!("bare literal `{}` without extern token definition", c);
                }
                TerminalString::Literal(l) => {
                    self.all_literals.entry(l).or_insert(span);
                }
            }
        }

        Ok(())
    }
}

///////////////////////////////////////////////////////////////////////////
// Construction phase -- if we are constructing a tokenizer, this
// phase builds up an internal token DFA.

pub fn construct(grammar: &mut Grammar, literals_map: Map<TerminalLiteral, Span>) -> NormResult<()> {
    let literals: Vec<TerminalLiteral> =
        literals_map.keys()
                    .cloned()
                    .collect();

    // Build up two vectors, one of parsed regular expressions and
    // one of precedences, that are parallel with `literals`.
    let mut regexs = Vec::with_capacity(literals.len());
    let mut precedences = Vec::with_capacity(literals.len());
    intern::read(|interner| {
        for &literal in &literals {
            match literal {
                TerminalLiteral::Quoted(s) => {
                    precedences.push(Precedence(1));
                    regexs.push(re::parse_literal(interner.data(s)));
                }
            }
        }
    });

    let dfa = match dfa::build_dfa(&regexs, &precedences) {
        Ok(dfa) => dfa,
        Err(ambiguity) => {
            let literal0 = literals[ambiguity.match0.index()];
            let literal1 = literals[ambiguity.match1.index()];
            let span0 = literals_map[&literal0];
            let _span1 = literals_map[&literal1];
            return_err!(
                span0,
                "ambiguity detected between the terminal `{}` and the terminal `{}`",
                literal0, literal1);
        }
    };

    let intern = InternToken {
        literals: literals,
        dfa: dfa
    };

    grammar.items.push(GrammarItem::InternToken(intern));

    Ok(())
}


