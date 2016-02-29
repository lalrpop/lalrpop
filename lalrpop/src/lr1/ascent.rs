//! A compiler from an LR(1) table to a [recursive ascent] parser.
//!
//! [recursive ascent]: https://en.wikipedia.org/wiki/Recursive_ascent_parser

use collections::{Multimap, Set};
use grammar::repr::{Grammar,
                    NonterminalString,
                    Production,
                    Symbol,
                    TerminalString, TypeParameter, TypeRepr, Types};
use lr1::core::*;
use lr1::lookahead::Token;
use lr1::state_graph::StateGraph;
use rust::RustWrite;
use std::io::{self, Write};
use tls::Tls;
use util::{Escape, Sep};

pub fn compile<'grammar,W:Write>(
    grammar: &'grammar Grammar,
    user_start_symbol: NonterminalString,
    start_symbol: NonterminalString,
    states: &[LR1State<'grammar>],
    out: &mut RustWrite<W>)
    -> io::Result<()>
{
    let graph = StateGraph::new(&states);
    let mut ascent = RecursiveAscent::new(grammar, user_start_symbol, start_symbol,
                                          &graph, states, out);
    ascent.write()
}

struct RecursiveAscent<'ascent,'grammar:'ascent,W:Write+'ascent> {
    /// the complete grammar
    grammar: &'grammar Grammar,

    graph: &'ascent StateGraph,

    /// some suitable prefix to separate our identifiers from the user's
    prefix: &'grammar str,

    /// types from the grammar
    types: &'grammar Types,

    /// the start symbol S the user specified
    user_start_symbol: NonterminalString,

    /// the synthetic start symbol S' that we specified
    start_symbol: NonterminalString,

    /// the vector of states
    states: &'ascent [LR1State<'grammar>],

    /// for each state, the set of symbols that it will require for
    /// input
    state_inputs: Vec<StackSuffix<'grammar>>,

    /// where we write output
    out: &'ascent mut RustWrite<W>,

    /// type parameters for the `Nonterminal` type
    nonterminal_type_params: Vec<TypeParameter>,
}

/// Tracks the suffix of the stack (that is, top-most elements) that any
/// particular state is aware of. We break the suffix into two parts:
/// optional and fixed, which always look like this:
///
/// ```
/// ... A B C X Y Z
/// ~~~ ~~~~~ ~~~~~
///  |    |     |
///  |    |   Fixed (top of the stack)
///  |    |
///  |  Optional (will be popped after the fixed portion)
///  |
/// Prefix (stuff we don't know about that is also on the stack
/// ```
///
/// The idea of an "optional" member is not that it may or may not be
/// on the stack. The entire suffix will always be on the stack. An
/// *optional* member is one that *we* may or may not *consume*. So
/// the above stack suffix could occur given a state with items like:
///
/// ```
/// NT1 = A B C X Y Z (*) "."
/// NT2 = X Y Z (*) ","
/// ```
///
/// Depending on what comes next, if we reduce NT1, we will consume
/// all six symbols, but if we reduce NT2, we will only reduce three.
#[derive(Copy, Clone, Debug)]
struct StackSuffix<'grammar> {
    /// all symbols that are known to be on the stack (optional + fixed).
    all: &'grammar [Symbol],

    /// optional symbols will be consumed by *some* reductions in this
    /// state, but not all
    len_optional: usize,
}

impl<'grammar> StackSuffix<'grammar> {
    fn len(&self) -> usize {
        self.all.len()
    }

    /// returns the (optional, fixed) -- number of optional
    /// items in stack prefix and numer of fixed
    fn optional_fixed_lens(&self) -> (usize, usize) {
        (self.len_optional, self.len() - self.len_optional)
    }

    fn is_not_empty(&self) -> bool {
        self.len() > 0
    }

    fn optional(&self) -> &'grammar [Symbol] {
        &self.all[..self.len_optional]
    }

    fn fixed(&self) -> &'grammar [Symbol] {
        &self.all[self.len_optional..]
    }
}

impl<'ascent,'grammar,W:Write> RecursiveAscent<'ascent,'grammar,W> {
    fn new(grammar: &'grammar Grammar,
           user_start_symbol: NonterminalString,
           start_symbol: NonterminalString,
           graph: &'ascent StateGraph,
           states: &'ascent [LR1State<'grammar>],
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

