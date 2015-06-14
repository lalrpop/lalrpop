/*!
 * Compiled representation of a grammar. Simplified, normalized
 * version of parse-tree.
 */

use intern::InternedString;

pub struct Grammar {
    pub nonterminals: Vec<Nonterminal>,
    pub action_fns: Vec<ActionFn>,
}

pub struct Nonterminal {
    name: InternedString,
    alternatives: Vec<Alternative>,
    action_fn: usize
}

pub struct Alternative {
    symbols: Vec<Symbol>
}

pub enum Symbol {
    Terminal(InternedString),
    Nonterminal(InternedString),
}

pub struct ActionFn {
    code: String
}

