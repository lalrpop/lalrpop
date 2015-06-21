//! A compiler from an LR(1) table to a [recursive ascent] parser.
//!
//! [recursive ascent]: https://en.wikipedia.org/wiki/Recursive_ascent_parser

use intern::{intern, InternedString};
use grammar::repr::{Grammar, Production, Symbol};
use lr1::{Action, Lookahead, State, StateIndex};
use rust::RustWrite;
use std::io::{self, Write};
use util::{Sep, Set, WorkSet};

pub type Path = Vec<InternedString>;

pub fn compile<'grammar>(grammar: &'grammar Grammar,
                         action_path: &Path,
                         states: &[State<'grammar>],
                         out: &mut Write)
{
    let mut ascent = RecursiveAscent::new(grammar, action_path, states, out);
    ascent.write();
}

struct RecursiveAscent<'ascent,'grammar:'ascent> {
    grammar: &'grammar Grammar,
    action_path: &'ascent Path,
    states: &'ascent [State<'grammar>],
    state_prefixes: Vec<&'grammar [Symbol]>,
    out: RustWrite<&'ascent mut Write>,
}

impl<'ascent,'grammar> RecursiveAscent<'ascent,'grammar> {
    fn new(grammar: &'grammar Grammar,
           action_path: &'ascent Path,
           states: &'ascent [State<'grammar>],
           out: &'ascent mut Write)
           -> RecursiveAscent<'ascent,'grammar>
    {
        let num_states = states.len();

        RecursiveAscent {
            grammar: grammar,
            states: states,
            state_prefixes: states.iter().map(|s| s.prefix()).collect(),
            action_path: action_path,
            out: RustWrite::new(out),
        }
    }

    fn write(&mut self) -> io::Result<()> {
        try!(self.write_terminal_use());
        rust!(self.out, "");

        try!(self.write_return_type_defn());
        rust!(self.out, "");

        for i in 0..self.states.len() {
            try!(self.write_state_fn(StateIndex(i)));
            rust!(self.out, "");
        }

        Ok(())
    }

    fn write_terminal_use(&mut self) -> io::Result<()> {
        rust!(self.out, "use {} as Terminal;",
              self.grammar.types.terminal_type());
        Ok(())
    }

    fn write_return_type_defn(&mut self) -> io::Result<()> {
        rust!(self.out, "enum Nonterminal {{");

        // make an enum with one variant per nonterminal; I considered
        // making different enums per state, but this would mean we
        // have to unwrap and rewrap as we pass up the stack, which
        // seems silly
        for &nt in self.grammar.productions.keys() {
            rust!(self.out, "{}({}),", nt, self.grammar.types.nonterminal_type(nt));
        }

        rust!(self.out, "}}");
        Ok(())
    }

