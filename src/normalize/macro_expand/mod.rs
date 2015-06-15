use std::collections::{HashMap, HashSet};
use intern::{intern, read, InternedString};
use grammar::parse_tree::{Alternative, Condition, ConditionOp, Grammar, GrammarItem,
                          MacroSymbol, NonterminalData, Span, Symbol, TypeRef};
use normalize::{NormResult, NormError};
use regex::Regex;

#[cfg(test)]
mod test;

pub fn expand_macros(input: Grammar) -> NormResult<Grammar> {
    let Grammar { type_name, items } = input;

    let (macro_defs, mut items): (Vec<_>, Vec<_>) =
        items.into_iter().partition(|mi| mi.is_macro_def());

    let macro_defs: HashMap<InternedString, NonterminalData> =
        macro_defs.into_iter()
                  .map(|md| match md {
                      GrammarItem::Nonterminal(data) => (data.name, data),
                      _ => unreachable!()
                  })
                  .collect();

    let mut expander = MacroExpander::new(macro_defs);
    try!(expander.expand(&mut items));

    Ok(Grammar { type_name: type_name, items: items })
}

struct MacroExpander {
    macro_defs: HashMap<InternedString, NonterminalData>,
    expansion_stack: Vec<MacroSymbol>,
    expansion_set: HashSet<InternedString>,
}

