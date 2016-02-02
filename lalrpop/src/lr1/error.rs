//! Error reporting. For now very stupid and simplistic.

use grammar::repr::Grammar;
use std::io::{self, Write};
use super::{Action, Conflict, Lookahead, State, TableConstructionError};

pub fn report_error<'grammar>(out: &mut Write,
                              _grammar: &'grammar Grammar,
                              error: &TableConstructionError<'grammar>)
                              -> io::Result<()>
{
    for state in &error.states {
        for (&lookahead, conflicts) in &state.conflicts {
            for conflict in conflicts {
                try!(report_error_naive(out, &error.states, lookahead, conflict));
            }
        }
    }
    Ok(())
}

/// Naive error reporting. This is still used for LALR(1) reduction
/// errors but ought to be phased out completely, I imagine.
fn report_error_naive<'grammar>(out: &mut Write,
                                states: &[State<'grammar>],
                                lookahead: Lookahead,
                                conflict: &Conflict<'grammar>)
                                -> io::Result<()>
{
    try!(writeln!(out, "when in this state:"));
    for item in states[conflict.state.0].items.vec.iter() {
        try!(writeln!(out, "  {:?}", item));
    }
    try!(writeln!(out, "and looking at a token `{:?}`,", lookahead));
    try!(writeln!(out, "we can reduce to a `{}`", conflict.production.nonterminal));
    match conflict.action {
        Action::Shift(_) =>
            try!(writeln!(out, "but we can also shift")),
        Action::Reduce(prod) =>
            try!(writeln!(out, "but we can also reduce to a `{}`", prod.nonterminal)),
    }
    Ok(())
}
