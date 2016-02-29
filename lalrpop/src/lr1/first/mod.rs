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

    /// Returns `FIRST(...symbols)`. If `...symbols` may match
    /// epsilon, then this returned set will include EOF. (This is
    /// kind of repurposing EOF to serve as a binary flag of sorts.)
    pub fn first0(&self, grammar: &Grammar, symbols: &[Symbol]) -> TokenSet {
        let mut result = TokenSet::new(grammar);

        for symbol in symbols {
            match *symbol {
                Symbol::Terminal(t) => {
                    result.insert(grammar, Token::Terminal(t));
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
                        return result;
                    }
                }
            }
        }

        result.insert(grammar, Token::EOF);
        result
    }

    pub fn first1(&self, grammar: &Grammar, symbols: &[Symbol], lookahead: Token)
                  -> TokenSet
    {
        let mut set = self.first0(grammar, symbols);

        // we use EOF as the signal that `symbols` matches epsilon:
        let epsilon = set.take_eof(grammar);

        if epsilon {
            set.insert(grammar, lookahead);
        }

        set
    }
}

