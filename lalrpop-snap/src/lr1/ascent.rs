//! A compiler from an LR(1) table to a [recursive ascent] parser.
//!
//! [recursive ascent]: https://en.wikipedia.org/wiki/Recursive_ascent_parser

use grammar::repr::{ActionKind,
                    Grammar,
                    NonterminalString,
                    Symbol,
                    TerminalString, TypeParameter, TypeRepr, Types};
use lr1::{Lookahead, State, StateIndex};
use rust::RustWrite;
use std::io::{self, Write};
use util::{Escape, Multimap, Set, Sep};

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
    /// the complete grammar
    grammar: &'grammar Grammar,

    /// some suitable prefix to separate our identifiers from the user's
    prefix: &'grammar str,

    /// types from the grammar
    types: &'grammar Types,

    /// the start symbol S the user specified
    user_start_symbol: NonterminalString,

    /// the synthetic start symbol S' that we specified
    start_symbol: NonterminalString,

    /// the vector of states
    states: &'ascent [State<'grammar>],

    /// for each state, the maximal prefix of tokens on the stack;
    /// this is the number of arguments that the state fn expects
    state_prefixes: Vec<&'grammar [Symbol]>,

    /// where we write output
    out: &'ascent mut RustWrite<W>,

    /// type parameters for the `Nonterminal` type
    nonterminal_type_params: Vec<TypeParameter>,
}

impl<'ascent,'grammar,W:Write> RecursiveAscent<'ascent,'grammar,W> {
    fn new(grammar: &'grammar Grammar,
           user_start_symbol: NonterminalString,
           start_symbol: NonterminalString,
           states: &'ascent [State<'grammar>],
           out: &'ascent mut RustWrite<W>)
           -> RecursiveAscent<'ascent,'grammar,W>
    {
        // The nonterminal type needs to be parameterized by all the
        // type parameters that actually appear in the types of
        // nonterminals.  We can't just use *all* type parameters
        // because that would leave unused lifetime/type parameters in
        // some cases.
        let referenced_ty_params: Set<TypeParameter> =
            grammar.types.nonterminal_types()
                         .into_iter()
                         .flat_map(|t| t.referenced())
                         .collect();

        let nonterminal_type_params: Vec<_> =
            grammar.type_parameters.iter()
                                   .filter(|t| referenced_ty_params.contains(t))
                                   .cloned()
                                   .collect();

        RecursiveAscent {
            grammar: grammar,
            prefix: &grammar.prefix,
            types: &grammar.types,
            states: states,
            state_prefixes: states.iter().map(|s| s.prefix()).collect(),
            user_start_symbol: user_start_symbol,
            start_symbol: start_symbol,
            out: out,
            nonterminal_type_params: nonterminal_type_params,
        }
    }

    fn write(&mut self) -> io::Result<()> {
        rust!(self.out, "");
        rust!(self.out, "mod {}parse{} {{",
              self.prefix, self.start_symbol);

        // these stylistic lints are annoying for the generated code,
        // which doesn't follow conventions:
        rust!(self.out, "#![allow(non_snake_case, non_camel_case_types, \
                         unused_mut, unused_variables, unused_imports)]");
        rust!(self.out, "");

        try!(self.write_uses());

        try!(self.write_start_fn());

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
        try!(self.out.write_uses("super::", &self.grammar));

        if self.grammar.intern_token.is_none() {
            rust!(self.out, "use super::{}ToTriple;", self.prefix);
        }

        Ok(())
    }

    fn write_return_type_defn(&mut self) -> io::Result<()> {
        // sometimes some of the variants are not used, particularly
        // if we are generating multiple parsers from the same file:
        rust!(self.out, "#[allow(dead_code)]");
        rust!(self.out, "pub enum {}Nonterminal<{}> {{",
              self.prefix,
              Sep(", ", &self.nonterminal_type_params));

        // make an enum with one variant per nonterminal; I considered
        // making different enums per state, but this would mean we
        // have to unwrap and rewrap as we pass up the stack, which
        // seems silly
        for &nt in self.grammar.productions.keys() {
            rust!(self.out, "{}({}),", Escape(nt), self.types.nonterminal_type(nt));
        }

        rust!(self.out, "}}");
        Ok(())
    }

