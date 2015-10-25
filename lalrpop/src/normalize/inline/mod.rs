/*!
 * Inlining of nonterminals
 */

use grammar::repr::*;
use normalize::NormResult;

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
    for (_, data) in &mut grammar.nonterminals {
        let mut new_productions = vec![];
        let mut new_action_fn_defns = vec![];

        for into_production in &data.productions {
            if !into_production.symbols.contains(&Symbol::Nonterminal(inline_nt)) {
                new_productions.push(into_production.clone());
                continue;
            }

            for inline_production in &inline_productions {
                let mut inliner = Inliner {
                    action_fn_defns: &grammar.action_fn_defns,
                    inline_production: inline_production,
                    into_production: into_production,
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
    /// Action fn defns
    action_fn_defns: &'a [ActionFnDefn],

    /// The production `A = B C D` being inlined
    inline_production: &'a Production,

    /// The `X = Y A Z` being inlined into
    into_production: &'a Production,

    /// The list of symbols we building up for the new production.
    /// For example, this would (eventually) contain `Y B C D Z`,
    /// given our running example.
    new_symbols: Vec<InlinedSymbol>,

    /// The output vector of all productions for `X` that we have created
    new_productions: &'a mut Vec<Production>,

    /// Vector of all action fn defns from the grammar.
    new_action_fn_defns: &'a mut Vec<ActionFnDefn>,
}

impl<'a> Inliner<'a> {
    fn inline(&mut self, into_symbols: &[Symbol]) {
        let new_symbols_len = self.new_symbols.len();

        if into_symbols.is_empty() {
            // create an action fn for the result of inlining
            let into_action = self.into_production.action;
            let inline_action = self.inline_production.action;
            let into_fallible = self.action_fn_defns[into_action.index()].fallible;
            let into_ret_type = self.action_fn_defns[into_action.index()].ret_type.clone();
            let inline_fallible = self.action_fn_defns[inline_action.index()].fallible;
            let index = self.action_fn_defns.len() + self.new_action_fn_defns.len();
            let action_fn = ActionFn::new(index);
            let inline_defn = InlineActionFnDefn {
                action: into_action,
                symbols: self.new_symbols.clone()
            };
            self.new_action_fn_defns.push(ActionFnDefn {
                fallible: into_fallible || inline_fallible,
                ret_type: into_ret_type,
                kind: ActionFnDefnKind::Inline(inline_defn),
            });
            let prod_symbols: Vec<Symbol> =
                self.new_symbols.iter()
                                .flat_map(|sym| match *sym {
                                    InlinedSymbol::Original(s) => vec![s],
                                    InlinedSymbol::Inlined(_, ref s) => s.clone(),
                                })
                                .collect();
            self.new_productions.push(Production {
                nonterminal: self.into_production.nonterminal,
                span: self.into_production.span,
                symbols: prod_symbols,
                action: action_fn,
            });
        } else {
            let next_symbol = into_symbols[0];
            match next_symbol {
                Symbol::Nonterminal(n) if n == self.inline_production.nonterminal => {
                    self.new_symbols.push(
                        InlinedSymbol::Inlined(
                            self.inline_production.action,
                            self.inline_production.symbols.clone()));
                }
                _ => {
                    self.new_symbols.push(InlinedSymbol::Original(next_symbol));
                }
            }
            self.inline(&into_symbols[1..]);
        }

        self.new_symbols.truncate(new_symbols_len);
    }
}
