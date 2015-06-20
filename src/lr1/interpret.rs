//! LR(1) interpeter. Just builds up parse trees. Intended for testing.

use lr1::{Action, State, Lookahead};
use grammar::repr::*;
use std::fmt::{Debug, Display, Formatter, Error};
use util::Sep;

#[derive(PartialEq, Eq)]
pub enum ParseTree {
    Nonterminal(NonterminalString, Vec<ParseTree>),
    Terminal(TerminalString),
}

pub fn interpret<TOKENS>(states: &[State], tokens: TOKENS) -> Result<ParseTree, ()>
    where TOKENS: Iterator<Item=TerminalString>
{
    let mut m = Machine::new(states);
    m.execute(tokens)
}

struct Machine<'states, 'grammar:'states> {
    states: &'states [State<'grammar>],
    state_stack: Vec<&'states State<'grammar>>,
    data_stack: Vec<ParseTree>,
}

impl<'states, 'grammar> Machine<'states, 'grammar> {
    fn new(states: &'states [State<'grammar>]) -> Machine<'states,'grammar> {
        Machine { states: states,
                  state_stack: vec![],
                  data_stack: vec![] }
    }

    fn execute<TOKENS>(&mut self, mut tokens: TOKENS) -> Result<ParseTree, ()>
        where TOKENS: Iterator<Item=TerminalString>
    {
        assert!(self.state_stack.is_empty());
        assert!(self.data_stack.is_empty());

        self.state_stack.push(&self.states[0]);

        let mut token = tokens.next();
        while let Some(terminal) = token {
            let state = *self.state_stack.last().unwrap();

            // check whether we can shift this token
            match state.tokens.get(&Lookahead::Terminal(terminal)) {
                None => { return Err(()); }

                Some(&Action::Shift(next_index)) => {
                    self.data_stack.push(ParseTree::Terminal(terminal));
                    self.state_stack.push(&self.states[next_index.0]);
                    token = tokens.next();
                }

                Some(&Action::Reduce(production)) => {
                    let more = self.reduce(production);
                    assert!(more);
                }
            }
        }

        // drain now for EOF
        loop {
            let state = *self.state_stack.last().unwrap();
            match state.tokens.get(&Lookahead::EOF) {
                None => { return Err(()); }
                Some(&Action::Shift(_)) => { unreachable!("cannot shift EOF") }
                Some(&Action::Reduce(production)) => {
                    if !self.reduce(production) {
                        assert_eq!(self.data_stack.len(), 1);
                        return Ok(self.data_stack.pop().unwrap());
                    }
                }
            }
        }
    }

    fn reduce(&mut self, production: &Production) -> bool {
        let args = production.symbols.len();

        // remove the top N items from the data stack
        let mut popped = vec![];
        for _ in 0 .. args {
            popped.push(self.data_stack.pop().unwrap());
        }
        popped.reverse();

        // remove the top N states
        for _ in 0 .. args {
            self.state_stack.pop().unwrap();
        }

        // construct the new, reduced tree and push it on the stack
        let tree = ParseTree::Nonterminal(production.nonterminal, popped);
        self.data_stack.push(tree);

        // recover the state and extract the "Goto" action
        let receiving_state = *self.state_stack.last().unwrap();
        match receiving_state.gotos.get(&production.nonterminal) {
            Some(goto_state) => {
                self.state_stack.push(&self.states[goto_state.0]);
                true // keep going
            }
            None => {
                false // all done
            }
        }
    }
}

impl Debug for ParseTree {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        Display::fmt(self, fmt)
    }
}

impl Display for ParseTree {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            ParseTree::Nonterminal(id, ref trees) => write!(fmt, "[{}: {}]", id, Sep(", ", trees)),
            ParseTree::Terminal(id) => write!(fmt, "{}", id),
        }
    }
}
