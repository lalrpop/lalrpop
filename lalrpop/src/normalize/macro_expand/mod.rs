use std::collections::{HashMap, HashSet};
use intern::{intern, read, InternedString};
use grammar::parse_tree::{Alternative, Condition, ConditionOp, ExprSymbol, Grammar, GrammarItem,
                          MacroSymbol, NonterminalData, NonterminalString, RepeatOp, RepeatSymbol,
                          Span, Symbol, TypeRef};
use normalize::{NormResult, NormError};
use normalize::norm_util::{self, Symbols};
use regex::Regex;
use std::mem;

#[cfg(test)]
mod test;

pub fn expand_macros(input: Grammar) -> NormResult<Grammar> {
    let items = input.items;

    let (macro_defs, mut items): (Vec<_>, Vec<_>) =
        items.into_iter().partition(|mi| mi.is_macro_def());

    let macro_defs: HashMap<_, _> =
        macro_defs.into_iter()
                  .map(|md| match md {
                      GrammarItem::Nonterminal(data) => (data.name, data),
                      _ => unreachable!()
                  })
                  .collect();

    let mut expander = MacroExpander::new(macro_defs);
    try!(expander.expand(&mut items));

    Ok(Grammar { items: items, ..input})
}

struct MacroExpander {
    macro_defs: HashMap<NonterminalString, NonterminalData>,
    expansion_set: HashSet<NonterminalString>,
    expansion_stack: Vec<Symbol>,
}

impl MacroExpander {
    fn new(macro_defs: HashMap<NonterminalString, NonterminalData>) -> MacroExpander {
        MacroExpander {
            macro_defs: macro_defs,
            expansion_stack: Vec::new(),
            expansion_set: HashSet::new()
        }
    }

    fn expand(&mut self, items: &mut Vec<GrammarItem>) -> NormResult<()> {
        let mut counter = 0;
        loop {
            // Find any macro uses in items added since last round and
            // replace them in place with the expanded version:
            for item in &mut items[counter..] {
                self.replace_item(item);
            }
            counter = items.len();

            // No more expansion to do.
            if self.expansion_stack.is_empty() {
                return Ok(());
            }

            // Drain expansion stack:
            while let Some(sym) = self.expansion_stack.pop() {
                match sym {
                    Symbol::Macro(msym) =>
                        items.push(try!(self.expand_macro_symbol(msym))),
                    Symbol::Expr(expr) =>
                        items.push(try!(self.expand_expr_symbol(expr))),
                    Symbol::Repeat(repeat) =>
                        items.push(try!(self.expand_repeat_symbol(*repeat))),
                    _ =>
                        assert!(false, "don't know how to expand `{:?}`", sym)
                }
            }
        }
    }

    fn replace_item(&mut self, item: &mut GrammarItem) {
        match *item {
            GrammarItem::TokenType(..) => { }
            GrammarItem::Use(..) => { }
            GrammarItem::Nonterminal(ref mut data) => {
                // Should not encounter macro definitions here,
                // they've already been siphoned off.
                assert!(!data.is_macro_def());

                for alternative in &mut data.alternatives {
                    self.replace_symbols(&mut alternative.expr.symbols);
                }
            }
        }
    }

    fn replace_symbols(&mut self, symbols: &mut [Symbol]) {
        for symbol in symbols {
            self.replace_symbol(symbol);
        }
    }