    fn write_state_fn(&mut self, this_index: StateIndex) -> io::Result<()> {
        let this_state = &self.states[this_index.0];
        let this_prefix = self.state_prefixes[this_index.0];
        let terminal_type = self.grammar.types.terminal_type();

        // Each time we shift or goto, we transfer some amount of our
        // tokens in the stack away. We then need to deal with the
        // remainder.  This code is common amongst many arms, so we
        // create helper routines for it. There is a distinct helper
        // for each possible number of tokens remaining. This set
        // tracks the helpers needed.
        let mut goto_helpers = WorkSet::new();

        // Leave a comment explaining what this state is.
        rust!(self.out, "// State {}", this_index.0);
        for item in this_state.items.iter() {
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

        rust!(self.out, "fn state{}<TOKENS>(", this_index.0);
        rust!(self.out, "lookahead: Option<{}>,", terminal_type);
        rust!(self.out, "tokens: &mut TOKENS,");
        for i in 0..this_prefix.len() {
            let term = if i < this_prefix.len()-1 {","} else {")"};
            rust!(self.out, "sym{}: {}{}", i, this_prefix[i].ty(&self.grammar.types), term);
        }
        rust!(self.out, "-> Result<(usize, Option<{}>, Nonterminal), Option<{}>>",
              terminal_type, terminal_type);
        rust!(self.out, "where TOKENS: Iterator<Item={}>", terminal_type);
        rust!(self.out, "{{");

        rust!(self.out, "match lookahead {{");
        for (token, action) in &this_state.tokens {
            match *token {
                Lookahead::Terminal(s) => {
                    rust!(self.out, "Some({}) => {{", self.grammar.pattern(s));
                }
                Lookahead::EOF => {
                    rust!(self.out, "None => {{");
                }
            }

            match *action {
                Action::Shift(next_index) => {
                    // "shift" the lookahead onto the "stack"
                    rust!(self.out, "let sym{} = lookahead;", this_prefix.len());
                    rust!(self.out, "let lookahead = tokens.next();");

                    // transition to the new state
                    let (kept_syms, result) =
                        try!(self.transition(this_prefix, next_index, "lookahead", "tokens"));

                    // handle gotos via a helper routine (see above)
                    try!(self.call_goto(this_index, "tokens", kept_syms, result,
                                        &mut goto_helpers));
                }

                Action::Reduce(production) => {
                    let n = this_prefix.len(); // number we have
                    let m = production.symbols.len(); // number action code wants
                    let (keep, transfer) = self.pop_syms(n, m);

                    // invoke the action code
                    rust!(self.out, "let nt = action{}({});",
                          production.action_fn.index(),
                          Sep(", ", &transfer));

                    // wrap up the result and handle "gotos" via a helper routine (see above)
                    rust!(self.out, "let result = ({}, Nonterminal::{}(nt));",
                          m, // number of symbols that were popped
                          self.grammar.types.nonterminal_type(production.nonterminal));
                    self.call_goto(this_index, "tokens", keep, "result", &mut goto_helpers);
                }
            }

            rust!(self.out, "}}");
        }

        // if we hit this, the next token is not recognized, so generate an error
        rust!(self.out, "_ => {{");
        rust!(self.out, "Err(lookahead)");
        rust!(self.out, "}}");

        rust!(self.out, "}}");

        rust!(self.out, "}}");

        while let Some(depth) = goto_helpers.pop() {
            try!(self.write_goto_fn(this_index, depth, &mut goto_helpers));
        }

        Ok(())
    }

    fn write_goto_fn(&mut self,
                     this_index: StateIndex,
                     keep_len: usize,
                     goto_helpers: &mut WorkSet<usize>)
                     -> io::Result<()>
    {
        let this_state = &self.states[this_index.0];
        let this_prefix = &self.state_prefixes[this_index.0][..keep_len];
        let terminal_type = self.grammar.types.terminal_type();

        rust!(self.out, "fn state{}goto{}<TOKENS>(", this_index.0, keep_len);
        rust!(self.out, "tokens: &mut TOKENS,");
        for i in 0..keep_len {
            rust!(self.out, "mut sym{}: {},", i, this_prefix[i].ty(&self.grammar.types));
        }
        rust!(self.out, "mut result: (usize, Option<{}>, Nonterminal))", terminal_type);
        rust!(self.out, "-> Result<(usize, Nonterminal), Option<{}>>", terminal_type);
        rust!(self.out, "where TOKENS: Iterator<Item={}>", terminal_type);
        rust!(self.out, "{{");

        rust!(self.out, "loop {{");
        rust!(self.out, "let (popped, lookahead, nt) = result;");

        // check whether this state has been popped and, if so, return.
        rust!(self.out, "if popped > 0 {{");
        rust!(self.out, "return (popped - 1, lookahead, nt);");
        rust!(self.out, "}}");

        // if not, we have to examine the type of the nonterminal and
        // decide what state to jump to:
        rust!(self.out, "match nt {{");
        for (&nt, &next_index) in &this_state.gotos {
            rust!(self.out, "Nonterminal::{}(sym{}) => {{", nt, keep_len + 1);

            let (kept_syms, result) =
                try!(self.transition(this_prefix, next_index, "lookahead", "tokens"));

            try!(self.maybe_call_goto(this_index, keep_len, "tokens", kept_syms,
                                      result, goto_helpers));

            rust!(self.out, "}}");
        }

        // errors are not possible in the goto phase; a missing entry
        // indicates parse successfully completed, so just pop off all
        // remaining stack frames
        rust!(self.out, "_ => {{");
        rust!(self.out, "Ok((usize::MAX, lookahead, nt))");
        rust!(self.out, "}}");

        rust!(self.out, "}}");

        rust!(self.out, "}}");
        Ok(())
    }

    fn pop_syms(&self, depth: usize, to_pop: usize) -> (Vec<String>, Vec<String>) {
        ((0 .. depth-to_pop).map(|i| format!("sym{}", i)).collect(),
         (depth-to_pop .. depth).map(|i| format!("sym{}", i)).collect())
    }

    fn transition(&mut self,
                  prefix: &[Symbol],
                  next_index: StateIndex,
                  lookahead: &str,
                  tokens: &str)
                  -> io::Result<(Vec<String>, &'static str)>
    {
        // depth of stack, including the newly shifted token
        let n = prefix.len() + 1;

        // number of tokens next state expects; will
        // always be at least 1 for the newly shifted
        // token
        let m = self.state_prefixes[next_index.0].len();
        assert!(m >= 1);

        let (kept_syms, transfer_syms) = self.pop_syms(n, m);

        // invoke next state, transferring the top `m` tokens
        rust!(self.out, "let result = try!(state{}(lookahead, tokens, {}));",
              next_index.0, Sep(", ", &transfer_syms));

        Ok((kept_syms, "result"))
    }

    fn maybe_call_goto(&mut self,
                       this_index: StateIndex,
                       this_depth: usize,
                       tokens: &str,
                       kept_syms: Vec<String>,
                       result: &str,
                       goto_helpers: &mut WorkSet<usize>)
                       -> io::Result<()>
    {
        if kept_syms.len() == this_depth {
            for keep_sym in kept_syms {
                rust!(self.out, "{} = {};", keep_sym, keep_sym);
            }
        } else {
            try!(self.call_goto(this_index, tokens, kept_syms, result, goto_helpers));
        }
        Ok(())
    }

    fn call_goto(&mut self,
                 this_index: StateIndex,
                 tokens: &str,
                 kept_syms: Vec<String>,
                 result: &str,
                 goto_helpers: &mut WorkSet<usize>)
                 -> io::Result<()>
    {
        rust!(self.out, "state{}goto{}({}, {}, {})",
              this_index.0, kept_syms.len(), tokens, Sep(", ", &kept_syms), result);
        goto_helpers.insert(kept_syms.len());
        Ok(())
    }
}

