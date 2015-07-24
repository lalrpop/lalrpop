//! Validate checks some basic safety conditions.

use super::{NormResult, NormError};
use super::norm_util::{self, Symbols};

use grammar::parse_tree::*;
use intern::{intern, InternedString};
use util::{Map, Multimap, Sep, set};

#[cfg(test)]
mod test;

pub fn validate(grammar: &Grammar) -> NormResult<()> {
    let globals = ScopeChain {
        previous: None,
        nonterminals: grammar.items
                             .iter()
                             .filter_map(|item| item.as_nonterminal())
                             .map(|nt| (nt.name, Def::TopLevel(nt.args.len())))
                             .collect()
    };

    let extern_token: Option<&ExternToken> =
        grammar.items
               .iter()
               .filter_map(|item| item.as_extern_token())
               .next();

    let validator = Validator {
        grammar: grammar,
        globals: globals,
        extern_token: extern_token,
    };

    validator.validate()
}

struct Validator<'grammar> {
    grammar: &'grammar Grammar,
    extern_token: Option<&'grammar ExternToken>,
    globals: ScopeChain<'grammar>,
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
                    if data.span != self.extern_token.unwrap().span {
                        return_err!(
                            data.span,
                            "multiple extern token definitions are not permitted");
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

                    match data.enum_token.type_name {
                        TypeRef::Id(_) | TypeRef::Nominal { .. } => { /* OK */ }
                        _ => {
                            return_err!(
                                data.enum_token.type_span,
                                "expected a nominal type here, like `Token` or `foo::Token<T>`");
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
            SymbolKind::Terminal(_) => {
                /* see postvalidate! */
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

                for arg in &msym.args {
                    try!(self.validate_symbol(scope, arg));
                }
            }
            SymbolKind::Repeat(ref repeat) => {
                try!(self.validate_symbol(scope, &repeat.symbol));
            }
            SymbolKind::Choose(ref sym) | SymbolKind::Name(_, ref sym) => {
                try!(self.validate_symbol(scope, sym));
            }
            SymbolKind::Lookahead | SymbolKind::Lookbehind => {
                if self.extern_token.is_none() {
                    return_err!(symbol.span,
                                "lookahead/lookbehind not permitted without \
                                 an extern token declaration");
                }

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

        Ok(())
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