    fn replace_symbol(&mut self, symbol: &mut Symbol) {
        match *symbol {
            Symbol::Macro(ref mut m) => {
                for sym in &mut m.args {
                    self.replace_symbol(sym);
                }
            }
            Symbol::Expr(ref mut expr) => {
                self.replace_symbols(&mut expr.symbols);
            }
            Symbol::Repeat(ref mut repeat) => {
                self.replace_symbol(&mut repeat.symbol);
            }
            Symbol::Terminal(_) |
            Symbol::Nonterminal(_) => {
                return;
            }
            Symbol::Choose(ref mut sym) |
            Symbol::Name(_, ref mut sym) => {
                self.replace_symbol(sym);
                return;
            }
        }

        // only symbols we intend to expand fallthrough to here

        let key = NonterminalString(intern(&symbol.canonical_form()));
        let to_expand = mem::replace(symbol, Symbol::Nonterminal(key));
        if self.expansion_set.insert(key) {
            self.expansion_stack.push(to_expand);
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    // Macro expansion

    fn expand_macro_symbol(&mut self, msym: MacroSymbol) -> NormResult<GrammarItem> {
        let msym_name = NonterminalString(intern(&msym.canonical_form()));

        let mdef = match self.macro_defs.get(&msym.name) {
            Some(v) => v,
            None => return_err!(msym.span, "no macro definition found for `{}`", msym.name)
        };

        if mdef.args.len() != msym.args.len() {
            return_err!(msym.span, "expected {} arguments to `{}` but found {}",
                        mdef.args.len(), msym.name, msym.args.len());
        }

        let args: HashMap<NonterminalString, Symbol> =
            mdef.args.iter()
                     .cloned()
                     .zip(msym.args.into_iter())
                     .collect();

        let type_decl = mdef.type_decl.as_ref().map(|tr| self.macro_expand_type_ref(&args, tr));

        // due to the use of `try!`, it's a bit awkward to write this with an iterator
        let mut alternatives: Vec<Alternative> = vec![];

        for alternative in &mdef.alternatives {
            if !try!(self.evaluate_cond(&args, &alternative.condition)) {
                continue;
            }
            alternatives.push(Alternative {
                span: msym.span,
                expr: self.macro_expand_expr_symbol(&args, &alternative.expr),
                condition: None,
                action: alternative.action.clone(),
            });
        }

        Ok(GrammarItem::Nonterminal(NonterminalData {
            public: mdef.public,
            span: msym.span,
            name: msym_name,
            args: vec![],
            type_decl: type_decl,
            alternatives: alternatives
        }))
    }

    fn macro_expand_type_refs(&self,
                              args: &HashMap<NonterminalString, Symbol>,
                              type_refs: &[TypeRef])
                              -> Vec<TypeRef>
    {
        type_refs.iter().map(|tr| self.macro_expand_type_ref(args, tr)).collect()
    }

    fn macro_expand_type_ref(&self,
                             args: &HashMap<NonterminalString, Symbol>,
                             type_ref: &TypeRef)
                             -> TypeRef
    {
        match *type_ref {
            TypeRef::Tuple(ref trs) =>
                TypeRef::Tuple(self.macro_expand_type_refs(args, trs)),
            TypeRef::Nominal { ref path, ref types } =>
                TypeRef::Nominal { path: path.clone(),
                                   types: self.macro_expand_type_refs(args, types) },
            TypeRef::Lifetime(id) =>
                TypeRef::Lifetime(id),
            TypeRef::OfSymbol(ref sym) =>
                TypeRef::OfSymbol(sym.clone()),
            TypeRef::Id(id) => {
                match args.get(&NonterminalString(id)) {
                    Some(sym) => TypeRef::OfSymbol(sym.clone()),
                    None => TypeRef::Nominal { path: vec![id], types: vec![] },
                }
            }
        }
    }

    fn evaluate_cond(&self,
                     args: &HashMap<NonterminalString, Symbol>,
                     opt_cond: &Option<Condition>)
                     -> NormResult<bool>
    {
        if let Some(ref c) = *opt_cond {
            match args[&c.lhs] {
                Symbol::Terminal(lhs) => {
                    match c.op {
                        ConditionOp::Equals => Ok(lhs.0 == c.rhs),
                        ConditionOp::NotEquals => Ok(lhs.0 != c.rhs),
                        ConditionOp::Match => self.re_match(c.span, lhs.0, c.rhs),
                        ConditionOp::NotMatch => Ok(!try!(self.re_match(c.span, lhs.0, c.rhs))),
                    }
                }
                ref lhs => {
                    return_err!(
                        c.span,
                        "invalid condition LHS `{}`, expected a terminal, not `{}`", c.lhs, lhs);
                }
            }
        } else {
            Ok(true)
        }
    }

    fn re_match(&self, span: Span, lhs: InternedString, regex: InternedString) -> NormResult<bool> {
        read(|interner| {
            let re = match Regex::new(interner.data(regex)) {
                Ok(re) => re,
                Err(err) => return_err!(span, "invalid regular expression `{}`: {}", regex, err),
            };
            Ok(re.is_match(interner.data(lhs)))
        })
    }

    fn macro_expand_symbols(&self,
                            args: &HashMap<NonterminalString, Symbol>,
                            expr: &[Symbol])
                            -> Vec<Symbol>
    {
        expr.iter().map(|s| self.macro_expand_symbol(args, s)).collect()
    }

    fn macro_expand_expr_symbol(&self,
                                args: &HashMap<NonterminalString, Symbol>,
                                expr: &ExprSymbol)
                                -> ExprSymbol
    {
        ExprSymbol { span: expr.span, // FIXME derived span
                     symbols: self.macro_expand_symbols(args, &expr.symbols) }
    }

    fn macro_expand_symbol(&self,
                           args: &HashMap<NonterminalString, Symbol>,
                           symbol: &Symbol)
                           -> Symbol
    {
        match *symbol {
            Symbol::Expr(ref expr) =>
                Symbol::Expr(self.macro_expand_expr_symbol(args, expr)),
            Symbol::Terminal(id) =>
                Symbol::Terminal(id),
            Symbol::Nonterminal(id) =>
                match args.get(&id) {
                    Some(sym) => sym.clone(),
                    None => Symbol::Nonterminal(id),
                },
            Symbol::Macro(ref msym) =>
                Symbol::Macro(MacroSymbol {
                    name: msym.name,
                    args: self.macro_expand_symbols(args, &msym.args),
                    span: msym.span,
                }),
            Symbol::Repeat(ref r) =>
                Symbol::Repeat(Box::new(RepeatSymbol {
                    span: r.span,
                    op: r.op,
                    symbol: self.macro_expand_symbol(args, &r.symbol)
                })),
            Symbol::Choose(ref sym) =>
                Symbol::Choose(Box::new(self.macro_expand_symbol(args, sym))),
            Symbol::Name(id, ref sym) =>
                Symbol::Name(id, Box::new(self.macro_expand_symbol(args, sym))),
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    // Expr expansion

    fn expand_expr_symbol(&mut self, expr: ExprSymbol) -> NormResult<GrammarItem> {
        let name = NonterminalString(intern(&expr.canonical_form()));

        let ty_ref = match norm_util::analyze_expr(&expr) {
            Symbols::Named(names) => {
                let (_, ex_id, ex_sym) = names[0];
                return_err!(
                    expr.span,
                    "named symbols like `~{}:{}` are only allowed at the top-level of a nonterminal",
                    ex_id, ex_sym)
            }
            Symbols::Anon(syms) => {
                maybe_tuple(
                    syms.into_iter()
                        .map(|(_, s)| TypeRef::OfSymbol(s.clone()))
                        .collect())
            }
        };

        Ok(GrammarItem::Nonterminal(NonterminalData {
            public: false,
            span: expr.span,
            name: name,
            args: vec![],
            type_decl: Some(ty_ref),
            alternatives: vec![Alternative { span: expr.span,
                                             expr: expr,
                                             condition: None,
                                             action: Some(format!("(~~)")) }]
        }))
    }

    ///////////////////////////////////////////////////////////////////////////
    // Expr expansion

    fn expand_repeat_symbol(&mut self, repeat: RepeatSymbol) -> NormResult<GrammarItem> {
        let name = NonterminalString(intern(&repeat.canonical_form()));
        let v = intern("v");
        let e = intern("e");

        let base_symbol_ty = TypeRef::OfSymbol(repeat.symbol.clone());

        match repeat.op {
            RepeatOp::Star => {
                let path = vec![intern("std"), intern("vec"), intern("Vec")];
                let ty_ref = TypeRef::Nominal { path: path, types: vec![base_symbol_ty] };

                Ok(GrammarItem::Nonterminal(NonterminalData {
                    public: false,
                    span: repeat.span,
                    name: name,
                    args: vec![],
                    type_decl: Some(ty_ref),
                    alternatives: vec![
                        // X* =
                        Alternative {
                            span: repeat.span,
                            expr: ExprSymbol {
                                span: repeat.span,
                                symbols: vec![],
                            },
                            condition: None,
                            action: Some(format!("vec![]"))
                        },

                        // X* = ~v:X+ ~e:X
                        Alternative {
                            span: repeat.span,
                            expr: ExprSymbol {
                                span: repeat.span,
                                symbols: vec![Symbol::Name(v, Box::new(Symbol::Nonterminal(name))),
                                              Symbol::Name(e, Box::new(repeat.symbol.clone()))]
                            },
                            condition: None,
                            action: Some(format!("{{ let mut v = v; v.push(e); v }}"))
                        }],
                }))
            }

            RepeatOp::Plus => {
                let path = vec![intern("std"), intern("vec"), intern("Vec")];
                let ty_ref = TypeRef::Nominal { path: path, types: vec![base_symbol_ty] };

                Ok(GrammarItem::Nonterminal(NonterminalData {
                    public: false,
                    span: repeat.span,
                    name: name,
                    args: vec![],
                    type_decl: Some(ty_ref),
                    alternatives: vec![
                        // X+ = X
                        Alternative {
                            span: repeat.span,
                            expr: ExprSymbol {
                                span: repeat.span,
                                symbols: vec![repeat.symbol.clone()]
                            },
                            condition: None,
                            action: Some(format!("vec![~~]"))
                        },

                        // X+ = ~v:X+ ~e:X
                        Alternative {
                            span: repeat.span,
                            expr: ExprSymbol {
                                span: repeat.span,
                                symbols: vec![Symbol::Name(v, Box::new(Symbol::Nonterminal(name))),
                                              Symbol::Name(e, Box::new(repeat.symbol.clone()))]
                            },
                            condition: None,
                            action: Some(format!("{{ let mut v = v; v.push(e); v }}"))
                        }],
                }))
            }

            RepeatOp::Question => {
                let path = vec![intern("std"), intern("option"), intern("Option")];
                let ty_ref = TypeRef::Nominal { path: path, types: vec![base_symbol_ty] };

                Ok(GrammarItem::Nonterminal(NonterminalData {
                    public: false,
                    span: repeat.span,
                    name: name,
                    args: vec![],
                    type_decl: Some(ty_ref),
                    alternatives: vec![
                        // X? = X => Some(~~)
                        Alternative { span: repeat.span,
                                      expr: ExprSymbol {
                                          span: repeat.span,
                                          symbols: vec![repeat.symbol.clone()]
                                      },
                                      condition: None,
                                      action: Some(format!("Some(~~)")) },

                        // X? = { => None; }
                        Alternative { span: repeat.span,
                                      expr: ExprSymbol {
                                          span: repeat.span,
                                          symbols: vec![]
                                      },
                                      condition: None,
                                      action: Some(format!("None")) }]
                }))
            }
        }
    }
}

fn maybe_tuple(v: Vec<TypeRef>) -> TypeRef {
    if v.len() == 1 {
        v.into_iter().next().unwrap()
    } else {
        TypeRef::Tuple(v)
    }
}
