/*!
 * Inlining of nonterminals
 */

use grammar::repr::*;
use normalize::NormResult;

mod graph;

pub fn inline(mut grammar: Grammar) -> NormResult<Grammar> {
    let order = try!(graph::inline_order(&grammar));
    for nt in order {
        grammar = inline_nt(grammar, nt);
    }
    Ok(grammar)
}

fn inline_nt(grammar: Grammar, _nt: NonterminalString) -> Grammar {
    grammar
}
