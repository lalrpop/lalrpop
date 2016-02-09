//! Error reporting. For now very stupid and simplistic.

use itertools::Itertools;
use grammar::repr::*;
use session::Session;
use std::io::{self, Result, Write};
use util::{Map, map};
use lr1::backtrace::{Example, ExampleStyles, ExampleSymbol, Tracer};
use lr1::lookahead::{Lookahead, LookaheadSet};
use super::{Action, Conflict, LR0Item, Item,
            State, StateIndex, TableConstructionError};

#[cfg(test)] mod test;

pub fn report_error(session: &Session,
                    grammar: &Grammar,
                    error: &TableConstructionError,
                    out: &mut Write)
                    -> io::Result<()>
{
    let cx = ErrorReportingCx::new(session, grammar, &error.states);
    cx.report_errors(out)
}

struct ErrorReportingCx<'cx> {
    session: &'cx Session,
    grammar: &'cx Grammar,
    states: &'cx [State<'cx>],
}

#[derive(Debug)]
enum ConflictClassification {
    /// The grammar is ambiguous. This means we have two examples of
    /// precisely the same set of symbols which can be reduced in two
    /// distinct ways.
    Ambiguity { action: Example, reduce: Example },

    /// The grammar is ambiguous, and moreover it looks like a
    /// precedence error. This means that the reduction is to a
    /// nonterminal `T` and the shift is some symbol sandwiched
    /// between two instances of `T`.
    Precedence { shift: Example, reduce: Example, nonterminal: NonterminalString },

    /// Suggest inlining `nonterminal`. Makes sense if there are two
    /// levels in the reduction tree in both examples, and the suffix
    /// after the inner reduction is the same in all cases.
    SuggestInline { shift: Example, reduce: Example,
                    nonterminal: NonterminalString },

    /// Like the previous, but suggest replacing `nonterminal` with
    /// `symbol?`. Makes sense if the thing to be inlined consists of
    /// two alternatives, `X = symbol | ()`.
    SuggestQuestion { shift: Example, reduce: Example,
                      nonterminal: NonterminalString, symbol: Symbol },

    /// Can't say much beyond that a conflict occurred.
    InsufficientLookahead { action: Example, reduce: Example },

    /// Really can't say *ANYTHING*.
    Naive,
}

