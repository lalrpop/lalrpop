use super::{NormResult, NormError};
use super::norm_util::{self, AlternativeAction, Symbols};

use std::collections::{HashMap};
use intern::{InternedString};
use grammar::parse_tree::{Alternative, Grammar, GrammarItem,
                          NonterminalData, Span, Symbol, TypeRef};
use grammar::repr::{Types, TypeRepr};

#[cfg(test)]
mod test;

pub fn infer_types(grammar: &Grammar) -> NormResult<Types> {
    let inferencer = try!(TypeInferencer::new(&grammar));
    inferencer.infer_types()
}

struct TypeInferencer<'grammar> {
    stack: Vec<InternedString>,
    nonterminals: HashMap<InternedString, NT<'grammar>>,
    types: Types,
}

#[derive(Copy, Clone)]
struct NT<'grammar> {
    span: Span,
    type_decl: &'grammar Option<TypeRef>,
    alternatives: &'grammar Vec<Alternative>,
}

fn extract_token_type(grammar: &Grammar) -> NormResult<TypeRepr> {
    let mut token_types =
        grammar.items
               .iter()
               .filter_map(|item| {
                   match *item {
                       GrammarItem::TokenType(ref data) => Some(&data.type_name),
                       _ => None,
                   }
               });

    let token_type = token_types.next();
    let token_type = match token_type {
        Some(tt) => tt,
        None => return_err!(grammar.span, "no token type specified")
    };

    if let Some(_) = token_types.next() {
        return_err!(grammar.span, "multiple token types specified");
    }

    Ok(token_type.type_repr())
}

impl<'grammar> TypeInferencer<'grammar> {
    fn new(grammar: &'grammar Grammar) -> NormResult<TypeInferencer<'grammar>> {
        let token_type =
            try!(extract_token_type(grammar));

        let nonterminals =
            grammar.items
                   .iter()
                   .filter_map(|item| {
                       match *item {
                           GrammarItem::TokenType(..) =>
                               None,
                           GrammarItem::Nonterminal(ref data) => {
                               assert!(!data.is_macro_def()); // normalized away by now
                               Some((data.name, NT::new(data)))
                           }
                       }
                   })
                   .collect();

        Ok(TypeInferencer { stack: vec![],
                            nonterminals: nonterminals,
                            types: Types::new(token_type) })
    }

    fn infer_types(mut self) -> NormResult<Types> {
        let ids: Vec<InternedString> =
            self.nonterminals.iter()
                             .map(|(&id, _)| id)
                             .collect();

        for id in ids {
            try!(self.nonterminal_type(id));
            debug_assert!(self.types.lookup_nonterminal_type(id).is_some());
        }

        Ok(self.types)
    }

    fn nonterminal_type(&mut self, id: InternedString) -> NormResult<TypeRepr> {
        if let Some(repr) = self.types.lookup_nonterminal_type(id) {
            return Ok(repr.clone());
        }

        let nt = self.nonterminals[&id];
        if self.stack.contains(&id) {
            return_err!(nt.span, "cannot infer type of `{}` because it references itself", id);
        }

        let ty = try!(self.push(id, |this| {
            if let &Some(ref type_decl) = nt.type_decl {
                return this.type_ref(type_decl);
            }

            let mut alternative_types: Vec<_> = try! {
                nt.alternatives.iter()
                               .map(|alt| this.alternative_type(alt))
                               .collect()
            };

            // if there are no alternatives, then call it an error
            if alternative_types.is_empty() {
                return_err!(nt.span,
                            "nonterminal `{}` has no alternatives and hence parse cannot succeed",
                            id);
            }

            for ((ty, alt), i) in
                alternative_types[1..].iter().zip(&nt.alternatives[1..]).zip(1..)
            {
                if &alternative_types[0] != ty {
                    return_err!(alt.expr.span,
                                "type of alternative #{} is `{}`, \
                                 but type of first alternative is `{}`",
                                i+1, ty, alternative_types[0]);
                }
            }

            Ok(alternative_types.pop().unwrap())
        }));

        self.types.add_type(id, ty.clone());
        Ok(ty)
    }

    fn push<F,R>(&mut self, id: InternedString, f: F) -> NormResult<R>
        where F: FnOnce(&mut TypeInferencer) -> NormResult<R>
    {
        self.stack.push(id);
        let r = f(self);
        assert_eq!(self.stack.pop().unwrap(), id);
        r
    }

    fn type_ref(&mut self, type_ref: &TypeRef) -> NormResult<TypeRepr> {
        match *type_ref {
            TypeRef::Tuple(ref types) => {
                let types = try! {
                    types.iter().map(|t| self.type_ref(t)).collect()
                };
                Ok(TypeRepr::Tuple(types))
            }
            TypeRef::Nominal { ref path, ref types } => {
                let types = try! {
                    types.iter().map(|t| self.type_ref(t)).collect()
                };
                Ok(TypeRepr::Nominal { path: path.clone(), types: types })
            }
            TypeRef::Lifetime(id) => {
                Ok(TypeRepr::Lifetime(id))
            }
            TypeRef::Id(id) => {
                Ok(TypeRepr::Nominal { path: vec![id], types: vec![] })
            }
            TypeRef::OfSymbol(ref symbol) => {
                self.symbol_type(symbol)
            }
        }
    }

    fn alternative_type(&mut self, alt: &Alternative) -> NormResult<TypeRepr> {
        match norm_util::analyze_action(alt) {
            AlternativeAction::User(_) => {
                return_err!(alt.span, "cannot infer types if there is custom action code");
            }

            AlternativeAction::Default(Symbols::Named(ref syms)) => {
                return_err!(alt.span,
                            "cannot infer types in the presence of named symbols like `~{}:{}`",
                            syms[0].1, syms[0].2);
            }

            AlternativeAction::Default(Symbols::Anon(syms)) => {
                let symbol_types: Vec<TypeRepr> = try! {
                    syms.iter()
                        .map(|&(_, sym)| self.symbol_type(sym))
                        .collect()
                };
                Ok(maybe_tuple(symbol_types))
            }
        }
    }

    fn symbol_type(&mut self, symbol: &Symbol) -> NormResult<TypeRepr> {
        match *symbol {
            Symbol::Terminal(_) => Ok(self.types.terminal_type().clone()),
            Symbol::Nonterminal(id) => self.nonterminal_type(id),
            Symbol::Choose(ref s) => self.symbol_type(s),
            Symbol::Name(_, ref s) => self.symbol_type(s),

            Symbol::Repeat(..) | Symbol::Expr(..) | Symbol::Macro(..) => {
                unreachable!("symbol `{:?}` should have been expanded away", symbol)
            }
        }
    }
}

impl<'grammar> NT<'grammar> {
    fn new(data: &'grammar NonterminalData) -> NT<'grammar> {
        NT { span: data.span, type_decl: &data.type_decl, alternatives: &data.alternatives }
    }
}

fn maybe_tuple(v: Vec<TypeRepr>) -> TypeRepr {
    if v.len() == 1 {
        v.into_iter().next().unwrap()
    } else {
        TypeRepr::Tuple(v)
    }
}
