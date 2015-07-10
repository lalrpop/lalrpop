//! Validate checks some basic safety conditions.

use super::{NormResult, NormError};
use grammar::parse_tree::*;
use intern::{read, InternedString};
use regex::Regex;
use util::{Map, Multimap, Set};

#[cfg(test)]
mod test;

thread_local! {
    static IDENTIFIER_RE: Regex = Regex::new("^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap()
}

pub fn validate(grammar: &Grammar) -> NormResult<()> {
    let globals = ScopeChain {
        previous: None,
        nonterminals: grammar.items
                             .iter()
                             .filter_map(|item| item.as_nonterminal())
                             .map(|nt| (nt.name, Def::TopLevel(nt.args.len())))
                             .collect()
    };

    let conversions: Set<_> =
        grammar.items
               .iter()
               .filter_map(|item| item.as_extern_token())
               .flat_map(|tt| tt.enum_token.conversions.iter().map(|conversion| conversion.from))
               .collect();

    let validator = Validator {
        grammar: grammar,
        globals: globals,
        conversions: conversions,
    };

    validator.validate()
}

struct Validator<'grammar> {
    grammar: &'grammar Grammar,
    globals: ScopeChain<'grammar>,
    conversions: Set<TerminalString>,
}

#[derive(Copy, Clone, Debug)]
enum Def {
    TopLevel(usize), // argument is the number of macro arguments
    MacroArg
}

#[derive(Debug)]
struct ScopeChain<'scope> {
    previous: Option<&'scope ScopeChain<'scope>>,
    nonterminals: Map<NonterminalString, Def>,
}

impl<'grammar> Validator<'grammar> {
    fn validate(&self) -> NormResult<()> {
        for item in &self.grammar.items {
            match *item {
                GrammarItem::Use(..) => { }
                GrammarItem::ExternToken(ref data) => {
                    for conversion in &data.enum_token.conversions {
                        if !self.is_identifier(conversion.to) {
                            return_err!(conversion.span,
                                        "`{}` is not a Rust identifier",
                                        conversion.to)
                        }
                    }
                }
                GrammarItem::Nonterminal(ref data) => {
                    let nonterminals = try!(self.validate_macro_args(data.span, &data.args));
                    let locals = ScopeChain {
                        previous: Some(&self.globals),
                        nonterminals: nonterminals,
                    };
                    for alternative in &data.alternatives {
                        try!(self.validate_alternative(&locals, alternative));
                    }
                }
            }
        }
        Ok(())
    }

    fn validate_macro_args(&self,
                           span: Span,
                           args: &[NonterminalString])
                           -> NormResult<Map<NonterminalString, Def>> {
        for (index, arg) in args.iter().enumerate() {
            if args[..index].contains(&arg) {
                return_err!(span, "multiple macro arguments declared with the name `{}`", arg);
            }
        }
        Ok(args.iter().map(|&nt| (nt, Def::MacroArg)).collect())
    }

    fn validate_alternative(&self,
                            scope: &ScopeChain,
                            alternative: &Alternative)
                            -> NormResult<()> {
        if let Some(ref condition) = alternative.condition {
            let def = try!(self.validate_nt(scope, condition.span, condition.lhs));
            match def {
                Def::MacroArg => { /* OK */ }
                Def::TopLevel(_) => {
                    return_err!(condition.span,
                                "only macro arguments can be used in conditions, not `{}`",
                                condition.lhs);
                }
            }
        }

        try!(self.validate_expr(scope, &alternative.expr));

        Ok(())
    }

    fn validate_expr(&self,
                     scope: &ScopeChain,
                     expr: &ExprSymbol)
                     -> NormResult<()> {
        for symbol in &expr.symbols {
            try!(self.validate_symbol(scope, symbol));
        }

        let chosen: Vec<&Symbol> =
            expr.symbols.iter()
                        .filter(|sym| match sym.kind {
                            SymbolKind::Choose(_) => true,
                            _ => false,
                        })
                        .collect();

        let named: Multimap<InternedString, &Symbol> =
            expr.symbols.iter()
                        .filter_map(|sym| match sym.kind {
                            SymbolKind::Name(nt, _) => Some((nt, sym)),
                            _ => None,
                        })
                        .collect();

        if !chosen.is_empty() && !named.is_empty() {
            return_err!(chosen[0].span,
                        "anonymous symbols like this one cannot be combined with \
                         named symbols like `{}`",
                        named.into_iter().next().unwrap().1[0]);
        }

        for (name, syms) in named.into_iter() {
            if syms.len() > 1{
                return_err!(syms[1].span,
                            "multiple symbols named `{}` are not permitted",
                            name);
            }
        }

        Ok(())
    }

    fn validate_symbol(&self,
                       scope: &ScopeChain,
                       symbol: &Symbol)
                       -> NormResult<()> {
        match symbol.kind {
            SymbolKind::Expr(ref expr) => {
                try!(self.validate_expr(scope, expr));
            }
            SymbolKind::Terminal(term) => {
                try!(self.validate_terminal(symbol.span, term));
            }
            SymbolKind::Nonterminal(nt) => {
                try!(self.validate_nt(scope, symbol.span, nt));
            }
            SymbolKind::Macro(ref msym) => {
                debug_assert!(msym.args.len() > 0);
                let def = try!(self.validate_nt(scope, symbol.span, msym.name));
                match def {
                    Def::MacroArg => {
                        return_err!(symbol.span, "`{}` is not a macro", msym.name)
                    }
                    Def::TopLevel(arity) => {
                        if arity == 0 {
                            return_err!(symbol.span, "`{}` is not a macro", msym.name);
                        } else if arity != msym.args.len() {
                            return_err!(symbol.span,
                                        "wrong number of arguments to `{}`: \
                                         expected {}, found {}",
                                        msym.name, arity, msym.args.len());
                        }
                    }
                }
            }
            SymbolKind::Repeat(ref repeat) => {
                try!(self.validate_symbol(scope, &repeat.symbol));
            }
            SymbolKind::Choose(ref sym) | SymbolKind::Name(_, ref sym) => {
                try!(self.validate_symbol(scope, sym));
            }
        }

        Ok(())
    }

    fn validate_terminal(&self,
                         span: Span,
                         term: TerminalString)
                         -> NormResult<()> {
        // if this is a valid Rust identifier, then the
        // terminal is accepted
        if self.is_identifier(term.0) {
            return Ok(());
        }

        // otherwise, a remapping must have been defined
        if self.conversions.contains(&term) {
            return Ok(());
        }

        return_err!(span, "terminal `{}` is neither a valid Rust identifier \
                           and nor does it have a defined conversion", term);
    }

    fn is_identifier(&self, term: InternedString) -> bool {
        IDENTIFIER_RE.with(|identifier_re|
                           read(|interner|
                                identifier_re.is_match(interner.data(term))))
    }

    fn validate_nt(&self,
                   scope: &ScopeChain,
                   span: Span,
                   nt: NonterminalString)
                   -> NormResult<Def> {
        match scope.def(nt) {
            Some(def) => Ok(def),
            None => return_err!(span, "no definition found for nonterminal `{}`", nt)
        }
    }
}

impl<'scope> ScopeChain<'scope> {
    fn def(&self, nt: NonterminalString) -> Option<Def> {
        self.nonterminals.get(&nt)
                         .cloned()
                         .or_else(|| self.previous.and_then(|s| s.def(nt)))
    }
}
