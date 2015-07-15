//! A compiler from an LR(1) table to a [recursive ascent] parser.
//!
//! [recursive ascent]: https://en.wikipedia.org/wiki/Recursive_ascent_parser

use intern::{InternedString};
use grammar::repr::{Grammar, NonterminalString, Symbol, TerminalString, Types};
use lr1::{Lookahead, State, StateIndex};
use rust::RustWrite;
use std::io::{self, Write};
use util::Sep;

pub type Path = Vec<InternedString>;

pub fn compile<'grammar,W:Write>(
    grammar: &'grammar Grammar,
    user_start_symbol: NonterminalString,
    start_symbol: NonterminalString,
    states: &[State<'grammar>],
    out: &mut RustWrite<W>)
    -> io::Result<()>
{
    let mut ascent = RecursiveAscent::new(grammar, user_start_symbol, start_symbol, states, out);
    ascent.write()
}

struct RecursiveAscent<'ascent,'grammar:'ascent,W:Write+'ascent> {
    grammar: &'grammar Grammar,
    prefix: &'grammar str,
    types: &'grammar Types,
    user_start_symbol: NonterminalString,
    start_symbol: NonterminalString,
    states: &'ascent [State<'grammar>],
    state_prefixes: Vec<&'grammar [Symbol]>,
    out: &'ascent mut RustWrite<W>,
}

impl<'ascent,'grammar,W:Write> RecursiveAscent<'ascent,'grammar,W> {
    fn new(grammar: &'grammar Grammar,
           user_start_symbol: NonterminalString,
           start_symbol: NonterminalString,
           states: &'ascent [State<'grammar>],
           out: &'ascent mut RustWrite<W>)
           -> RecursiveAscent<'ascent,'grammar,W>
    {
        RecursiveAscent {
            grammar: grammar,
            prefix: &grammar.prefix,
            types: &grammar.types,
            states: states,
            state_prefixes: states.iter().map(|s| s.prefix()).collect(),
            user_start_symbol: user_start_symbol,
            start_symbol: start_symbol,
            out: out,
        }
    }

    fn write(&mut self) -> io::Result<()> {
        try!(self.write_start_fn());

        rust!(self.out, "");
        rust!(self.out, "mod {}parse{} {{",
              self.prefix, self.start_symbol);

        rust!(self.out, "#![allow(non_snake_case, unused_mut, unused_variables)]");
        rust!(self.out, "");

        try!(self.write_uses());

        rust!(self.out, "");
        try!(self.write_return_type_defn());

        for i in 0..self.states.len() {
            rust!(self.out, "");
            try!(self.write_state_fn(StateIndex(i)));
        }

        rust!(self.out, "}}");

        Ok(())
    }

    fn write_uses(&mut self) -> io::Result<()> {
        for u in &self.grammar.uses {
            rust!(self.out, "use {};", u);
        }
        Ok(())
    }

    fn write_return_type_defn(&mut self) -> io::Result<()> {
        rust!(self.out, "pub enum {}Nonterminal {{", self.prefix);

        // make an enum with one variant per nonterminal; I considered
        // making different enums per state, but this would mean we
        // have to unwrap and rewrap as we pass up the stack, which
        // seems silly
        for &nt in self.grammar.productions.keys() {
            rust!(self.out, "{}({}),", nt, self.types.nonterminal_type(nt));
        }

        rust!(self.out, "}}");
        Ok(())
    }

    fn write_start_fn(&mut self) -> io::Result<()> {
        let terminal_type = self.types.terminal_enum_type();
        rust!(self.out, "#[allow(non_snake_case)]");
        try!(self.out.write_pub_fn_header(
            self.grammar,
            format!("parse_{}", self.user_start_symbol),
            vec![format!("{}TOKENS: IntoIterator<Item={}>", self.prefix, terminal_type)],
            vec![format!("{}tokens: {}TOKENS", self.prefix, self.prefix)],
            format!("Result<(Option<{}>, {}), Option<{}>>",
                    terminal_type, self.types.nonterminal_type(self.start_symbol), terminal_type),
            vec![]));
        rust!(self.out, "{{");
        rust!(self.out, "let mut {}tokens = {}tokens.into_iter();", self.prefix, self.prefix);
        rust!(self.out, "let {}lookahead = {}tokens.next();", self.prefix, self.prefix);
        rust!(self.out, "match try!({}parse{}::{}state0({}lookahead, &mut {}tokens)) {{",
              self.prefix, self.start_symbol, self.prefix, self.prefix, self.prefix);
        rust!(self.out, "({}lookahead, {}parse{}::{}Nonterminal::{}({}nt)) => \
                         Ok(({}lookahead, {}nt)),",
              self.prefix, self.prefix, self.start_symbol, self.prefix, self.start_symbol,
              self.prefix, self.prefix, self.prefix);
        rust!(self.out, "_ => unreachable!(),");
        rust!(self.out, "}}");
        rust!(self.out, "}}");

        Ok(())
    }

