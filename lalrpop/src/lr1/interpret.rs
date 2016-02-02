//! LR(1) interpeter. Just builds up parse trees. Intended for testing.

use lr1::{Action, State, StateIndex, Lookahead};
use generate::ParseTree;
use grammar::repr::*;
use std::iter::IntoIterator;
use std::fmt::{Debug, Display, Formatter, Error};
use util::Sep;

pub type InterpretError<'grammar> = (&'grammar State<'grammar>, Lookahead);

/// Feed in the given tokens and then EOF, returning the final parse tree that is reduced.
pub fn interpret<'grammar,TOKENS>(states: &'grammar [State<'grammar>], tokens: TOKENS)
                         -> Result<ParseTree, InterpretError<'grammar>>
    where TOKENS: IntoIterator<Item=TerminalString>
{
    let mut m = Machine::new(states);
    m.execute(tokens.into_iter())
}

/// Feed in the given tokens and returns the states on the stack.
pub fn interpret_partial<'grammar,TOKENS>(states: &'grammar [State<'grammar>], tokens: TOKENS)
                                          -> Result<Vec<StateIndex>, InterpretError<'grammar>>
    where TOKENS: IntoIterator<Item=TerminalString>
{
    let mut m = Machine::new(states);
    try!(m.execute_partial(tokens.into_iter()));
    Ok(m.state_stack)
}

struct Machine<'grammar> {
    states: &'grammar [State<'grammar>],
    state_stack: Vec<StateIndex>,
    data_stack: Vec<ParseTree>,
}

impl<'grammar> Machine<'grammar> {
    fn new(states: &'grammar [State<'grammar>]) -> Machine<'grammar> {
        Machine { states: states,
                  state_stack: vec![],
                  data_stack: vec![] }
    }

    fn top_state(&self) -> &'grammar State<'grammar> {
        let index = self.state_stack.last().unwrap();
        &self.states[index.0]
    }

    fn execute_partial<TOKENS>(&mut self, mut tokens: TOKENS)
                               -> Result<(), InterpretError<'grammar>>
        where TOKENS: Iterator<Item=TerminalString>
    {
        assert!(self.state_stack.is_empty());
        assert!(self.data_stack.is_empty());

        self.state_stack.push(StateIndex(0));

        let mut token = tokens.next();
        while let Some(terminal) = token {
            let state = self.top_state();

            // check whether we can shift this token
            match state.tokens.get(&Lookahead::Terminal(terminal)) {
                None => { return Err((state, Lookahead::Terminal(terminal))); }

                Some(&Action::Shift(next_index)) => {
                    self.data_stack.push(ParseTree::Terminal(terminal));
                    self.state_stack.push(next_index);
                    token = tokens.next();
                }

                Some(&Action::Reduce(production)) => {
                    let more = self.reduce(production);
                    assert!(more);
                }
            }
        }

        Ok(())
    }

    fn execute<TOKENS>(&mut self, tokens: TOKENS)
                           -> Result<ParseTree, InterpretError<'grammar>>
        where TOKENS: Iterator<Item=TerminalString>
    {
        try!(self.execute_partial(tokens));

        // drain now for EOF
        loop {
            let state = self.top_state();
            match state.tokens.get(&Lookahead::EOF) {
                None => { return Err((state, Lookahead::EOF)); }
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
        let receiving_state = self.top_state();
        match receiving_state.gotos.get(&production.nonterminal) {
            Some(&goto_state) => {
                self.state_stack.push(goto_state);
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
