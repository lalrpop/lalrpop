//! First set construction and computation.

use grammar::repr::*;
use lr1::lookahead::{Lookahead, LookaheadSet};
use util::{Map, map};

#[cfg(test)]
mod test;

pub struct FirstSets {
    map: Map<NonterminalString, LookaheadSet>
}

impl FirstSets {
    pub fn new(grammar: &Grammar) -> FirstSets {
        let mut this = FirstSets { map: map() };
        let mut changed = true;
        while changed {
            changed = false;
            for production in grammar.nonterminals.values()
                                                  .flat_map(|p| &p.productions) {
                let nt = production.nonterminal;
                let (lookahead, _) =
                    this.first(grammar, &production.symbols, Lookahead::EOF);
                let first_set =
                    this.map.entry(nt).or_insert_with(|| LookaheadSet::new(grammar));
                changed |= first_set.insert_set(&lookahead);
            }
        }
        this
    }

    pub fn first(&self, grammar: &Grammar, symbols: &[Symbol], lookahead: Lookahead)
                 -> (LookaheadSet, bool) {
        let mut result = LookaheadSet::new(grammar);

        for symbol in symbols {
            match *symbol {
                Symbol::Terminal(t) => {
                    result.insert(grammar, Lookahead::Terminal(t));
                    return (result, false);
                }

                Symbol::Nonterminal(nt) => {
                    let mut empty_prod = false;
                    match self.map.get(&nt) {
                        None => {
                            // This should only happen during set
                            // construction; it corresponds to an
                            // entry that has not yet been
                            // built. Otherwise, it would mean a
                            // terminal with no productions. Either
                            // way, the resulting first set should be
                            // empty.
                        }
                        Some(set) => {
                            for lookahead in set.iter(grammar) {
                                match lookahead {
                                    Lookahead::EOF => {
                                        empty_prod = true;
                                    }
                                    Lookahead::Terminal(_) => {
                                        result.insert(grammar, lookahead);
                                    }
                                }
                            }
                        }
                    }
                    if !empty_prod {
                        return (result, false);
                    }
                }
            }
        }

        result.insert(grammar, lookahead);
        (result, true)
    }
}

