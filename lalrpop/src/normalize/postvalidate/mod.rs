//! Validate that terminal IDs have conversions. This can only
//! be done after macro expansion because some macro arguments
//! never make it into an actual production and are only used
//! in `if` conditions; we use string literals for those,
//! but they do not have to have a defined conversion.

use super::{NormResult, NormError};

use grammar::parse_tree::*;
use util::{Set};

#[cfg(test)]
mod test;

pub fn validate(grammar: &Grammar) -> NormResult<()> {
    let conversions: Set<_> =
        grammar.items
               .iter()
               .filter_map(|item| item.as_extern_token())
               .flat_map(|tt| tt.enum_token.conversions.iter().map(|conversion| conversion.from))
               .collect();

    let validator = Validator {
        grammar: grammar,
        conversions: conversions,
    };

    validator.validate()
}

struct Validator<'grammar> {
    grammar: &'grammar Grammar,
    conversions: Set<TerminalString>,
}

impl<'grammar> Validator<'grammar> {
    fn validate(&self) -> NormResult<()> {
        for item in &self.grammar.items {
            match *item {
                GrammarItem::Use(..) => { }
                GrammarItem::ExternToken(_) => { }
                GrammarItem::Nonterminal(ref data) => {
                    for alternative in &data.alternatives {
                        try!(self.validate_alternative(alternative));
                    }
                }
            }
        }
        Ok(())
    }

    fn validate_alternative(&self, alternative: &Alternative) -> NormResult<()> {
        assert!(alternative.condition.is_none()); // macro expansion should have removed these
        try!(self.validate_expr(&alternative.expr));
        Ok(())
    }

    fn validate_expr(&self, expr: &ExprSymbol) -> NormResult<()> {
        for symbol in &expr.symbols {
            try!(self.validate_symbol(symbol));
        }
        Ok(())
    }

    fn validate_symbol(&self, symbol: &Symbol) -> NormResult<()> {
        match symbol.kind {
            SymbolKind::Expr(ref expr) => {
                try!(self.validate_expr(expr));
            }
            SymbolKind::Terminal(term) => {
                try!(self.validate_terminal(symbol.span, term));
            }
            SymbolKind::Nonterminal(_) => {
            }
            SymbolKind::Macro(..) => {
                panic!("macro not removed: {:?}", symbol);
            }
            SymbolKind::Repeat(ref repeat) => {
                try!(self.validate_symbol(&repeat.symbol));
            }
            SymbolKind::Choose(ref sym) | SymbolKind::Name(_, ref sym) => {
                try!(self.validate_symbol(sym));
            }
            SymbolKind::Lookahead | SymbolKind::Lookbehind => { }
        }

        Ok(())
    }

    fn validate_terminal(&self, span: Span, term: TerminalString) -> NormResult<()> {
        if !self.conversions.contains(&term) {
            return_err!(span, "terminal `{}` does not have a pattern defined for it",
                        term);
        }

        Ok(())
    }
}