    // Generates a function `parse_Foo` that will parse an entire
    // input as `Foo`. An error is reported if the entire input is not
    // consumed.
    fn write_start_fn(&mut self) -> io::Result<()> {
        let error_type = self.types.error_type();
        let parse_error_type = self.parse_error_type();

        let (type_parameters, parameters);

        if self.grammar.intern_token.is_some() {
            // if we are generating the tokenizer, we just need the
            // input, and that has already been added as one of the
            // user parameters
            type_parameters = vec![];
            parameters = vec![];
        } else {
            // otherwise, we need an iterator of type `TOKENS`
            let mut user_type_parameters = String::new();
            for type_parameter in &self.grammar.type_parameters {
                user_type_parameters.push_str(&format!("{}, ", type_parameter));
            }
            type_parameters = vec![
                format!("{}TOKEN: {}ToTriple<{}Error={}>",
                        self.prefix, self.prefix, user_type_parameters, error_type),
                format!("{}TOKENS: IntoIterator<Item={}TOKEN>", self.prefix, self.prefix)];
            parameters = vec![format!("{}tokens: {}TOKENS", self.prefix, self.prefix)];
        }

        try!(self.out.write_pub_fn_header(
            self.grammar,
            format!("parse_{}", self.user_start_symbol),
            type_parameters,
            parameters,
            format!("Result<{}, {}>",
                    self.types.nonterminal_type(self.start_symbol),
                    parse_error_type),
            vec![]));
        rust!(self.out, "{{");

        if self.grammar.intern_token.is_some() {
            // if we are generating the tokenizer, create a matcher as our input iterator
            rust!(self.out, "let mut {}tokens = super::{}intern_token::{}Matcher::new(input);",
                  self.prefix, self.prefix, self.prefix);
        } else {
            // otherwise, convert one from the `IntoIterator`
            // supplied, using the `ToTriple` trait which inserts
            // errors/locations etc if none are given
            rust!(self.out, "let {}tokens = {}tokens.into_iter();", self.prefix, self.prefix);
            rust!(self.out, "let mut {}tokens = {}tokens.map(|t| {}ToTriple::to_triple(t));",
                  self.prefix, self.prefix, self.prefix);
        }

        try!(self.next_token("lookahead", "tokens"));
        rust!(self.out, "match try!({}state0({}None, &mut {}tokens, {}lookahead)) {{",
              self.prefix, self.grammar.user_parameter_refs(), self.prefix, self.prefix);

        // extra tokens?
        rust!(self.out, "(_, Some({}lookahead), _) => {{", self.prefix);
        rust!(self.out, "Err({}ParseError::ExtraToken {{ token: {}lookahead }})",
              self.prefix, self.prefix);
        rust!(self.out, "}}");

        // otherwise, we expect to see only the goal terminal
        rust!(self.out, "(_, None, {}Nonterminal::{}({}nt)) => {{",
              self.prefix, Escape(self.start_symbol), self.prefix);
        rust!(self.out, "Ok({}nt)", self.prefix);
        rust!(self.out, "}}");

        // nothing else should be possible
        rust!(self.out, "_ => unreachable!(),");
        rust!(self.out, "}}");
        rust!(self.out, "}}");

        Ok(())
    }