        let state_inputs =
            states.iter()
                  .map(|state| Self::state_input_for(state))
                  .collect();

        RecursiveAscent {
            grammar: grammar,
            prefix: &grammar.prefix,
            types: &grammar.types,
            graph: graph,
            states: states,
            state_inputs: state_inputs,
            user_start_symbol: user_start_symbol,
            start_symbol: start_symbol,
            out: out,
            nonterminal_type_params: nonterminal_type_params,
        }
    }

    /// Compute the stack suffix that the state expects on entry.
    fn state_input_for(state: &'ascent LR1State<'grammar>)
                       -> StackSuffix<'grammar>
    {
        let max_prefix = state.max_prefix();
        let will_pop = state.will_pop();
        StackSuffix {
            all: max_prefix,
            len_optional: max_prefix.len() - will_pop.len(),
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
        for &nt in self.grammar.nonterminals.keys() {
            let ty = self.types.spanned_type(self.types.nonterminal_type(nt).clone());
            rust!(self.out, "{}({}),", Escape(nt), ty);
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
        rust!(self.out, "match try!({}state0({}&mut {}tokens, {}lookahead)) {{",
              self.prefix, self.grammar.user_parameter_refs(),
              self.prefix, self.prefix);

        // extra tokens?
        rust!(self.out, "(Some({}lookahead), _) => {{", self.prefix);
        rust!(self.out, "Err({}ParseError::ExtraToken {{ token: {}lookahead }})",
              self.prefix, self.prefix);
        rust!(self.out, "}}");

        // otherwise, we expect to see only the goal terminal
        rust!(self.out, "(None, {}Nonterminal::{}((_, {}nt, _))) => {{",
              self.prefix, Escape(self.start_symbol), self.prefix);
        rust!(self.out, "Ok({}nt)", self.prefix);
        rust!(self.out, "}}");

        // nothing else should be possible
        rust!(self.out, "_ => unreachable!(),");
        rust!(self.out, "}}");
        rust!(self.out, "}}");

        Ok(())
    }

    /// Writes the function that corresponds to a given state. This
    /// function takes arguments corresponding to the stack slots of
    /// the LR(1) machine. It consumes tokens and handles reduces
    /// etc. It will return once it has popped at least one symbol off
    /// of the LR stack.
    ///
    /// Note that for states which have a custom kind, this function
    /// emits nothing at all other than a possible comment explaining
    /// the state.
    fn write_state_fn(&mut self, this_index: StateIndex) -> io::Result<()> {
        let this_state = &self.states[this_index.0];
        let inputs = self.state_inputs[this_index.0];

        rust!(self.out, "");

        // Leave a comment explaining what this state is.
        if Tls::session().emit_comments {
            rust!(self.out, "// State {}", this_index.0);
            rust!(self.out, "//     AllInputs = {:?}", inputs.all);
            rust!(self.out, "//     OptionalInputs = {:?}", inputs.optional());
            rust!(self.out, "//     FixedInputs = {:?}", inputs.fixed());
            rust!(self.out, "//     WillPushLen = {:?}", this_state.will_push().len());
            rust!(self.out, "//     WillPush = {:?}", this_state.will_push());
            rust!(self.out, "//     WillProduce = {:?}", this_state.will_produce());
            rust!(self.out, "//");
            for item in this_state.items.vec.iter() {
                rust!(self.out, "//     {:?}", item);
            }
            rust!(self.out, "//");
            for (terminal, action) in &this_state.shifts {
                rust!(self.out, "//   {:?} -> {:?}", terminal, action);
            }
            for (token, action) in &this_state.reductions {
                rust!(self.out, "//   {:?} -> {:?}", token, action);
            }
            rust!(self.out, "//");
            for (nt, state) in &this_state.gotos {
                rust!(self.out, "//     {:?} -> {:?}", nt, state);
            }
        }

        try!(self.emit_state_fn_header("state", this_index.0, inputs));

        // possibly move some fixed inputs into optional stack slots
        let stack_suffix = try!(self.adjust_inputs(this_index, inputs));

        // set to true if goto actions are worth generating
        let mut fallthrough = false;

        rust!(self.out, "match {}lookahead {{", self.prefix);

        // first emit shifts:
        for (&terminal, &next_index) in &this_state.shifts {
            let sym_name = format!("{}sym{}", self.prefix, inputs.len());
            try!(self.consume_terminal(terminal, sym_name));

            // transition to the new state
            if try!(self.transition("result", stack_suffix, next_index, &["tokens"])) {
                fallthrough = true;
            }

            rust!(self.out, "}}");
        }

        // now emit reduces. It frequently happens that many tokens
        // trigger the same reduction, so group these by the
        // production that we are going to be reducing.
        let reductions: Multimap<_, Vec<_>> =
            this_state.reductions.iter()
                                 .map(|(&token, &production)| (production, token))
                                 .collect();
        for (production, tokens) in reductions {
            for (index, &token) in tokens.iter().enumerate() {
                let pattern = match token {
                    Token::Terminal(s) => format!("Some({})", self.match_terminal_pattern(s)),
                    Token::EOF => format!("None"),
                };
                if index < tokens.len() - 1 {
                    rust!(self.out, "{} |", pattern);
                } else {
                    rust!(self.out, "{} => {{", pattern);
                }
            }

            try!(self.emit_reduce_action("result", stack_suffix, production));

            if production.symbols.len() > 0 {
                // if we popped anything off of the stack, then this frame is done
                rust!(self.out, "return Ok({}result);", self.prefix);
            } else {
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
            rust!(self.out, "loop {{");

            // In most states, we know precisely when the top stack
            // slot will be consumed (basically, when we reduce or
            // when we transition to another state). But in some states,
            // we may not know. Consider:
            //
            //     X = A (*) "0" ["."]
            //     X = A (*) B ["."]
            //     B = (*) "0" "1" ["."]
            //
            // Now if we see a `"0"` this *could* be the start of a `B
            // = "0" "1"` or it could be the continuation of `X = A
            // "0"`. We won't know until we see the *next* character
            // (which will either be `"0"` or `"."`). If it turns out to be
            // `X = A "0"`, then the state handling the `"0"` will reduce
            // and consume the `A` and the `"0"`. But otherwise it will shift
            // the `"1"` and leave the `A` unprocessed.
            //
            // In cases like this, the `adjust_inputs` routine will
            // have taken the top of the stack ("A") and put it into
            // an `Option`. After the state processing the `"0"`
            // returns then, we can check this option to see whether
            // it has popped the `"A"` (in which case we ought to
            // return) or not (in which case we ought to shift the `B`
            // value that it returned to us).
            let top_slot_optional = {
                stack_suffix.is_not_empty() &&
                    stack_suffix.fixed().is_empty()
            };
            if top_slot_optional {
                rust!(self.out, "if {}sym{}.is_none() {{",
                      self.prefix, stack_suffix.len() - 1);
                rust!(self.out, "return Ok({}result);", self.prefix);
                rust!(self.out, "}}");
            }

            rust!(self.out, "let ({}lookahead, {}nt) = {}result;",
                  self.prefix, self.prefix, self.prefix);

            rust!(self.out, "match {}nt {{", self.prefix);
            for (&nt, &next_index) in &this_state.gotos {
                // The nonterminal we are shifting becomes symN, where
                // N is the number of inputs to this state (which are
                // numbered sym0..sym(N-1)). It is never optional
                // because we always transition to a state with at
                // least *one* fixed input.
                rust!(self.out, "{}Nonterminal::{}({}sym{}) => {{",
                      self.prefix, Escape(nt), self.prefix, stack_suffix.len());
                try!(self.transition("result", stack_suffix, next_index,
                                     &["tokens", "lookahead"]));
                rust!(self.out, "}}");
            }

            // Errors are not possible in the goto phase; a missing entry
            // indicates parse successfully completed, so just bail out.
            if this_state.gotos.len() != self.grammar.nonterminals.keys().len() {
                rust!(self.out, "_ => {{");
                rust!(self.out, "return Ok(({}lookahead, {}nt));",
                      self.prefix, self.prefix);
                rust!(self.out, "}}");
            }

            rust!(self.out, "}}"); // match

            rust!(self.out, "}}"); // while/loop
        } else if fallthrough {
            rust!(self.out, "return Ok({}result);", self.prefix);
        }

        rust!(self.out, "}}"); // fn

        Ok(())
    }

    fn emit_state_fn_header(
        &mut self,
        fn_kind: &str, // e.g. "state", "custom"
        fn_index: usize, // state index, custom kind index, etc
        suffix: StackSuffix<'grammar>)
        -> io::Result<()>
    {
        let optional_prefix = suffix.optional();
        let fixed_prefix = suffix.fixed();

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

        let (fn_args, starts_with_terminal) =
            self.fn_args(optional_prefix, fixed_prefix);

        try!(self.out.write_pub_fn_header(
            self.grammar,
            format!("{}{}{}", self.prefix, fn_kind, fn_index),
            vec![format!("{}TOKENS: Iterator<Item=Result<{},{}>>",
                         self.prefix, triple_type, iter_error_type)],
            fn_args,
            format!("Result<(Option<{}>, {}Nonterminal<{}>), {}>",
                    triple_type, self.prefix,
                    Sep(", ", &self.nonterminal_type_params),
                    parse_error_type),
            vec![]));

        rust!(self.out, "{{");

        rust!(self.out, "let mut {}result: (Option<{}>, {}Nonterminal<{}>);",
              self.prefix, triple_type, self.prefix,
              Sep(", ", &self.nonterminal_type_params));

        // shift lookahead is necessary; see `starts_with_terminal` above
        if starts_with_terminal {
            try!(self.next_token("lookahead", "tokens"));
        }

        Ok(())
    }

    // Compute the set of arguments that the function for a state or
    // custom-kind expects.  The argument `symbols` represents the top
    // portion of the stack which this function expects to be given.
    // Each of them will be given an argument like `sym3: &mut
    // Option<Sym3>` where `Sym3` is the type of the symbol.
    //
    // Returns a list of argument names and a flag if this fn resulted
    // from pushing a terminal (in which case the lookahead must be
    // computed interally).
    fn fn_args(&mut self,
               optional_prefix: &[Symbol],
               fixed_prefix: &[Symbol])
               -> (Vec<String>, bool) {
        assert!(
            /* start state: */ (optional_prefix.is_empty() && fixed_prefix.is_empty()) ||
            /* any other state: */ !fixed_prefix.is_empty());
        let triple_type = self.triple_type();

        // to reduce the size of the generated code, if the state
        // results from shifting a terminal, then we do not pass the
        // lookahead in as an argument, but rather we load it as the
        // first thing in this function; this saves some space because
        // there are more edges than there are states in the graph.
        let starts_with_terminal =
            fixed_prefix.last().map(|l| l.is_terminal())
                               .unwrap_or(false);


        let mut base_args =
            vec![format!("{}tokens: &mut {}TOKENS", self.prefix, self.prefix)];
        if !starts_with_terminal {
            base_args.push(format!("{}lookahead: Option<{}>", self.prefix, triple_type));
        }

        // "Optional symbols" may or may not be consumed, so take an
        // `&mut Option`
        let optional_args =
            (0..optional_prefix.len())
            .map(|i| format!("{}sym{}: &mut Option<{}>",
                             self.prefix,
                             i,
                             self.types.spanned_type(
                                 optional_prefix[i].ty(&self.types).clone())));

        // "Fixed symbols" will be consumed before we return, so take the value itself
        let fixed_args =
            (0..fixed_prefix.len())
            .map(|i| format!("{}sym{}: {}",
                             self.prefix,
                             optional_prefix.len() + i,
                             self.types.spanned_type(
                                 fixed_prefix[i].ty(&self.types).clone())));

        let all_args =
            base_args.into_iter()
                     .chain(optional_args)
                     .chain(fixed_args)
                     .collect();

        (all_args, starts_with_terminal)
    }

    /// Examine the states that we may transition to. Unless this is
    /// the start state, we will always take at least 1 fixed input:
    /// the most recently pushed symbol (let's call it `symX`), and we
    /// may have others as well. But if this state can transition to
    /// another state can takes some of those inputs as optional
    /// parameters, we need to convert them them options. This
    /// function thus emits code to move each sum `symX` into an
    /// option, and returns an adjusted stack-suffix that reflects the
    /// changes made.
    fn adjust_inputs(&mut self,
                     state_index: StateIndex,
                     inputs: StackSuffix<'grammar>)
                     -> io::Result<StackSuffix<'grammar>>
    {
        let mut result = inputs;

        let top_opt =
            self.graph.successors(state_index)
                      .iter()
                      .any(|succ_state| {
                          let succ_inputs = &self.state_inputs[succ_state.0];

                          // Check for a successor state with a suffix like:
                          //
                          //     ... OPT_1 ... OPT_N FIXED_1
                          //
                          // (Remember that *every* successor state will have
                          // at least one fixed input.)
                          //
                          // So basically we are looking for states
                          // that, when they return, may *optionally* have consumed
                          // the top of our stack.
                          assert!(succ_inputs.fixed().len() >= 1);
                          succ_inputs.fixed().len() == 1 &&
                              succ_inputs.optional().len() > 0
                      });

        // If we find a successor that may optionally consume the top
        // of our stack, convert our fixed inputs into optional ones.
        //
        // (Here we convert *all* fixed inputs. Honestly, I can't
        // remember if this is necessary, or just for simplicity. I
        // suspect the latter. --nmatsakis)
        if top_opt {
            let start_num = inputs.optional().len();
            for sym_num in (start_num .. start_num + inputs.fixed().len()) {
                rust!(self.out,
                      "let {}sym{} = &mut Some({}sym{});",
                      self.prefix, sym_num,
                      self.prefix, sym_num);
            }
            result.len_optional = result.len();
        }

        Ok(result)
    }

    /// Given that we have, locally, `optional` number of optional stack slots
    /// followed by `fixed` number of fixed stack slots, prepare the inputs
    /// to be supplied to `inputs`. Returns a string of names for this inputs.
    fn pop_syms(&mut self, optional: usize, fixed: usize, inputs: StackSuffix<'grammar>)
                -> io::Result<Vec<String>> {
        let total_have = optional + fixed;
        let total_need = inputs.len();
        (total_have - total_need .. total_have) // number relative to us
            .zip(0 .. total_need) // number relative to them
            .map(|(h, n)| {
                let name = format!("{}sym{}", self.prefix, h);
                let have_optional = h < optional;
                let need_optional = n < inputs.len_optional;

                // if we have something stored in an `Option`, but the next state
                // consumes it unconditionally, then "pop" it
                if have_optional && !need_optional {
                    rust!(self.out, "let {} = {}.take().unwrap();", name, name);
                } else {
                    // we should never have something stored
                    // unconditionally that the next state only
                    // "maybe" consumes -- we should have fixed this
                    // in the `adjust_inputs` phase
                    assert_eq!(have_optional, need_optional);
                }
                Ok(name)
            })
            .collect()
    }

    /// Emit code to shift/goto into the state `next_index`. Returns
    /// `true` if the current state may be valid after the target
    /// state returns, or `false` if `transition` will just return
    /// afterwards.
    ///
    /// # Arguments
    ///
    /// - `into_result`: name of variable to store result from target state into
    /// - `stack_suffix`: the suffix of the LR stack that current state is aware of,
    ///   and how it is distributed into optional/fixed slots
    /// - `next_index`: target state
    /// - `other_args`: other arguments we are threading along
    fn transition(&mut self,
                  into_result: &str,
                  stack_suffix: StackSuffix<'grammar>,
                  next_index: StateIndex,
                  other_args: &[&str])
                  -> io::Result<bool>
    {
        // the depth of the suffix of the stack that we are aware of
        // in the current state, including the newly shifted token
        let (optional, mut fixed) = stack_suffix.optional_fixed_lens();
        fixed += 1; // we just shifted another symbol
        let total = optional + fixed;
        assert!(total == stack_suffix.len() + 1);

        // symbols that the next state expects; will always be include
        // at least one fixed input
        let next_inputs = self.state_inputs[next_index.0];
        assert!(next_inputs.fixed().len() >= 1);
        assert!(next_inputs.len() <= total);

        let transfer_syms =
            try!(self.pop_syms(optional, fixed, next_inputs));

        let other_args =
            other_args.iter()
                      .map(|s| format!("{}{}", self.prefix, s))
                      .collect();

        let fn_name = format!("{}state{}", self.prefix, next_index.0);

        // invoke next state, transferring the top `m` tokens
        rust!(self.out,
              "{}{} = try!({}({}{}, {}));",
              self.prefix,
              into_result,
              fn_name,
              self.grammar.user_parameter_refs(),
              Sep(", ", &other_args),
              Sep(", ", &transfer_syms));

        // if the target state takes at least **two** fixed tokens,
        // then it will have consumed the top of **our** stack frame,
        // so we should just return
        if next_inputs.fixed().len() >= 2 {
            rust!(self.out, "return Ok({}{});", self.prefix, into_result);
            Ok(false)
        } else {
            Ok(true)
        }
    }

    /// Executes a reduction of `production`, storing the result into
    /// the variable `into_var`, which should have type
    /// `(Option<(L,T,L)>, Nonterminal)`.
    fn emit_reduce_action(&mut self,
                          into_var: &str,
                          stack_suffix: StackSuffix<'grammar>,
                          production: &'grammar Production)
                          -> io::Result<()>
    {
        let loc_type = self.types.terminal_loc_type();

        let (optional, fixed) = stack_suffix.optional_fixed_lens();
        let production_inputs = StackSuffix {
            all: &production.symbols,
            len_optional: 0,
        };
        let transfer_syms = try!(self.pop_syms(optional, fixed, production_inputs));

        // identify the "start" location for this production; this
        // is typically the start of the first symbol we are
        // reducing; but in the case of an empty production, it
        // will be the last symbol pushed, or at worst `default`.
        if let Some(first_sym) = transfer_syms.first() {
            rust!(self.out, "let {}start = {}.0.clone();",
                  self.prefix, first_sym);
        } else if stack_suffix.len() > 0 {
            // we pop no symbols, so grab from the top of the stack
            // (unless we are in the start state)
            let top = stack_suffix.len() - 1;
            if !stack_suffix.fixed().is_empty() {
                rust!(self.out, "let {}start = {}sym{}.2.clone();",
                      self.prefix, self.prefix, top);
            } else {
                // top of stack is optional; should not have been popped yet tho
                rust!(self.out, "let {}start = {}sym{}.as_ref().unwrap().2.clone();",
                      self.prefix, self.prefix, top);
            }
        } else {
            // this only occurs in the start state
            rust!(self.out, "let {}start: {} = ::std::default::Default::default();",
                  self.prefix, loc_type);
        }

        // identify the "end" location for this production;
        // this is typically the end of the last symbol we are reducing,
        // but in the case of an empty production it will come from the
        // lookahead
        if let Some(last_sym) = transfer_syms.last() {
            rust!(self.out, "let {}end = {}.2.clone();",
                  self.prefix, last_sym);
        } else {
            rust!(self.out, "let {}end = {}lookahead.as_ref()\
                             .map(|o| o.0.clone())\
                             .unwrap_or_else(|| {}start.clone());",
                  self.prefix, self.prefix, self.prefix);
        }

        let transfered_syms = transfer_syms.len();

        let mut args = transfer_syms;
        if transfered_syms == 0 {
            args.push(format!("&{}start", self.prefix));
            args.push(format!("&{}end", self.prefix));
        }

        // invoke the action code
        let is_fallible = self.grammar.action_is_fallible(production.action);
        if is_fallible {
            rust!(self.out, "let {}nt = try!(super::{}action{}({}{}));",
                  self.prefix,
                  self.prefix,
                  production.action.index(),
                  self.grammar.user_parameter_refs(),
                  Sep(", ", &args))
        } else {
            rust!(self.out, "let {}nt = super::{}action{}({}{});",
                  self.prefix,
                  self.prefix,
                  production.action.index(),
                  self.grammar.user_parameter_refs(),
                  Sep(", ", &args))
        }

        // wrap up the produced value into `Nonterminal` along with
        rust!(self.out, "let {}nt = {}Nonterminal::{}((",
              self.prefix, self.prefix, Escape(production.nonterminal));
        rust!(self.out, "{}start,", self.prefix);
        rust!(self.out, "{}nt,", self.prefix);
        rust!(self.out, "{}end,", self.prefix);
        rust!(self.out, "));");

        // wrap up the result along with the (unused) lookahead
        rust!(self.out, "{}{} = ({}lookahead, {}nt);",
              self.prefix, into_var, self.prefix, self.prefix);

        Ok(())
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
                        let_name: String)
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

        pattern = format!("({}loc1, {}, {}loc2)", self.prefix, pattern, self.prefix);

        rust!(self.out, "Some({}) => {{", pattern);

        rust!(self.out, "let {} = ({}loc1, ({}), {}loc2);",
              let_name, self.prefix, pattern_names.join(", "), self.prefix);

        Ok(())
    }

    fn triple_type(&self) -> TypeRepr {
        self.types.triple_type()
    }

    fn parse_error_type(&self) -> String {
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
