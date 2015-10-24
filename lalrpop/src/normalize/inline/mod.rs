/*!
 * Inlining of nonterminals
 */

use intern::intern;
use grammar::repr::*;
use normalize::NormResult;
use util::Sep;

mod graph;

#[cfg(test)]
mod test;

pub fn inline(mut grammar: Grammar) -> NormResult<Grammar> {
    let order = try!(graph::inline_order(&grammar));
    for nt in order {
        inline_nt(&mut grammar, nt);
    }
    Ok(grammar)
}

fn inline_nt(grammar: &mut Grammar, inline_nt: NonterminalString) {
    let inline_productions: Vec<_> = grammar.productions_for(inline_nt).iter().cloned().collect();
    for (&nt, data) in &mut grammar.nonterminals {
        let mut new_productions = vec![];
        let mut new_action_fn_defns = vec![];

        for into_production in &data.productions {
            if !into_production.symbols.contains(&Symbol::Nonterminal(inline_nt)) {
                new_productions.push(into_production.clone());
                continue;
            }

            let into_action = into_production.action;
            let into_fallible = grammar.action_fn_defns[into_action.index()].fallible;

            for inline_production in &inline_productions {
                let inline_fallible = grammar.action_fn_defns[inline_production.action.index()].fallible;

                let mut inliner = Inliner {
                    prefix: &grammar.prefix,
                    types: &grammar.types,
                    action_fn_defns: &grammar.action_fn_defns,
                    inline_production: inline_production,
                    into_production: into_production,
                    new_defn: ActionFnDefn {
                        arg_patterns: vec![],
                        arg_types: vec![],
                        ret_type: grammar.types.nonterminal_type(nt).clone(),
                        fallible: into_fallible || inline_fallible,
                        code: format!("{}action{}(", grammar.prefix, into_action.index()),
                    },
                    new_symbols: vec![],
                    new_productions: &mut new_productions,
                    new_action_fn_defns: &mut new_action_fn_defns,
                };

                inliner.inline(&into_production.symbols);
            }
        }

        data.productions = new_productions;
        grammar.action_fn_defns.extend(new_action_fn_defns);
    }
}

struct Inliner<'a> {
    /// Prefix from grammar
    prefix: &'a str,

    /// Types for nonterminals etc.
    types: &'a Types,

    /// Action fn defns
    action_fn_defns: &'a [ActionFnDefn],

    /// The production `A = B C D` being inlined
    inline_production: &'a Production,

    /// The `X = Y A Z` being inlined into
    into_production: &'a Production,

    /// The new action fn we are constructing. If we were inlining
    /// `A = B C D` (with action 44) into `X = Y A Z` (with action 22),
    /// this would look something like:
    ///
    /// ```
    /// fn __action66(__0: Y, __1: B, __2: C, __3: D, __4: Z) {
    ///     __action22(__0, __action44(__1, __2, __3), __4)
    /// }
    /// ```
    new_defn: ActionFnDefn,

    /// The list of symbols we building up for the new production.
    /// For example, this would (eventually) contain `Y B C D Z`,
    /// given our running example.
    new_symbols: Vec<Symbol>,

    /// The output vector of all productions for `X` that we have created
    new_productions: &'a mut Vec<Production>,

    /// Vector of all action fn defns from the grammar.
    new_action_fn_defns: &'a mut Vec<ActionFnDefn>,
}

impl<'a> Inliner<'a> {
    fn inline(&mut self, into_symbols: &[Symbol]) {
        let types = self.types;
        let arg_patterns_len = self.new_defn.arg_patterns.len();
        let arg_types_len = self.new_defn.arg_types.len();
        let code_len = self.new_defn.code.len();
        let new_symbols_len = self.new_symbols.len();

        if into_symbols.is_empty() {
            // create an action fn for the result of inlining
            let index = self.action_fn_defns.len() + self.new_action_fn_defns.len();
            let action_fn = ActionFn::new(index);
            self.new_defn.code.push_str(")");
            self.new_action_fn_defns.push(self.new_defn.clone());
            self.new_productions.push(Production {
                nonterminal: self.into_production.nonterminal,
                span: self.into_production.span,
                symbols: self.new_symbols.clone(),
                action: action_fn,
            });
        } else {
            let next_symbol = into_symbols[0];
            match next_symbol {
                Symbol::Nonterminal(n) if n == self.inline_production.nonterminal => {
                    let arg_patterns: Vec<_> =
                        (arg_patterns_len..arg_patterns_len+self.inline_production.symbols.len())
                        .map(|i| intern(&format!("{}{}", self.prefix, i)))
                        .collect();
                    let inline_action =
                        self.inline_production.action;
                    let inline_fallible =
                        self.action_fn_defns[inline_action.index()].fallible;
                    let new_code = if inline_fallible {
                        format!("try!({}action{}({})), ",
                                self.prefix, inline_action.index(), Sep(",", &arg_patterns))
                    } else {
                        format!("{}action{}({}), ",
                                self.prefix, inline_action.index(), Sep(",", &arg_patterns))
                    };
                    self.new_symbols.extend(self.inline_production.symbols.iter().cloned());
                    self.new_defn.arg_patterns.extend(arg_patterns);
                    self.new_defn.arg_types.extend(
                        self.inline_production.symbols
                                              .iter()
                                              .map(|s| s.ty(types).clone()));
                    self.new_defn.code.push_str(
                        &new_code);
                }
                _ => {
                    let arg = format!("{}{}, ", self.prefix, arg_patterns_len);
                    self.new_symbols.push(next_symbol);
                    self.new_defn.arg_patterns.push(intern(&arg));
                    self.new_defn.arg_types.push(next_symbol.ty(types).clone());
                    self.new_defn.code.push_str(&arg);
                }
            }
            self.inline(&into_symbols[1..]);
        }

        self.new_defn.arg_patterns.truncate(arg_patterns_len);
        self.new_defn.arg_types.truncate(arg_types_len);
        self.new_defn.code.truncate(code_len);
        self.new_symbols.truncate(new_symbols_len);
    }
}
