//! Error reporting. For now very stupid and simplistic.

use grammar::repr::Grammar;
use std::io::{self, Write};

use super::{Action, TableConstructionError};

pub fn report_error<'grammar>(out: &mut Write,
                              grammar: &'grammar Grammar,
                              error: &TableConstructionError<'grammar>)
                              -> io::Result<()>
{
    try!(writeln!(out, "when in this state:"));
    for item in error.items.iter() {
        try!(writeln!(out, "  {:?}", item));
    }
    try!(writeln!(out, "and looking at a token `{:?}`,", error.lookahead));
    try!(writeln!(out, "we can reduce to a `{}`", error.production.nonterminal));
    match error.conflict {
        Action::Shift(_) =>
            try!(writeln!(out, "but we can also shift")),
        Action::Reduce(prod) =>
            try!(writeln!(out, "but we can also reduce to a `{}`", prod.nonterminal)),
    }
    Ok(())
}