impl<'cx> ErrorReportingCx<'cx> {
    fn new(session: &'cx Session, grammar: &'cx Grammar, states: &'cx [State<'cx>])
           -> Self {
        ErrorReportingCx {
            session: session,
            grammar: grammar,
            states: states,
        }
    }

    fn report_errors(&self, out: &mut Write) -> io::Result<()> {
        for state in self.states {
            for (&lookahead, conflicts) in &state.conflicts {
                for conflict in conflicts {
                    try!(self.report_error(lookahead, conflict, out));
                }
            }
        }
        Ok(())
    }

    fn report_error(&self,
                    lookahead: Lookahead,
                    conflict: &Conflict,
                    out: &mut Write)
                    -> io::Result<()>
    {
        match self.classify(lookahead, conflict) {
            ConflictClassification::Ambiguity { action, reduce } => {
                self.report_error_ambiguity(action, reduce, out)
            }
            ConflictClassification::Precedence { shift, reduce, nonterminal } => {
                self.report_error_precedence(shift, reduce, nonterminal, out)
            }
            ConflictClassification::SuggestInline { shift, reduce, nonterminal } => {
                self.report_error_suggest_inline(lookahead, conflict,
                                                 shift, reduce,
                                                 nonterminal, out)
            }
            ConflictClassification::SuggestQuestion { shift, reduce,
                                                      nonterminal, symbol } => {
                self.report_error_suggest_question(lookahead, conflict,
                                                   shift, reduce,
                                                   nonterminal, symbol, out)
            }
            ConflictClassification::InsufficientLookahead { action, reduce } => {
                self.report_error_insufficient_lookahead(lookahead, conflict,
                                                         action, reduce,
                                                         out)
            }
            ConflictClassification::Naive => {
                self.report_error_naive(lookahead, conflict, out)
            }
        }
    }

    fn paint_example(&self,
                     example: &Example,
                     styles: &ExampleStyles,
                     out: &mut Write)
                     -> io::Result<()> {
        for line in example.paint(styles) {
            try!(writeln!(out, "{}", line));
        }
        Ok(())
    }

    fn paint_example_symbols(&self,
                             example: &Example,
                             number_syms: usize,
                             styles: &ExampleStyles,
                             out: &mut Write)
                             -> io::Result<()> {
        let line = example.paint_symbols(number_syms, styles);
        try!(writeln!(out, "{}", line));
        Ok(())
    }

    fn report_error_ambiguity_core(&self,
                                   shift: &Example,
                                   reduce: &Example,
                                   out: &mut Write)
                                   -> io::Result<()> {
        let styles = ExampleStyles::ambig(self.session);
        try!(writeln!(out, "{}",
                      self.session.heading.paint("Ambiguous grammar detected")));
        try!(writeln!(out, ""));
        try!(writeln!(out, "The following symbols can be reduced in two ways:"));
        try!(self.paint_example_symbols(shift, shift.symbols.len(), &styles, out));
        try!(writeln!(out, "They could be reduced like so:"));
        try!(self.paint_example(shift, &styles, out));
        try!(writeln!(out, "Alternatively, they could be reduced like so:"));
        try!(self.paint_example(reduce, &styles, out));
        Ok(())
    }

    fn report_error_ambiguity(&self,
                              shift: Example,
                              reduce: Example,
                              out: &mut Write)
                              -> io::Result<()> {
        try!(self.report_error_ambiguity_core(&shift, &reduce, out));
        try!(writeln!(out, ""));
        try!(writeln!(out, "LALRPOP does not yet support ambiguous grammars."));
        try!(writeln!(out, "See the LALRPOP manual for advice on \
                            making your grammar unambiguous."));
        Ok(())
    }

    fn report_error_precedence(&self,
                               shift: Example,
                               reduce: Example,
                               nonterminal: NonterminalString,
                               out: &mut Write)
                               -> io::Result<()> {
        try!(self.report_error_ambiguity_core(&shift, &reduce, out));
        try!(writeln!(out, ""));
        try!(writeln!(out, "This looks like a precedence error related to `{}`.",
                      nonterminal));
        try!(writeln!(out, "See the LALRPOP manual for advice on encoding precedence."));
        Ok(())
    }

    fn report_error_not_lr1_core(&self,
                                 lookahead: Lookahead,
                                 conflict: &Conflict,
                                 action: &Example,
                                 reduce: &Example,
                                 out: &mut Write)
                                 -> io::Result<()> {
        let styles = ExampleStyles::new(self.session);
        try!(writeln!(out, "{}", self.session.heading.paint("Grammar is not LR(1)")));
        try!(writeln!(out, ""));
        try!(writeln!(out, "The grammar as written cannot be parsed using an LR(1)"));
        try!(writeln!(out, "parser, as more than one token of lookahead would be required."));
        try!(writeln!(out, "After encountering the following symbols in the input:"));
        if action.cursor >= reduce.cursor {
            try!(self.paint_example_symbols(&action, action.cursor, &styles, out));
        } else {
            try!(self.paint_example_symbols(&reduce, reduce.cursor, &styles, out));
        }
        match lookahead {
            Lookahead::Terminal(term) => {
                let term = term.to_string();
                try!(writeln!(out, "and when looking at the terminal `{}`,",
                              self.session.cursor_symbol.paint(term)));
            }
            Lookahead::EOF => {
                try!(writeln!(out, "and when there are no more input tokens,"));
            }
        }
        try!(writeln!(out, "parsing could continue in two distinct ways."));

        try!(writeln!(out, ""));
        match conflict.action {
            Action::Shift(_) =>
                try!(writeln!(out, "First, the parser could take no action yet, \
                                    leading to:")),
            Action::Reduce(production) =>
                try!(writeln!(out, "First, the parser could reduce `{}`, \
                                    leading to:", production.nonterminal)),
        }
        try!(self.paint_example(&action, &styles, out));

        try!(writeln!(out, ""));
        try!(writeln!(out, "Second, the parser could reduce `{}`, leading to:",
                      reduce.reductions[0].nonterminal));
        try!(self.paint_example(&reduce, &styles, out));
        try!(writeln!(out, "(Note that an LR(1) parser must execute reductions"));
        try!(writeln!(out, "as soon as it can.)"));

        Ok(())
    }

    fn report_error_suggest_inline(&self,
                                   lookahead: Lookahead,
                                   conflict: &Conflict,
                                   shift: Example,
                                   reduce: Example,
                                   nonterminal: NonterminalString,
                                   out: &mut Write)
                                   -> io::Result<()> {
        try!(self.report_error_not_lr1_core(lookahead, conflict, &shift, &reduce, out));
        try!(writeln!(out, ""));
        try!(writeln!(out, "It appears you could resolve this problem by adding"));
        try!(writeln!(out, "the annotation `#[inline]` to the definition of `{}`.",
                      nonterminal));
        try!(writeln!(out, "For more information, see the section on inlining"));
        try!(writeln!(out, "in the LALROP manual."));
        Ok(())
    }

    fn report_error_suggest_question(&self,
                                     lookahead: Lookahead,
                                     conflict: &Conflict,
                                     shift: Example,
                                     reduce: Example,
                                     nonterminal: NonterminalString,
                                     symbol: Symbol,
                                     out: &mut Write)
                                     -> io::Result<()> {
        try!(self.report_error_not_lr1_core(lookahead, conflict, &shift, &reduce, out));
        try!(writeln!(out, ""));
        try!(writeln!(out, "It appears you could resolve this problem by replacing"));
        try!(writeln!(out, "uses of `{}` with `{}?` (or, alternatively, by adding the",
                      nonterminal, symbol));
        try!(writeln!(out, "annotation `#[inline]` to the definition of `{}`.",
                      nonterminal));
        try!(writeln!(out, "For more information, see the section on inlining"));
        try!(writeln!(out, "in the LALROP manual."));
        Ok(())
    }

    fn report_error_insufficient_lookahead(&self,
                                           lookahead: Lookahead,
                                           conflict: &Conflict,
                                           action: Example,
                                           reduce: Example,
                                           out: &mut Write)
                                           -> io::Result<()> {
        try!(self.report_error_not_lr1_core(lookahead, conflict, &action, &reduce, out));
        try!(writeln!(out, ""));
        try!(writeln!(out, "See the LALRPOP manual for advice on \
                            making your grammar LR(1)."));
        Ok(())
    }

    /// Naive error reporting. This is still used for LALR(1) reduction
    /// errors but ought to be phased out completely, I imagine.
    fn report_error_naive(&self,
                          lookahead: Lookahead,
                          conflict: &Conflict,
                          out: &mut Write)
                          -> io::Result<()> {
        try!(writeln!(out, "when in this state:"));
        for item in self.states[conflict.state.0].items.vec.iter() {
            try!(writeln!(out, "  {:?}", item));
        }
        try!(writeln!(out, "and looking at a token `{:?}`,", lookahead));
        try!(writeln!(out, "we can reduce to a `{}`",
                      conflict.production.nonterminal));
        match conflict.action {
            Action::Shift(_) =>
                try!(writeln!(out, "but we can also shift")),
            Action::Reduce(prod) =>
                try!(writeln!(out, "but we can also reduce to a `{}`",
                              prod.nonterminal)),
        }
        Ok(())
    }

    fn tracer(&self) -> Tracer<'cx, 'cx> {
        Tracer::new(self.session, self.grammar, self.states)
    }

    fn classify(&self,
                lookahead: Lookahead,
                conflict: &'cx Conflict<'cx>)
                -> ConflictClassification
    {
        // Find examples from the conflicting action (either a shift
        // or a reduce).
        let action_examples = match conflict.action {
            Action::Shift(_) => self.shift_examples(lookahead, conflict),
            Action::Reduce(production) => self.reduce_examples(conflict.state,
                                                               production,
                                                               lookahead)
        };

        // Find examples from the conflicting reduce.
        let reduce_examples = self.reduce_examples(conflict.state,
                                                   conflict.production,
                                                   lookahead);

        if let Some(classification) = self.try_classify_ambiguity(lookahead,
                                                                  conflict,
                                                                  &action_examples,
                                                                  &reduce_examples) {
            return classification;
        }

        if let Some(classification) = self.try_classify_inline(lookahead,
                                                               conflict,
                                                               &action_examples,
                                                               &reduce_examples) {
            return classification;
        }

        // Give up. Just grab an example from each and pair them up.
        // If there aren't even two examples, something's pretty
        // bogus, but we'll just call it naive.
        action_examples
            .into_iter()
            .zip(reduce_examples)
            .next()
            .map(|(action, reduce)| {
                ConflictClassification::InsufficientLookahead {
                    action: action,
                    reduce: reduce,
                }
            })
            .unwrap_or(ConflictClassification::Naive)
    }

    fn try_classify_ambiguity(&self,
                              lookahead: Lookahead,
                              conflict: &'cx Conflict<'cx>,
                              action_examples: &[Example],
                              reduce_examples: &[Example])
                              -> Option<ConflictClassification> {
        action_examples
            .iter()
            .cartesian_product(reduce_examples)
            .filter(|&(action, reduce)| action.symbols == reduce.symbols)
            .filter(|&(action, reduce)| action.cursor == reduce.cursor)
            .map(|(action, reduce)| {
                // Consider whether to call this a precedence
                // error. We do this if we are stuck between reducing
                // `T = T S T` and shifting `S`.
                if let Action::Shift(_) = conflict.action {
                    if let Lookahead::Terminal(term) = lookahead {
                        let nt = conflict.production.nonterminal;
                        if conflict.production.symbols.len() == 3 &&
                            conflict.production.symbols[0] == Symbol::Nonterminal(nt) &&
                            conflict.production.symbols[1] == Symbol::Terminal(term) &&
                            conflict.production.symbols[2] == Symbol::Nonterminal(nt)
                        {
                            return ConflictClassification::Precedence {
                                shift: action.clone(),
                                reduce: reduce.clone(),
                                nonterminal: conflict.production.nonterminal,
                            };
                        }
                    }
                }
                ConflictClassification::Ambiguity {
                    action: action.clone(),
                    reduce: reduce.clone()
                }
            })
            .next()
    }

    fn try_classify_inline(&self,
                           _lookahead: Lookahead,
                           _conflict: &'cx Conflict<'cx>,
                           action_examples: &[Example],
                           reduce_examples: &[Example])
                           -> Option<ConflictClassification> {
        action_examples
            .iter()
            .cartesian_product(reduce_examples)
            .filter(|&(action, _)| action.reductions.len() == 2)
            .filter(|&(_, reduce)| reduce.reductions.len() == 2)
            .filter(|&(_, reduce)|
                    reduce.reductions[0].nonterminal !=
                    reduce.reductions[1].nonterminal)
            .filter(|&(action, reduce)| {
                let action_suffix = self.inner_suffix(action);
                let reduce_suffix = self.inner_suffix(reduce);
                action_suffix == reduce_suffix
            })
            .map(|(action, reduce)| {
                let nt = reduce.reductions[0].nonterminal;
                let nt_productions = self.grammar.productions_for(nt);
                if nt_productions.len() == 2 {
                    for &(i, j) in &[(0, 1), (1, 0)] {
                        if
                            nt_productions[i].symbols.is_empty() &&
                            nt_productions[j].symbols.len() == 1
                        {
                            return ConflictClassification::SuggestQuestion {
                                shift: action.clone(),
                                reduce: reduce.clone(),
                                nonterminal: nt,
                                symbol: nt_productions[j].symbols[0],
                            }
                        }
                    }
                }
                ConflictClassification::SuggestInline {
                    shift: action.clone(),
                    reduce: reduce.clone(),
                    nonterminal: nt,
                }
            })
            .next()
    }

    fn inner_suffix<'ex>(&self, example: &'ex Example) -> &'ex [ExampleSymbol] {
        let end = example.reductions[0].end;
        &example.symbols[end..]
    }

    fn shift_examples(&self,
                      lookahead: Lookahead,
                      conflict: &'cx Conflict<'cx>)
                      -> Vec<Example> {
        log!(self.session, Verbose, "Gathering shift examples");
        let state = &self.states[conflict.state.0];
        let conflicting_items = self.conflicting_shift_items(state, lookahead, conflict);
        let mut tracer = self.tracer();
        conflicting_items
            .into_iter()
            .flat_map(|(item, lookaheads)| {
                let shift_trace =
                    tracer.backtrace_shift(conflict.state, item, &lookaheads);
                let local_examples: Vec<Example> =
                    shift_trace.examples().collect();
                local_examples
            })
            .collect()
    }

    fn reduce_examples(&self,
                       state: StateIndex,
                       production: &'cx Production,
                       lookahead: Lookahead)
                       -> Vec<Example> {
        log!(self.session, Verbose, "Gathering reduce examples");
        let mut tracer = self.tracer();
        let reduce_trace = tracer.backtrace_reduce(state, Item {
            production: production,
            index: production.symbols.len(),
            lookahead: lookahead
        });
        reduce_trace.examples().collect()
    }

    fn conflicting_shift_items(&self,
                               state: &'cx State,
                               lookahead: Lookahead,
                               _conflict: &Conflict)
                               -> Map<LR0Item<'cx>, LookaheadSet> {
        // Lookahead must be a terminal, not EOF.
        // Find an item J like `Bar = ... (*) L ...`.
        let lookahead = match lookahead {
            Lookahead::EOF => panic!("can't shift EOF"),
            Lookahead::Terminal(s) => Symbol::Terminal(s),
        };
        let mut result = map();
        for item in
            state.items.vec.iter()
                           .filter(|i| i.can_shift())
                           .filter(|i| i.production.symbols[i.index] == lookahead)
        {
            result.entry(item.to_lr0())
                  .or_insert_with(|| LookaheadSet::new(self.grammar))
                  .insert(self.grammar, item.lookahead);
        }
        result
    }
}

