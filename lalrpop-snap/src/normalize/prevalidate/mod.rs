//! Validate checks some basic safety conditions.

use super::{NormResult, NormError};
use super::norm_util::{self, Symbols};

use grammar::parse_tree::*;
use grammar::repr;
use intern::{intern, InternedString};
use util::{Multimap, Sep, set};

#[cfg(test)]
mod test;

pub fn validate(grammar: &Grammar) -> NormResult<()> {
    let extern_token: Option<&ExternToken> =
        grammar.items
               .iter()
               .filter_map(|item| item.as_extern_token())
               .next();

    let validator = Validator {
        grammar: grammar,
        extern_token: extern_token,
    };

    validator.validate()
}

struct Validator<'grammar> {
    grammar: &'grammar Grammar,
    extern_token: Option<&'grammar ExternToken>,
}

impl<'grammar> Validator<'grammar> {
    fn validate(&self) -> NormResult<()> {
        if let Some(ref algorithm) = self.grammar.algorithm {
            match repr::Algorithm::from_str(algorithm.text) {
                Some(_) => { }
                None => {
                    return_err!(
                        algorithm.span,
                        "unrecognized algorithm `{}`", algorithm.text);
                }
            }
        }

        for item in &self.grammar.items {
            match *item {
                GrammarItem::Use(..) => { }
                GrammarItem::ExternToken(ref data) => {
                    if data.span != self.extern_token.unwrap().span {
                        return_err!(
                            data.span,
                            "multiple extern definitions are not permitted");
                    }

                    let allowed_names = vec![intern(LOCATION), intern(ERROR)];
                    let mut new_names = set();
                    for associated_type in &data.associated_types {
                        if !allowed_names.contains(&associated_type.type_name) {
                            return_err!(
                                associated_type.type_span,
                                "associated type `{}` not recognized, \
                                 try one of the following: {}",
                                associated_type.type_name,
                                Sep(", ", &allowed_names));
                        } else if !new_names.insert(associated_type.type_name) {
                            return_err!(
                                associated_type.type_span,
                                "associated type `{}` already specified",
                                associated_type.type_name);
                        }
                    }
                }
                GrammarItem::Nonterminal(ref data) => {
                    for annotation in &data.annotations {
                        return_err!(annotation.id_span,
                                    "unrecognized annotation `{}`",
                                    annotation.id);
                    }

                    for alternative in &data.alternatives {
                        try!(self.validate_alternative(alternative));
                    }
                }
                GrammarItem::InternToken(..) => { }
            }
        }
        Ok(())
    }

    fn validate_alternative(&self,
                            alternative: &Alternative)
                            -> NormResult<()> {
        try!(self.validate_expr(&alternative.expr));

        match norm_util::analyze_expr(&alternative.expr) {
            Symbols::Named(syms) => {
                if alternative.action.is_none() {
                    let sym =
                        syms.iter()
                            .map(|&(_, _, sym)| sym)
                            .next()
                            .unwrap();
                    return_err!(
                        sym.span,
                        "named symbols (like `{}`) require a custom action",
                        sym);
                }
            }
            Symbols::Anon(_) => { }
        }

        Ok(())
    }

    fn validate_expr(&self,
                     expr: &ExprSymbol)
                     -> NormResult<()> {
        for symbol in &expr.symbols {
            try!(self.validate_symbol(symbol));
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
                       symbol: &Symbol)
                       -> NormResult<()> {
        match symbol.kind {
            SymbolKind::Expr(ref expr) => {
                try!(self.validate_expr(expr));
            }
            SymbolKind::AmbiguousId(_) => {
                /* see resolve */
            }
            SymbolKind::Terminal(_) => {
                /* see postvalidate! */
            }
            SymbolKind::Nonterminal(_) => {
                /* see resolve */
            }
            SymbolKind::Macro(ref msym) => {
                debug_assert!(msym.args.len() > 0);
                for arg in &msym.args {
                    try!(self.validate_symbol(arg));
                }
            }
            SymbolKind::Repeat(ref repeat) => {
                try!(self.validate_symbol(&repeat.symbol));
            }
            SymbolKind::Choose(ref sym) | SymbolKind::Name(_, ref sym) => {
                try!(self.validate_symbol(sym));
            }
            SymbolKind::Lookahead | SymbolKind::Lookbehind => {
                // if using an internal tokenizer, lookahead/lookbehind are ok.
                if let Some(extern_token) = self.extern_token {
                    if extern_token.enum_token.is_some() {
                        // otherwise, the Location type must be specified.
                        let loc = intern(LOCATION);
                        if self.extern_token.unwrap().associated_type(loc).is_none() {
                            return_err!(
                                symbol.span,
                                "lookahead/lookbehind require you to declare the type of \
                                 a location; add a `type {} = ..` statement to the extern token \
                                 block",
                                LOCATION);
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
