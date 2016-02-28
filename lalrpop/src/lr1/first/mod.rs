//! First set construction and computation.

use collections::{Map, map};
use grammar::repr::*;
use lr1::lookahead::{Token, TokenSet};

#[cfg(test)]
mod test;

pub struct FirstSets {
    map: Map<NonterminalString, TokenSet>
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
                    this.first(grammar, &production.symbols, Token::EOF);
                let first_set =
                    this.map.entry(nt).or_insert_with(|| TokenSet::new(grammar));
                changed |= first_set.insert_set(&lookahead);
            }
        }
        this
    }

    pub fn first(&self, grammar: &Grammar, symbols: &[Symbol], lookahead: Token)
                 -> (TokenSet, bool) {
        let mut result = TokenSet::new(grammar);

        for symbol in symbols {
            match *symbol {
                Symbol::Terminal(t) => {
                    result.insert(grammar, Token::Terminal(t));
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
                                    Token::EOF => {
                                        empty_prod = true;
                                    }
                                    Token::Terminal(_) => {
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