    fn write_state_fn(&mut self, this_index: StateIndex) -> io::Result<()> {
        let this_state = &self.states[this_index.0];
        let this_prefix = self.state_prefixes[this_index.0];
        let terminal_type = self.types.terminal_enum_type();

        // Leave a comment explaining what this state is.
        rust!(self.out, "// State {}", this_index.0);
        for item in this_state.items.vec.iter() {
            rust!(self.out, "//   {:?}", item);
        }
        rust!(self.out, "//");
        for (token, action) in &this_state.tokens {
            rust!(self.out, "//   {:?} -> {:?}", token, action);
        }
        rust!(self.out, "//");
        for (nt, state) in &this_state.gotos {
            rust!(self.out, "//   {:?} -> {:?}", nt, state);
        }

        // set to true if goto actions are worth generating
        let mut fallthrough = false;

        rust!(self.out, "pub fn {}state{}<", self.prefix, this_index.0);
        rust!(self.out, "    {}TOKENS: Iterator<Item={}>,", self.prefix, terminal_type);
        rust!(self.out, ">(");
        rust!(self.out, "mut {}lookahead: Option<{}>,", self.prefix, terminal_type);
        rust!(self.out, "{}tokens: &mut {}TOKENS,", self.prefix, self.prefix);
        for i in 0..this_prefix.len() {
            rust!(self.out, "{}sym{}: &mut Option<{}>,",
                  self.prefix, i, this_prefix[i].ty(&self.types));
        }
        rust!(self.out, ") -> Result<(Option<{}>, {}Nonterminal), Option<{}>> {{",
              terminal_type, self.prefix, terminal_type);

        rust!(self.out, "let mut {}result: (Option<{}>, {}Nonterminal);",
              self.prefix, terminal_type, self.prefix);

        rust!(self.out, "match {}lookahead {{", self.prefix);

        // first emit shifts:
        for (token, next_index) in
            this_state.tokens.iter()
                             .filter_map(|(token, action)| action.shift().map(|n| (token, n)))
        {
            match *token {
                Lookahead::Terminal(s) => {
                    let sym_name = format!("{}sym{}", self.prefix, this_prefix.len());
                    try!(self.consume_terminal(s, sym_name))
                }
                Lookahead::EOF =>
                    unreachable!("should never have to shift EOF")
            }

            // "shift" the lookahead onto the "stack" by taking its address
            rust!(self.out, "let {}lookahead = {}tokens.next();",
                  self.prefix, self.prefix);

            // transition to the new state
            let transition =
                self.transition(this_prefix, next_index, "lookahead", "tokens");
            rust!(self.out, "{}result = {};", self.prefix, transition);

            rust!(self.out, "}}");
            fallthrough = true;
        }

        // now emit reduces:
        for (token, production) in
            this_state.tokens.iter()
                             .filter_map(|(token, action)| action.reduce().map(|p| (token, p)))
        {
            match *token {
                Lookahead::Terminal(s) =>
                    try!(self.match_terminal(s)),
                Lookahead::EOF =>
                    rust!(self.out, "None => {{"),
            }

            let n = this_prefix.len(); // number of symbols we have on the stack
            let m = production.symbols.len(); // number action code wants
            assert!(n >= m);
            let transfer_syms = self.pop_syms(n, m);

            // "pop" the items off the stack
            for sym in &transfer_syms {
                rust!(self.out, "let {} = {}.take().unwrap();", sym, sym);
            }

            // invoke the action code
            rust!(self.out, "let {}nt = super::{}action{}({});",
                  self.prefix,
                  self.prefix,
                  production.action_fn.index(),
                  Sep(", ", &transfer_syms));

            // wrap up the result along with the (unused) lookahead
            if !transfer_syms.is_empty() {
                // if we popped anything off of the stack, then this frame is done
                rust!(self.out, "return Ok(({}lookahead, {}Nonterminal::{}({}nt)));",
                      self.prefix, self.prefix, production.nonterminal, self.prefix);
            } else {
                // otherwise, pop back
                rust!(self.out, "result = ({}lookahead, {}Nonterminal::{}({}nt));",
                      self.prefix, self.prefix, production.nonterminal, self.prefix);
                fallthrough = true;
            }

            rust!(self.out, "}}");
        }

        // if we hit this, the next token is not recognized, so generate an error
        rust!(self.out, "_ => {{");
        rust!(self.out, "return Err({}lookahead);", self.prefix);
        rust!(self.out, "}}");

        rust!(self.out, "}}"); // match

        // finally, emit gotos (if relevant)
        if fallthrough && !this_state.gotos.is_empty() {
            if this_prefix.len() > 0 {
                rust!(self.out, "while {}sym{}.is_some() {{", self.prefix, this_prefix.len() - 1);
            } else {
                rust!(self.out, "loop {{");
            }

            rust!(self.out, "let ({}lookahead, {}nt) = {}result;",
                  self.prefix, self.prefix, self.prefix);

            rust!(self.out, "match {}nt {{", self.prefix);
            for (&nt, &next_index) in &this_state.gotos {
                rust!(self.out, "{}Nonterminal::{}({}nt) => {{", self.prefix, nt, self.prefix);
                rust!(self.out, "let {}sym{} = &mut Some({}nt);",
                      self.prefix, this_prefix.len(), self.prefix);
                let transition = self.transition(this_prefix, next_index, "lookahead", "tokens");
                rust!(self.out, "{}result = {};", self.prefix, transition);
                rust!(self.out, "}}");
            }

            // errors are not possible in the goto phase; a missing entry
            // indicates parse successfully completed, so just bail out
            if this_state.gotos.len() != self.grammar.productions.keys().len() {
                rust!(self.out, "_ => {{");
                rust!(self.out, "return Ok(({}lookahead, {}nt));", self.prefix, self.prefix);
                rust!(self.out, "}}");
            }

            rust!(self.out, "}}"); // match

            rust!(self.out, "}}"); // while/loop

            if this_prefix.len() > 0 {
                rust!(self.out, "return Ok({}result);", self.prefix);
            }
        } else if fallthrough {
            rust!(self.out, "return Ok({}result);", self.prefix);
        }

        rust!(self.out, "}}"); // fn

        Ok(())
    }