    fn write_state_fn(&mut self, this_index: StateIndex) -> io::Result<()> {
        let this_state = &self.states[this_index.0];
        let this_prefix = self.state_prefixes[this_index.0];
        let loc_type = self.types.terminal_loc_type();
        let triple_type = self.triple_type();
        let parse_error_type = self.parse_error_type();
        let error_type = self.types.error_type();

        // If we are generated the tokenizer, it generates ParseError
        // errors, otherwise they are user errors.
        let iter_error_type = if self.grammar.intern_token.is_some() {
            parse_error_type.clone()
        } else {
            format!("{}", error_type)
        };

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

        // to reduce the size of the generated code, if the state
        // results from shifting a terminal, then we do not pass the
        // lookahead in as an argument, but rather we load it as the
        // first thing in this function; this saves some space because
        // there are more edges than there are states in the graph.
        let starts_with_terminal =
            this_prefix.last().map(|l| l.is_terminal())
                              .unwrap_or(false);

        // compute the set of arguments that this state function expects
        let mut base_args =
            vec![format!("{}lookbehind: Option<{}>", self.prefix, loc_type),
                 format!("{}tokens: &mut {}TOKENS", self.prefix, self.prefix)];
        if !starts_with_terminal {
            base_args.push(format!("{}lookahead: Option<{}>", self.prefix, triple_type));
        }
        let sym_args: Vec<_> =
            (0..this_prefix.len())
            .map(|i| format!("{}sym{}: &mut Option<{}>",
                             self.prefix, i, this_prefix[i].ty(&self.types)))
            .collect();

        try!(self.out.write_pub_fn_header(
            self.grammar,
            format!("{}state{}", self.prefix, this_index.0),
            vec![format!("{}TOKENS: Iterator<Item=Result<{},{}>>",
                         self.prefix, triple_type, iter_error_type)],
            base_args.into_iter().chain(sym_args).collect(),
            format!("Result<(Option<{}>, Option<{}>, {}Nonterminal<{}>), {}>",
                    loc_type,
                    triple_type, self.prefix,
                    Sep(", ", &self.nonterminal_type_params),
                    parse_error_type),
            vec![]));

        rust!(self.out, "{{");
        rust!(self.out, "let mut {}result: (Option<{}>, Option<{}>, {}Nonterminal<{}>);",
              self.prefix, loc_type, triple_type, self.prefix,
              Sep(", ", &self.nonterminal_type_params));

        // shift lookahead is necessary; see `starts_with_terminal` above
        if starts_with_terminal {
            try!(self.next_token("lookahead", "tokens"));
        }

        rust!(self.out, "match {}lookahead {{", self.prefix);

        // first emit shifts:
        for (token, next_index) in
            this_state.tokens.iter()
                             .filter_map(|(token, action)| action.shift().map(|n| (token, n)))
        {
            match *token {
                Lookahead::Terminal(s) => {
                    let sym_name = format!("{}sym{}", self.prefix, this_prefix.len());
                    let lb_name = format!("{}lookbehind", self.prefix);
                    try!(self.consume_terminal(s, sym_name, lb_name));
                }
                Lookahead::EOF =>
                    unreachable!("should never have to shift EOF")
            }

            // transition to the new state
            let transition =
                self.transition(this_prefix, next_index, &["lookbehind", "tokens"]);
            rust!(self.out, "{}result = {};", self.prefix, transition);

            rust!(self.out, "}}");
            fallthrough = true;
        }

        // now emit reduces. It frequently happens that many tokens
        // trigger the same reduction, so group these by the
        // production that we are going to be reducing.
        let reductions: Multimap<_, _> =
            this_state.tokens.iter()
                             .filter_map(|(&token, action)| action.reduce().map(|p| (p, token)))
                             .collect();
        for (production, tokens) in reductions {
            for (index, &token) in tokens.iter().enumerate() {
                let pattern = match token {
                    Lookahead::Terminal(s) => format!("Some({})", self.match_terminal_pattern(s)),
                    Lookahead::EOF => format!("None"),
                };
                if index < tokens.len() - 1 {
                    rust!(self.out, "{} |", pattern);
                } else {
                    rust!(self.out, "{} => {{", pattern);
                }
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
            match production.action {
                ActionKind::Call(action_fn) => {
                    rust!(self.out, "let {}nt = super::{}action{}({}{});",
                          self.prefix,
                          self.prefix,
                          action_fn.index(),
                          self.grammar.user_parameter_refs(),
                          Sep(", ", &transfer_syms))
                }

                ActionKind::TryCall(action_fn) => {
                    rust!(self.out, "let {}nt = try!(super::{}action{}({}{}));",
                          self.prefix,
                          self.prefix,
                          action_fn.index(),
                          self.grammar.user_parameter_refs(),
                          Sep(", ", &transfer_syms))
                }

                ActionKind::Lookahead => {
                    // take the lookahead, if any; otherwise, we are
                    // at EOF, so taker the lookbehind (end of last
                    // pushed token); if that is missing too, then
                    // supply default.
                    rust!(self.out,
                          "let {}nt = \
                               {}lookahead.as_ref()\
                                          .map(|o| ::std::clone::Clone::clone(&o.0))\
                                          .or_else(|| ::std::clone::Clone::clone(&{}lookbehind))\
                                          .unwrap_or_default();",
                          self.prefix, self.prefix, self.prefix);
                }

                ActionKind::Lookbehind => {
                    // take lookbehind or supply default.
                    rust!(self.out,
                          "let {}nt = ::std::clone::Clone::clone(&{}lookbehind)\
                                      .unwrap_or_default();",
                          self.prefix, self.prefix);
                }
            }

            // wrap up the result along with the (unused) lookahead
            if !transfer_syms.is_empty() {
                // if we popped anything off of the stack, then this frame is done
                rust!(self.out, "return Ok(({}lookbehind, {}lookahead, {}Nonterminal::{}({}nt)));",
                      self.prefix, self.prefix, self.prefix,
                      Escape(production.nonterminal), self.prefix);
            } else {
                // otherwise, pop back
                rust!(self.out, "{}result = ({}lookbehind, {}lookahead, {}Nonterminal::{}({}nt));",
                      self.prefix, self.prefix, self.prefix, self.prefix,
                      Escape(production.nonterminal), self.prefix);
                fallthrough = true;
            }

            rust!(self.out, "}}");
        }

        // if we hit this, the next token is not recognized, so generate an error
        rust!(self.out, "_ => {{");
        rust!(self.out, "return Err({}ParseError::UnrecognizedToken {{", self.prefix);
        rust!(self.out, "token: {}lookahead,", self.prefix);
        rust!(self.out, "expected: vec![],");
        rust!(self.out, "}});");
        rust!(self.out, "}}");

        rust!(self.out, "}}"); // match

        // finally, emit gotos (if relevant)
        if fallthrough && !this_state.gotos.is_empty() {
            if this_prefix.len() > 0 {
                rust!(self.out, "while {}sym{}.is_some() {{", self.prefix, this_prefix.len() - 1);
            } else {
                rust!(self.out, "loop {{");
            }

            rust!(self.out, "let ({}lookbehind, {}lookahead, {}nt) = {}result;",
                  self.prefix, self.prefix, self.prefix, self.prefix);

            rust!(self.out, "match {}nt {{", self.prefix);
            for (&nt, &next_index) in &this_state.gotos {
                rust!(self.out, "{}Nonterminal::{}({}nt) => {{",
                      self.prefix, Escape(nt), self.prefix);
                rust!(self.out, "let {}sym{} = &mut Some({}nt);",
                      self.prefix, this_prefix.len(), self.prefix);
                let transition = self.transition(this_prefix, next_index,
                                                 &["lookbehind", "tokens", "lookahead"]);
                rust!(self.out, "{}result = {};", self.prefix, transition);
                rust!(self.out, "}}");
            }

            // errors are not possible in the goto phase; a missing entry
            // indicates parse successfully completed, so just bail out
            if this_state.gotos.len() != self.grammar.productions.keys().len() {
                rust!(self.out, "_ => {{");
                rust!(self.out, "return Ok(({}lookbehind, {}lookahead, {}nt));",
                      self.prefix, self.prefix, self.prefix);
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
                  other_args: &[&str])
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

        let other_args =
            other_args.iter()
                      .map(|s| format!("{}{}", self.prefix, s))
                      .collect();

        // invoke next state, transferring the top `m` tokens
        format!("try!({}state{}({}{}, {}))",
                self.prefix, next_index.0,
                self.grammar.user_parameter_refs(),
                Sep(", ", &other_args),
                Sep(", ", &transfer_syms))
    }

    /// Emit a pattern that matches `id` but doesn't extract any data.
    fn match_terminal_pattern(&mut self, id: TerminalString) -> String {
        let pattern = self.grammar.pattern(id)
                                      .map(&mut |_| "_");
        let pattern = format!("{}", pattern);
        format!("(_, {}, _)", pattern)
    }

    /// Emit a pattern that matches `id` and extracts its value, storing
    /// that value as `let_name`.
    fn consume_terminal(&mut self,
                        id: TerminalString,
                        let_name: String,
                        lb_name: String)
                        -> io::Result<()> {
        let mut pattern_names = vec![];
        let pattern = self.grammar.pattern(id).map(&mut |_| {
            let index = pattern_names.len();
            pattern_names.push(format!("{}tok{}", self.prefix, index));
            pattern_names.last().cloned().unwrap()
        });

        let mut pattern = format!("{}", pattern);
        if pattern_names.is_empty() {
            pattern_names.push(format!("{}tok", self.prefix));
            pattern = format!("{}tok @ {}", self.prefix, pattern);
        }

        pattern = format!("(_, {}, {}loc)", pattern, self.prefix);

        rust!(self.out, "Some({}) => {{", pattern);

        rust!(self.out, "let mut {} = Some({}loc);",
              lb_name, self.prefix);

        rust!(self.out, "let mut {} = &mut Some(({}));",
              let_name, pattern_names.join(", "));

        Ok(())
    }

    fn triple_type(&mut self) -> TypeRepr {
        self.types.triple_type()
    }

    fn parse_error_type(&mut self) -> String {
        format!("{}ParseError<{},{},{}>",
                self.prefix,
                self.types.terminal_loc_type(),
                self.types.terminal_token_type(),
                self.types.error_type())
    }

    fn next_token(&mut self, lookahead: &str, tokens: &str) -> io::Result<()> {
        rust!(self.out, "let {}{} = match {}{}.next() {{",
              self.prefix, lookahead, self.prefix, tokens);
        rust!(self.out, "Some(Ok(v)) => Some(v),");
        rust!(self.out, "None => None,");
        if self.grammar.intern_token.is_some() {
            // when we generate the tokenizer, the generated errors are `ParseError` values
            rust!(self.out, "Some(Err(e)) => return Err(e),");
        } else {
            // otherwise, they are user errors
            rust!(self.out, "Some(Err(e)) => return Err({}ParseError::User {{ error: e }}),",
                  self.prefix);
        }
        rust!(self.out, "}};");
        Ok(())
    }
}

