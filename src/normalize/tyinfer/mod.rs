use std::collections::{HashMap};
use intern::{intern, InternedString};
use grammar::parse_tree::{Alternative, Grammar, GrammarItem,
                          NonterminalData, RepeatSymbol, RepeatOp,
                          Span, Symbol, TypeRef};
use normalize::{NormResult, NormError};

#[cfg(test)]
mod test;

pub fn infer_types(mut grammar: Grammar) -> NormResult<Grammar> {
    {
        let mut inferencer = try!(TypeInferencer::new(&mut grammar));
        try!(inferencer.infer_types());
    }
    Ok(grammar)
}

struct TypeInferencer<'a> {
    token_type: TypeRef,
    stack: Vec<InternedString>,
    nonterminals: HashMap<InternedString, NT<'a>>,
}

struct NT<'a> {
    span: Span,
    type_decl: &'a mut Option<TypeRef>,
    alternatives: &'a Vec<Alternative>,
}

fn extract_token_type(grammar: &Grammar) -> NormResult<TypeRef> {
    let mut token_types =
        grammar.items
               .iter()
               .filter_map(|item| {
                   match *item {
                       GrammarItem::TokenType(ref data) => Some(data.type_name.clone()),
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

    Ok(token_type)
}

impl<'a> TypeInferencer<'a> {
    fn new(grammar: &'a mut Grammar) -> NormResult<TypeInferencer<'a>> {
        let token_type =
            try!(extract_token_type(grammar));

        let nonterminals =
            grammar.items
                   .iter_mut()
                   .filter_map(|item| {
                       match *item {
                           GrammarItem::TokenType(..) =>
                               None,
                           GrammarItem::Nonterminal(ref mut data) => {
                               assert!(!data.is_macro_def()); // normalized away by now
                               Some((data.name, NT::new(data)))
                           }
                       }
                   })
                   .collect();

        Ok(TypeInferencer { token_type: token_type,
                            stack: vec![],
                            nonterminals: nonterminals })
    }

    fn infer_types(&mut self) -> NormResult<()> {
        let ids: Vec<InternedString> =
            self.nonterminals.iter()
                             .map(|(&id, _)| id)
                             .collect();

        for id in ids {
            let ty = try!(self.nonterminal_type(id));
            *self.nonterminals.get_mut(&id).unwrap().type_decl = Some(ty);
        }

        Ok(())
    }

    fn nonterminal_type(&mut self, id: InternedString) -> NormResult<TypeRef> {
        let (span, type_decl, alternatives) = {
            let nt = &self.nonterminals[&id];
            (nt.span, nt.type_decl.clone(), nt.alternatives)
        };

        if self.stack.contains(&id) {
            return_err!(span, "cannot infer type of `{}` because it references itself", id);
        }

        self.push(id, |this| {
            let type_decl = type_decl; // FIXME rustc bug requires thisx
            if let Some(mut type_decl) = type_decl {
                try!(this.refresh_type(&mut type_decl));
                return Ok(type_decl);
            }

            let mut alternative_types: Vec<TypeRef> =
                try!(alternatives.iter()
                     .map(|alt| this.alternative_type(alt))
                     .collect());

            // if there are no alternatives, then call it an error
            if alternative_types.is_empty() {
                return_err!(span,
                            "nonterminal `{}` has no alternatives and hence parse cannot succeed",
                            id);
            }

            for (ty, alt) in alternative_types[1..].iter().zip(&alternatives[1..]) {
                if &alternative_types[0] != ty {
                    return_err!(alt.expr.span,
                                "type of this alternative is `{}`, \
                                 but type of first alternative is `{}`",
                                ty, alternative_types[0]);
                }
            }

            Ok(alternative_types.pop().unwrap())
        })
    }

    fn push<F,R>(&mut self, id: InternedString, f: F) -> R
        where F: FnOnce(&mut TypeInferencer) -> R
    {
        self.stack.push(id);
        let r = f(self);
        assert_eq!(self.stack.pop().unwrap(), id);
        r
    }

    fn refresh_type(&mut self, type_ref: &mut TypeRef) -> NormResult<()> {
        let replacement = match *type_ref {
            TypeRef::Tuple(ref mut types) |
            TypeRef::Nominal { path: _, ref mut types } => {
                for t in types {
                    try!(self.refresh_type(t));
                }
                return Ok(());
            }
            TypeRef::Lifetime(_) |
            TypeRef::Id(_) => {
                return Ok(());
            }
            TypeRef::OfSymbol(ref symbol) => {
                try!(self.symbol_type(symbol))
            }
        };

        *type_ref = replacement;
        Ok(())
    }

    fn alternative_type(&mut self, alt: &Alternative) -> NormResult<TypeRef> {
        // We can't infer types for alternatives with actions
        if alt.action.is_some() {
            return_err!(alt.span, "cannot infer types if there is custom action code");
        }

        // We can't infer types if there are named symbols, because
        // that should be a struct and we don't know which one.
        let named_symbols =
            alt.expr.symbols
                    .iter()
                    .filter(|sym| match **sym {
                        Symbol::Name(..) => true,
                        _ => false,
                    });
        for sym in named_symbols {
            return_err!(alt.span,
                        "cannot infer types in the presence of named symbols like `{}`",
                        sym);
        }

        // Otherwise, make a tuple of the items they chose with a `~`.
        let chosen_symbol_types: Vec<TypeRef> = try! {
            alt.expr.symbols
                    .iter()
                    .filter_map(|sym| match *sym {
                        Symbol::Choose(..) => Some(self.symbol_type(sym)),
                        _ => None,
                    })
                    .collect()
        };
        if !chosen_symbol_types.is_empty() {
            return Ok(maybe_tuple(chosen_symbol_types));
        }

        // If they didn't choose anything with a `~`, make a tuple of everything.
        let symbol_types: Vec<TypeRef> = try! {
            alt.expr.symbols.iter()
                            .map(|sym| self.symbol_type(sym))
                            .collect()
        };
        Ok(maybe_tuple(symbol_types))
    }

    fn symbol_type(&mut self, symbol: &Symbol) -> NormResult<TypeRef> {
        match *symbol {
            Symbol::Terminal(_) => Ok(self.token_type.clone()),
            Symbol::Nonterminal(id) => self.nonterminal_type(id),
            Symbol::Choose(ref s) => self.symbol_type(s),
            Symbol::Name(_, ref s) => self.symbol_type(s),
            Symbol::Repeat(ref rep) => self.repeat_type(rep),

            Symbol::Expr(..) | Symbol::Macro(..) => {
                unreachable!("symbol {} should have been expanded away", symbol)
            }
        }
    }

    fn repeat_type(&mut self, repeat: &RepeatSymbol) -> NormResult<TypeRef> {
        let symbol_type = try!(self.symbol_type(&repeat.symbol));
        let path = match repeat.op {
            RepeatOp::Plus |
            RepeatOp::Star =>
                vec![intern("std"), intern("vec"), intern("Vec")],
            RepeatOp::Question =>
                vec![intern("std"), intern("option"), intern("Option")],
        };
        Ok(TypeRef::Nominal { path: path, types: vec![symbol_type] })
    }
}

impl<'a> NT<'a> {
    fn new(data: &'a mut NonterminalData) -> NT<'a> {
        NT { span: data.span, type_decl: &mut data.type_decl, alternatives: &data.alternatives }
    }
}

fn maybe_tuple(v: Vec<TypeRef>) -> TypeRef {
    if v.len() == 1 {
        v.into_iter().next().unwrap()
    } else {
        TypeRef::Tuple(v)
    }
}