    fn pop_syms(&self, depth: usize, to_pop: usize) -> Vec<String> {
        (depth-to_pop .. depth).map(|i| format!("{}sym{}", self.prefix, i)).collect()
    }

    fn transition(&self,
                  prefix: &[Symbol],
                  next_index: StateIndex,
                  lookahead: &str,
                  tokens: &str)
                  -> String
    {
        // depth of stack, including the newly shifted token
        let n = prefix.len() + 1;

        // number of tokens next state expects; will
        // always be at least 1 for the newly shifted
        // token
        let m = self.state_prefixes[next_index.0].len();
        assert!(m >= 1);

        let transfer_syms = self.pop_syms(n, m);

        // invoke next state, transferring the top `m` tokens
        format!("try!({}state{}({}{}, {}{}, {}))",
                self.prefix, next_index.0,
                self.prefix, lookahead,
                self.prefix, tokens,
                Sep(", ", &transfer_syms))
    }

    /// Emit a pattern that matches `id` but doesn't extract any data.
    fn match_terminal(&mut self, id: TerminalString) -> io::Result<()> {
        let pattern = self.grammar.pattern(id)
                                  .map(&mut |_| "_");
        rust!(self.out, "Some({}) => {{", pattern);
        Ok(())
    }

    /// Emit a pattern that matches `id` and extracts its value, storing
    /// that value as `let_name`.
    fn consume_terminal(&mut self, id: TerminalString, let_name: String) -> io::Result<()> {
        let mut pattern_names = vec![];
        let pattern = self.grammar.pattern(id).map(&mut |_| {
            let index = pattern_names.len();
            pattern_names.push(format!("{}tok{}", self.prefix, index));
            pattern_names.last().cloned().unwrap()
        });

        if pattern_names.is_empty() {
            rust!(self.out, "Some({}tok @ {}) => {{", self.prefix, pattern);
            rust!(self.out, "let mut {} = &mut Some({}tok);",
                  let_name, self.prefix);
        } else {
            rust!(self.out, "Some({}) => {{", pattern);
            rust!(self.out, "let mut {} = &mut Some(({}));",
                  let_name, pattern_names.connect(", "));
        }

        Ok(())
    }
}

