//! First set construction and computation.

use grammar::repr::*;
use std::collections::{HashMap, HashSet};

use super::Lookahead;

#[cfg(test)]
mod test;

pub struct FirstSets {
    map: HashMap<NonterminalString, FirstSet>
}

pub type FirstSet = HashSet<Option<TerminalString>>;

impl FirstSets {
    pub fn new(grammar: &Grammar) -> FirstSets {
        let mut this = FirstSets { map: HashMap::new() };
        let mut changed = true;
        while changed {
            changed = false;
            for production in grammar.productions.values().flat_map(|p| p.iter()) {
                let nt = production.nonterminal;
                let lookahead = this.first(&production.symbols, Lookahead::EOF);
                let first_set = this.map.entry(nt).or_insert_with(|| HashSet::new());
                let cardinality = first_set.len();
                first_set.extend(
                    lookahead.into_iter()
                             .map(|la| match la {
                                 Lookahead::EOF => None,
                                 Lookahead::Terminal(t) => Some(t),
                             }));
                changed |= (cardinality != first_set.len());
            }
        }
        this
    }

    pub fn first(&self, symbols: &[Symbol], lookahead: Lookahead) -> Vec<Lookahead> {
        let mut result = vec![];

        for symbol in symbols {
            match *symbol {
                Symbol::Terminal(t) => {
                    result.push(Lookahead::Terminal(t));
                    return result;
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
                            for &opt_terminal in set {
                                if let Some(terminal) = opt_terminal {
                                    result.push(Lookahead::Terminal(terminal));
                                } else {
                                    empty_prod = true;
                                }
                            }
                        }
                    }
                    if !empty_prod {
                        return result;
                    }
                }
            }
        }

        result.push(lookahead);
        result
    }
}