impl MacroExpander {
    fn new(macro_defs: HashMap<InternedString, NonterminalData>) -> MacroExpander {
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

            // Drain macro queue:
            while let Some(msym) = self.expansion_stack.pop() {
                items.push(try!(self.expand_macro_symbol(msym)));
            }
        }
    }

    fn replace_item(&mut self, item: &mut GrammarItem) {
        match *item {
            GrammarItem::TokenType(..) => { }
            GrammarItem::Nonterminal(ref mut data) => {
                // Should not encounter macro definitions here,
                // they've already been siphoned off.
                assert!(!data.is_macro_def());

                for alternative in &mut data.alternatives {
                    self.replace_symbols(&mut alternative.expr);
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
        let key;

        match *symbol {
            Symbol::Macro(ref mut m) => {
                for sym in &mut m.args {
                    self.replace_symbol(sym);
                }

                key = intern(&m.canonical_form());
                if self.expansion_set.insert(key) {
                    self.expansion_stack.push(m.clone());
                }
            }
            Symbol::Terminal(_) |
            Symbol::Nonterminal(_) => {
                return;
            }
            Symbol::Plus(ref mut sym) |
            Symbol::Question(ref mut sym) |
            Symbol::Star(ref mut sym) |
            Symbol::Choose(ref mut sym) |
            Symbol::Name(_, ref mut sym) => {
                self.replace_symbol(sym);
                return;
            }
            Symbol::Expr(ref mut syms) => {
                self.replace_symbols(syms);
                return;
            }
        }

        // we only get here if this is a macro expansion
        *symbol = Symbol::Nonterminal(key);
    }

    fn expand_macro_symbol(&mut self, msym: MacroSymbol) -> NormResult<GrammarItem> {
        let msym_name = intern(&msym.canonical_form());

        let mdef = match self.macro_defs.get(&msym.name) {
            Some(v) => v,
            None => return_err!(msym.span, "no macro definition found for `{}`", msym.name)
        };

        if mdef.args.len() != msym.args.len() {
            return_err!(msym.span, "expected {} arguments to `{}` but found {}",
                        mdef.args.len(), msym.name, msym.args.len());
        }

        let args: HashMap<InternedString, Symbol> =
            mdef.args.iter()
                     .cloned()
                     .zip(msym.args.into_iter())
                     .collect();

        let type_decl = mdef.type_decl.as_ref().map(|tr| self.expand_type_ref(&args, tr));

        // due to the use of `try!`, it's a bit awkward to write this with an iterator
        let mut alternatives: Vec<Alternative> = vec![];

        for alternative in &mdef.alternatives {
            if !try!(self.evaluate_cond(&args, &alternative.condition)) {
                continue;
            }
            alternatives.push(Alternative {
                expr: self.expand_symbols(&args, &alternative.expr),
                condition: None,
                action: alternative.action.clone(),
            });
        }

        Ok(GrammarItem::Nonterminal(NonterminalData {
            name: msym_name,
            args: vec![],
            type_decl: type_decl,
            alternatives: alternatives
        }))
    }

    fn expand_type_refs(&self,
                        args: &HashMap<InternedString, Symbol>,
                        type_refs: &[TypeRef])
                        -> Vec<TypeRef>
    {
        type_refs.iter().map(|tr| self.expand_type_ref(args, tr)).collect()
    }

    fn expand_type_ref(&self,
                       args: &HashMap<InternedString, Symbol>,
                       type_ref: &TypeRef)
                       -> TypeRef
    {
        match *type_ref {
            TypeRef::Tuple(ref trs) =>
                TypeRef::Tuple(self.expand_type_refs(args, trs)),
            TypeRef::Nominal { ref path, ref types } =>
                TypeRef::Nominal { path: path.clone(),
                                   types: self.expand_type_refs(args, types) },
            TypeRef::Lifetime(id) =>
                TypeRef::Lifetime(id),
            TypeRef::OfSymbol(ref sym) =>
                TypeRef::OfSymbol(sym.clone()),
            TypeRef::Id(id) => {
                match args.get(&id) {
                    Some(sym) => TypeRef::OfSymbol(sym.clone()),
                    None => TypeRef::Nominal { path: vec![id], types: vec![] },
                }
            }
        }
    }

    fn evaluate_cond(&self,
                     args: &HashMap<InternedString, Symbol>,
                     opt_cond: &Option<Condition>)
                     -> NormResult<bool>
    {
        if let Some(ref c) = *opt_cond {
            match args[&c.lhs] {
                Symbol::Terminal(lhs) => {
                    match c.op {
                        ConditionOp::Equals => Ok(lhs == c.rhs),
                        ConditionOp::NotEquals => Ok(lhs != c.rhs),
                        ConditionOp::Match => self.re_match(c.span, lhs, c.rhs),
                        ConditionOp::NotMatch => Ok(!try!(self.re_match(c.span, lhs, c.rhs))),
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

    fn expand_symbols(&self,
                      args: &HashMap<InternedString, Symbol>,
                      expr: &[Symbol])
                      -> Vec<Symbol>
    {
        expr.iter().map(|s| self.expand_symbol(args, s)).collect()
    }

    fn expand_box_symbol(&self,
                         args: &HashMap<InternedString, Symbol>,
                         symbol: &Symbol)
                         -> Box<Symbol>
    {
        Box::new(self.expand_symbol(args, symbol))
    }

    fn expand_symbol(&self,
                     args: &HashMap<InternedString, Symbol>,
                     symbol: &Symbol)
                     -> Symbol
    {
        match *symbol {
            Symbol::Expr(ref expr) => Symbol::Expr(self.expand_symbols(args, expr)),
            Symbol::Terminal(id) => Symbol::Terminal(id),
            Symbol::Nonterminal(id) => {
                match args.get(&id) {
                    Some(sym) => sym.clone(),
                    None => Symbol::Nonterminal(id),
                }
            },
            Symbol::Macro(ref msym) => {
                Symbol::Macro(MacroSymbol {
                    name: msym.name,
                    args: self.expand_symbols(args, &msym.args),
                    span: msym.span,
                })
            },
            Symbol::Plus(ref sym) =>
                Symbol::Plus(self.expand_box_symbol(args, sym)),
            Symbol::Star(ref sym) =>
                Symbol::Star(self.expand_box_symbol(args, sym)),
            Symbol::Question(ref sym) =>
                Symbol::Question(self.expand_box_symbol(args, sym)),
            Symbol::Choose(ref sym) =>
                Symbol::Choose(self.expand_box_symbol(args, sym)),
            Symbol::Name(id, ref sym) =>
                Symbol::Name(id, self.expand_box_symbol(args, sym)),
        }
    }
}