//fn choose_example<'grammar>(states: &[State<'grammar>],
//                            lookahead: Lookahead,
//                            conflict: &Conflict<'grammar>)
//{
//    // Whenever we have a conflict in state S, there is always:
//    // - a given lookahead L that permits some reduction, due to
//    //   an item I like `Foo = ... (*) [L]`
//    // - another action that conflicts with R1.
//    //
//    // The backtrace code can give context to this item `I`, but the
//    // problem is that it often results in many different contexts,
//    // and we need to try and narrow those down to the one that will
//    // help the user understand the problem.
//    //
//    // For that, we turn to the conflicting action, which can either be
//    // a shift or reduce. Let's consider those two cases.
//    //
//    // ### Shift
//    //
//    // If the conflicting action is a shift, then there is at least
//    // one item J in the state S like `Bar = ... (*) L ...`. We can
//    // produce a backtrace from J and enumerate examples. We want to
//    // find a pair of examples from I and J that share a common
//    // prefix.
//    //
//    // ### Reduce
//    //
//    // If the conflicting action is a reduce, then there is at least
//    // one item J in S like `Bar = ... (*) [L]`. We can produce a
//    // backtrace for J and then search for an example that shares a
//    // common prefix.
//
//}
//
//fn conflicting_item<'grammar>(state: &State<'grammar>,
//                              lookahead: Lookahead,
//                              conflict: &Conflict<'grammar>)
//                              -> Item<'grammar>
//{
//    match conflict.action {
//        Action::Shift(_) => {
//        }
//        Action::Reduce(production) => {
//            // Must be at least some other item J in S like `Bar = ... (*) [L]`.
//            state.items.vec.iter()
//                           .filter(|i| i.can_reduce())
//                           .filter(|i| i.lookahead == lookahead)
//                           .filter(|i| i.production == production)
//                           .cloned()
//                           .next()
//                           .unwrap()
//        }
//    }
//}

