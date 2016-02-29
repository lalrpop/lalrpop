//! Error reporting. For now very stupid and simplistic.

use collections::{Map, map, set};
use lr1::trace::Tracer;
use lr1::core::*;
use lr1::example::{Example, ExampleStyles, ExampleSymbol};
use lr1::first::FirstSets;
use lr1::lookahead::{Token, TokenSet};
use itertools::Itertools;
use grammar::repr::*;
use message::{Message};
use message::builder::{Builder, BodyCharacter, Character, MessageBuilder};
use tls::Tls;

#[cfg(test)] mod test;

pub fn report_error(grammar: &Grammar,
                    error: &TableConstructionError)
                    -> Vec<Message>
{
    let mut cx = ErrorReportingCx::new(grammar, &error.states);
    cx.report_errors()
}

struct ErrorReportingCx<'cx, 'grammar: 'cx> {
    grammar: &'grammar Grammar,
    first_sets: FirstSets,
    states: &'cx [LR1State<'grammar>],
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

impl<'cx, 'grammar> ErrorReportingCx<'cx, 'grammar> {
    fn new(grammar: &'grammar Grammar,
           states: &'cx [LR1State<'grammar>])
           -> Self {
        ErrorReportingCx {
            grammar: grammar,
            first_sets: FirstSets::new(grammar),
            states: states,
        }
    }

    fn report_errors(&mut self) -> Vec<Message> {
        self.states
            .iter()
            .flat_map(|state| &state.conflicts)
            .map(|conflict| self.report_error(conflict))
            .collect()
    }

    fn report_error(&mut self, conflict: &LR1Conflict<'grammar>) -> Message {
        match self.classify(conflict) {
            ConflictClassification::Ambiguity { action, reduce } => {
                self.report_error_ambiguity(conflict, action, reduce)
            }
            ConflictClassification::Precedence { shift, reduce, nonterminal } => {
                self.report_error_precedence(conflict, shift, reduce, nonterminal)
            }
            ConflictClassification::SuggestInline { shift, reduce, nonterminal } => {
                self.report_error_suggest_inline(conflict,
                                                 shift, reduce,
                                                 nonterminal)
            }
            ConflictClassification::SuggestQuestion { shift, reduce,
                                                      nonterminal, symbol } => {
                self.report_error_suggest_question(conflict,
                                                   shift, reduce,
                                                   nonterminal, symbol)
            }
            ConflictClassification::InsufficientLookahead { action, reduce } => {
                self.report_error_insufficient_lookahead(conflict, action, reduce)
            }
            ConflictClassification::Naive => {
                self.report_error_naive(conflict)
            }
        }
    }

    fn report_error_ambiguity_core(&self,
                                   conflict: &LR1Conflict<'grammar>,
                                   shift: Example,
                                   reduce: Example)
                                   -> Builder<BodyCharacter> {
        let styles = ExampleStyles::ambig();
        MessageBuilder::new(conflict.production.span)
            .heading()
            .text("Ambiguous grammar detected")
            .end()
            .body()

            .begin_lines()
            .wrap_text("The following symbols can be reduced in two ways:")
            .push(reduce.to_symbol_list(reduce.symbols.len(), styles))
            .end()

            .begin_lines()
            .wrap_text("They could be reduced like so:")
            .push(reduce.into_picture(styles))
            .end()

            .begin_lines()
            .wrap_text("Alternatively, they could be reduced like so:")
            .push(shift.into_picture(styles))
            .end()
    }

    fn report_error_ambiguity(&self,
                              conflict: &LR1Conflict<'grammar>,
                              shift: Example,
                              reduce: Example)
                              -> Message {
        self.report_error_ambiguity_core(conflict, shift, reduce)
            .wrap_text("LALRPOP does not yet support ambiguous grammars. \
                        See the LALRPOP manual for advice on \
                        making your grammar unambiguous.")
            .end()
            .end()
    }

    fn report_error_precedence(&self,
                               conflict: &LR1Conflict<'grammar>,
                               shift: Example,
                               reduce: Example,
                               nonterminal: NonterminalString)
                               -> Message {
        self.report_error_ambiguity_core(conflict, shift, reduce)
            .begin_wrap()
            .text("Hint:")
            .styled(Tls::session().hint_text)
            .text("This looks like a precedence error related to")
            .push(nonterminal)
            .verbatimed()
            .punctuated(".")
            .text("See the LALRPOP manual for advice on encoding precedence.")
            .end()
            .end()
            .end()
    }

    fn report_error_not_lr1_core(&self,
                                 conflict: &LR1Conflict<'grammar>,
                                 action: Example,
                                 reduce: Example)
                                 -> Builder<BodyCharacter> {
        let styles = ExampleStyles::new();
        let builder =
            MessageBuilder::new(conflict.production.span)
            .heading()
            .text("Local ambiguity detected")
            .end()

            .body();

        let builder = builder
            .begin_lines()
            .begin_wrap()
            .text("The problem arises after having observed the following symbols")
            .text("in the input:")
            .end()
            .push(if action.cursor >= reduce.cursor {
                action.to_symbol_list(action.cursor, styles)
            } else {
                reduce.to_symbol_list(reduce.cursor, styles)
            })
            .begin_wrap();

        let builder = match conflict.lookahead {
            Token::Terminal(term) => {
                builder
                    .text("At that point, if the next token is a")
                    .push(term)
                    .verbatimed()
                    .styled(Tls::session().cursor_symbol)
                    .punctuated(",")
            }
            Token::EOF =>
                builder.text("If the end of the input is reached,")
        };

        let builder = builder
            .text("then the parser can proceed in two different ways.")
            .end()
            .end();

        let builder =
            self.describe_reduce(builder, styles, conflict.production,
                                 reduce, "First");

        match conflict.action {
            Action::Shift(_) =>
                self.describe_shift(builder, styles, conflict.lookahead,
                                    action, "Alternatively"),
            Action::Reduce(production) =>
                self.describe_reduce(builder, styles, production,
                                     action, "Alternatively"),
        }
    }

    fn describe_shift<C: Character>(&self,
                                    builder: Builder<C>,
                                    styles: ExampleStyles,
                                    lookahead: Token,
                                    example: Example,
                                    intro_word: &str)
                                    -> Builder<C>
    {
        // Shift actions can only happen with non-EOF lookaheads:
        let lookahead = lookahead.unwrap_terminal();

        // A shift example looks like:
        //
        // ...p1 ...p2 (*) L ...s2 ...s1
        // |     |               |     |
        // |     +-NT1-----------+     |
        // |                           |
        // |           ...             |
        // |                           |
        // +-NT2-----------------------+

        let nt1 = example.reductions[0].nonterminal;

        builder
            .begin_lines()
            .begin_wrap()
            .text(intro_word)
            .punctuated(",")
            .text("the parser could shift the")
            .push(lookahead)
            .verbatimed()
            .text("token and later use it to construct a")
            .push(nt1)
            .verbatimed()
            .punctuated(".")
            .text("This might then yield a parse tree like")
            .end()
            .push(example.into_picture(styles))
            .end()
    }

    fn describe_reduce<C: Character>(&self,
                                     builder: Builder<C>,
                                     styles: ExampleStyles,
                                     production: &Production,
                                     example: Example,
                                     intro_word: &str)
                                     -> Builder<C>
    {
        builder
            .begin_lines()
            .begin_wrap()
            .text(intro_word)
            .punctuated(",")
            .text("the parser could execute the production at")
            .push(production.span)
            .punctuated(",")
            .text("which would consume the top")
            .text(production.symbols.len())
            .text("token(s) from the stack")
            .text("and produce a")
            .push(production.nonterminal)
            .verbatimed()
            .punctuated(".")
            .text("This might then yield a parse tree like")
            .end()
            .push(example.into_picture(styles))
            .end()
    }

    fn report_error_suggest_inline(&self,
                                   conflict: &LR1Conflict<'grammar>,
                                   shift: Example,
                                   reduce: Example,
                                   nonterminal: NonterminalString)
                                   -> Message
    {
        let builder = self.report_error_not_lr1_core(conflict, shift, reduce);

        builder
            .begin_wrap()
            .text("Hint:")
            .styled(Tls::session().hint_text)
            .text("It appears you could resolve this problem by adding")
            .text("the annotation `#[inline]` to the definition of")
            .push(nonterminal)
            .verbatimed()
            .punctuated(".")
            .text("For more information, see the section on inlining")
            .text("in the LALRPOP manual.")
            .end()
            .end()
            .end()
    }

    fn report_error_suggest_question(&self,
                                     conflict: &LR1Conflict<'grammar>,
                                     shift: Example,
                                     reduce: Example,
                                     nonterminal: NonterminalString,
                                     symbol: Symbol)
                                     -> Message
    {
        let builder = self.report_error_not_lr1_core(conflict, shift, reduce);

        builder
            .begin_wrap()
            .text("Hint:")
            .styled(Tls::session().hint_text)
            .text("It appears you could resolve this problem by replacing")
            .text("uses of")
            .push(nonterminal)
            .verbatimed()
            .text("with")
            .text(symbol) // intentionally disable coloring here, looks better
            .adjacent_text("`", "?`")
            .text("(or, alternatively, by adding the annotation `#[inline]` \
                   to the definition of")
            .push(nonterminal)
            .punctuated(").")
            .text("For more information, see the section on inlining")
            .text("in the LALROP manual.")
            .end()
            .end()
            .end()
    }

    fn report_error_insufficient_lookahead(&self,
                                           conflict: &LR1Conflict<'grammar>,
                                           action: Example,
                                           reduce: Example)
                                           -> Message {
        // The reduce example will look something like:
        //
        //
        // ...p1 ...p2 (*) L ...s2 ...s1
        // |     |               |     |
        // |     +-NT1-----------+     |
        // |     |               |     |
        // |     +-...-----------+     |
        // |     |               |     |
        // |     +-NTn-----------+     |
        // |                           |
        // +-NTn+1---------------------+
        //
        // To solve the conflict, essentially, the user needs to
        // modify the grammar so that `NTn` does not appear with `L`
        // in its follow-set. How to guide them in this?

        let builder = self.report_error_not_lr1_core(conflict, action, reduce);

        builder
            .wrap_text("See the LALRPOP manual for advice on \
                        making your grammar LR(1).")
            .end()
            .end()
    }

    /// Naive error reporting. This is a fallback path which (I think)
    /// never actually executes.
    fn report_error_naive(&self,
                          conflict: &LR1Conflict<'grammar>)
                          -> Message {
        let mut builder =
            MessageBuilder::new(conflict.production.span)
            .heading()
            .text("Conflict detected")
            .end()
            .body()
            .begin_lines()
            .wrap_text("when in this state:")
            .indented();
        for item in self.states[conflict.state.0].items.vec.iter() {
            builder = builder.text(format!("{:?}", item));
        }
        let mut builder =
            builder.end()
                   .begin_wrap()
                   .text(format!("and looking at a token `{:?}`", conflict.lookahead))
                   .text("we can reduce to a")
                   .push(conflict.production.nonterminal)
                   .verbatimed();
        builder = match conflict.action {
            Action::Shift(_) =>
                builder.text("but we can also shift"),
            Action::Reduce(prod) =>
                builder.text("but we can also reduce to a")
                       .text(prod.nonterminal)
                       .verbatimed()
        };
        builder.end()
               .end()
               .end()
    }

    fn classify(&mut self, conflict: &LR1Conflict<'grammar>) -> ConflictClassification {
        // Find examples from the conflicting action (either a shift
        // or a reduce).
        let mut action_examples = match conflict.action {
            Action::Shift(_) => self.shift_examples(conflict),
            Action::Reduce(production) => self.reduce_examples(conflict.state,
                                                               production,
                                                               conflict.lookahead)
        };

        // Find examples from the conflicting reduce.
        let mut reduce_examples = self.reduce_examples(conflict.state,
                                                       conflict.production,
                                                       conflict.lookahead);

        // Prefer shorter examples to longer ones.
        action_examples.sort_by(|e, f| e.symbols.len().cmp(&f.symbols.len()));
        reduce_examples.sort_by(|e, f| e.symbols.len().cmp(&f.symbols.len()));

        if let Some(classification) = self.try_classify_ambiguity(conflict,
                                                                  &action_examples,
                                                                  &reduce_examples) {
            return classification;
        }

        if let Some(classification) = self.try_classify_question(conflict,
                                                                 &action_examples,
                                                                 &reduce_examples) {
            return classification;
        }

        if let Some(classification) = self.try_classify_inline(conflict,
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
                              conflict: &LR1Conflict<'grammar>,
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
                    if let Token::Terminal(term) = conflict.lookahead {
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

    fn try_classify_question(&self,
                             conflict: &LR1Conflict<'grammar>,
                             action_examples: &[Example],
                             reduce_examples: &[Example])
                             -> Option<ConflictClassification> {
        // If we get a shift/reduce conflict and the reduce
        // is of a nonterminal like:
        //
        //     T = { () | U }
        //
        // then suggest replacing T with U?. I'm being a bit lenient
        // here since I do not KNOW that it will help, but it often
        // does, and it's better style anyhow.

        match conflict.action {
            Action::Shift(_) => { }
            Action::Reduce(_) => {
                return None;
            }
        }

        let nt = conflict.production.nonterminal;
        let nt_productions = self.grammar.productions_for(nt);
        if nt_productions.len() == 2 {
            for &(i, j) in &[(0, 1), (1, 0)] {
                if
                    nt_productions[i].symbols.is_empty() &&
                    nt_productions[j].symbols.len() == 1
                {
                    return Some(ConflictClassification::SuggestQuestion {
                        shift: action_examples[0].clone(),
                        reduce: reduce_examples[0].clone(),
                        nonterminal: nt,
                        symbol: nt_productions[j].symbols[0],
                    });
                }
            }
        }

        None
    }

    fn try_classify_inline(&self,
                           conflict: &LR1Conflict<'grammar>,
                           action_examples: &[Example],
                           reduce_examples: &[Example])
                           -> Option<ConflictClassification> {
        // Inlining can help resolve a shift/reduce conflict because
        // it defers the need to reduce. In particular, if we inlined
        // all the reductions up until the last one, then we would be
        // able to *shift* the lookahead instead of having to reduce.
        // This can be helpful if we can see that shifting would let
        // us delay reducing until the lookahead diverges.

        // Only applicable to shift/reduce:
        match conflict.action {
            Action::Shift(_) => { }
            Action::Reduce(_) => {
                return None;
            }
        }

        // FIXME: The logic here finds the first example where inline
        // would help; but maybe we want to restrict it to cases
        // where inlining would help *all* the examples...?

        action_examples
            .iter()
            .cartesian_product(reduce_examples)
            .filter_map(|(shift, reduce)| {
                if self.try_classify_inline_example(shift, reduce) {
                    let nt = reduce.reductions[0].nonterminal;
                    Some(ConflictClassification::SuggestInline {
                        shift: shift.clone(),
                        reduce: reduce.clone(),
                        nonterminal: nt,
                    })
                } else {
                    None
                }
            })
            .next()
    }

    fn try_classify_inline_example<'ex>(&self,
                                        shift: &Example,
                                        reduce: &Example)
                                        -> bool {
        debug!("try_classify_inline_example({:?}, {:?})",
               shift, reduce);

        // In the case of shift, the example will look like
        //
        // ```
        // ... ... (*) L ...s1 ...
        // |   |             |   |
        // |   +-R0----------+   |
        // |  ...                |
        // +-Rn------------------+
        // ```
        //
        // We want to extract the symbols ...s1: these are the
        // things we are able to shift before being forced to
        // make our next hard decision (to reduce R0 or not).
        let shift_upcoming =
            &shift.symbols[shift.cursor+1 .. shift.reductions[0].end];
        debug!("try_classify_inline_example: shift_upcoming={:?}",
               shift_upcoming);

        // For the reduce, the example might look like
        //
        // ```
        // ...  ...   (*) ...s ...
        // | | |    |        |
        // | | +-R0-+        |
        // | | ...  |        |
        // | +--Ri--+        |
        // |  ...            |
        // +-R(i+1)----------+
        // ```
        //
        // where Ri is the last reduction that requires
        // shifting no additional symbols. In this case, if we
        // inlined R0...Ri, then we know we can shift L.
        let r0_end = reduce.reductions[0].end;
        let i = reduce.reductions.iter().position(|r| r.end != r0_end);
        let i = match i { Some(v) => v, None => return false };
        let ri = &reduce.reductions[i];
        let reduce_upcoming = &reduce.symbols[r0_end..ri.end];
        debug!("try_classify_inline_example: reduce_upcoming={:?} i={:?}",
               reduce_upcoming, i);

        // For now, we only suggest inlining a single nonterminal,
        // mostly because I am too lazy to weak the suggestion struct
        // and error messages (but the rest of the code below doesn't
        // make this assumption for the most part).
        if i != 1 {
            return false;
        }

        // Make sure that all the things we are suggesting inlining
        // are distinct so that we are not introducing a cycle.
        let mut duplicates = set();
        if reduce.reductions[0..i+1].iter()
                                    .any(|r| !duplicates.insert(r.nonterminal)) {
            return false;
        }

        // Compare the two suffixes to see whether they
        // diverge at some point.
        shift_upcoming
            .iter()
            .zip(reduce_upcoming)
            .filter_map(|(shift_sym, reduce_sym)| match (shift_sym, reduce_sym) {
                (&ExampleSymbol::Symbol(shift_sym),
                 &ExampleSymbol::Symbol(reduce_sym)) => {
                    if shift_sym == reduce_sym {
                        // same symbol on both; we'll be able to shift them
                        None
                    } else {
                        // different symbols: for this to
                        // work, must have disjoint first
                        // sets. We'll be approximate and
                        // supply the same Token::EOF to
                        // both, though in fact the actual
                        // lookahead may be helpful here.
                        let (shift_first, _) =
                            self.first_sets.first(self.grammar, &[shift_sym],
                                                  Token::EOF);
                        let (reduce_first, _) =
                            self.first_sets.first(self.grammar, &[reduce_sym],
                                                  Token::EOF);
                        if shift_first.is_disjoint(reduce_first) {
                            Some(true)
                        } else {
                            Some(false)
                        }
                    }
                }
                _ => {
                    // we don't expect to encounter any
                    // epsilons, I don't think, because those
                    // only occur with an empty reduce at the
                    // top level
                    Some(false)
                }
            })
            .next()
            .unwrap_or(false)
    }

    fn shift_examples(&self,
                      conflict: &LR1Conflict<'grammar>)
                      -> Vec<Example> {
        log!(Tls::session(), Verbose, "Gathering shift examples");
        let state = &self.states[conflict.state.0];
        let conflicting_items = self.conflicting_shift_items(state, conflict);
        conflicting_items
            .into_iter()
            .flat_map(|(item, _lookaheads)| {
                let tracer = Tracer::new(self.grammar, self.states);
                let shift_trace =
                    tracer.backtrace_shift(conflict.state, item);
                let local_examples: Vec<Example> =
                    shift_trace.examples(item).collect();
                local_examples
            })
            .collect()
    }

    fn reduce_examples(&self,
                       state: StateIndex,
                       production: &'grammar Production,
                       lookahead: Token)
                       -> Vec<Example> {
        log!(Tls::session(), Verbose, "Gathering reduce examples");
        let item = Item {
            production: production,
            index: production.symbols.len(),
            lookahead: lookahead
        };
        let tracer = Tracer::new(self.grammar, self.states);
        let reduce_trace = tracer.backtrace_reduce(state, item);
        reduce_trace.examples(item.to_lr0()).collect()
    }

    fn conflicting_shift_items(&self,
                               state: &LR1State<'grammar>,
                               conflict: &LR1Conflict<'grammar>)
                               -> Map<LR0Item<'grammar>, TokenSet> {
        // Lookahead must be a terminal, not EOF.
        // Find an item J like `Bar = ... (*) L ...`.
        let lookahead = Symbol::Terminal(conflict.lookahead.unwrap_terminal());
        let mut result = map();
        for item in
            state.items.vec.iter()
                           .filter(|i| i.can_shift())
                           .filter(|i| i.production.symbols[i.index] == lookahead)
        {
            result.entry(item.to_lr0())
                  .or_insert_with(|| TokenSet::new(self.grammar))
                  .insert(self.grammar, item.lookahead);
        }
        result
    }
}

//fn choose_example<'grammar>(states: &[State<'grammar>],
//                            lookahead: Token,
//                            conflict: &LR1Conflict<'grammar>)
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
//                              lookahead: Token,
//                              conflict: &LR1Conflict<'grammar>)
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
