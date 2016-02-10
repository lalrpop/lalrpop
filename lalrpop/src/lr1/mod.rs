//! Naive LR(1) generation algorithm.

use session::Session;
use grammar::repr::*;

pub mod ascent;

mod backtrace;
mod build;
mod build_lalr;
mod core;
mod error;
mod example;
mod first;
mod lookahead;
mod state_graph;

#[cfg(test)] mod interpret;

use self::core::{State};

pub use self::core::TableConstructionError;
pub use self::error::report_error;

pub fn build_states<'grammar>(session: &Session,
                              grammar: &'grammar Grammar,
                              start: NonterminalString)
                              -> Result<Vec<State<'grammar>>,
                                        TableConstructionError<'grammar>>
{
    match grammar.algorithm {
        Algorithm::LR1 => build::build_lr1_states(session, grammar, start),
        Algorithm::LALR1 => build_lalr::build_lalr_states(session, grammar, start),
    }
}

